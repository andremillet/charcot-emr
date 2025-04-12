// src/lib.rs
// Charcot EMR: Library module exposing core EMR functionality

use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce
};
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};
use uuid::Uuid;
use anyhow::{Result, anyhow, Context};

// FHIR-aligned data structures
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Patient {
    pub id: String,
    pub identifier: Vec<Identifier>,
    pub name: Vec<HumanName>,
    pub gender: String,
    pub birth_date: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Identifier {
    pub system: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HumanName {
    pub given: Vec<String>,
    pub family: Option<String>,
    pub prefix: Option<Vec<String>>,
    pub suffix: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Observation {
    pub id: String,
    pub status: String,
    pub code: Coding,
    pub subject: Reference,
    pub effective_date_time: String,
    pub value_quantity: Option<Quantity>,
    pub component: Option<Vec<Component>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Coding {
    pub system: String,
    pub code: String,
    pub display: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reference {
    pub reference: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quantity {
    pub value: f64,
    pub unit: String,
    pub system: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Component {
    pub code: Coding,
    pub value_quantity: Quantity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MedicationRequest {
    pub id: String,
    pub status: String,
    pub medication_codeable_concept: Coding,
    pub subject: Reference,
    pub authored_on: String,
    pub dosage_instruction: Vec<DosageInstruction>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DosageInstruction {
    pub text: String,
    pub timing: Timing,
    pub dose_and_rate: Vec<DoseAndRate>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Timing {
    pub repeat: Option<Repeat>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repeat {
    pub frequency: Option<i32>,
    pub period: Option<f64>,
    pub period_unit: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DoseAndRate {
    pub dose_quantity: Option<Quantity>,
}

// FHIR Bundle to hold all resources
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bundle {
    pub resource_type: String,
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub entry: Vec<BundleEntry>,
    #[serde(skip)]
    pub version_history: Vec<VersionEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BundleEntry {
    pub resource_type: String,
    pub resource: Resource,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "resourceType")]
pub enum Resource {
    Patient(Patient),
    Observation(Observation),
    MedicationRequest(MedicationRequest),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionEntry {
    pub timestamp: DateTime<Utc>,
    pub message: String,
    pub hash: String,
}

// Encrypted .med file format
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MedFile {
    pub iv: String,             // Base64 encoded initialization vector
    pub data: String,           // Base64 encoded encrypted data
    pub hash: String,           // SHA-256 hash of the unencrypted data
    pub created: DateTime<Utc>, // Creation timestamp
    pub modified: DateTime<Utc>, // Last modified timestamp
}

// Special data types with validation
pub struct BloodPressure {
    pub systolic: i32,
    pub diastolic: i32,
}

impl BloodPressure {
    pub fn new(systolic: i32, diastolic: i32) -> Result<Self> {
        // Basic validation
        if systolic < 40 || systolic > 300 {
            return Err(anyhow!("Invalid systolic value: {}. Expected range 40-300", systolic));
        }
        if diastolic < 20 || diastolic > 200 {
            return Err(anyhow!("Invalid diastolic value: {}. Expected range 20-200", diastolic));
        }

        Ok(BloodPressure { systolic, diastolic })
    }

    pub fn to_observation(&self, patient_id: &str) -> Observation {
        let now = Utc::now().to_rfc3339();
        
        Observation {
            id: Uuid::new_v4().to_string(),
            status: "final".to_string(),
            code: Coding {
                system: "http://loinc.org".to_string(),
                code: "85354-9".to_string(),
                display: "Blood pressure panel".to_string(),
            },
            subject: Reference {
                reference: format!("Patient/{}", patient_id),
            },
            effective_date_time: now,
            value_quantity: None,
            component: Some(vec![
                Component {
                    code: Coding {
                        system: "http://loinc.org".to_string(),
                        code: "8480-6".to_string(),
                        display: "Systolic blood pressure".to_string(),
                    },
                    value_quantity: Quantity {
                        value: self.systolic as f64,
                        unit: "mmHg".to_string(),
                        system: "http://unitsofmeasure.org".to_string(),
                        code: "mm[Hg]".to_string(),
                    },
                },
                Component {
                    code: Coding {
                        system: "http://loinc.org".to_string(),
                        code: "8462-4".to_string(),
                        display: "Diastolic blood pressure".to_string(),
                    },
                    value_quantity: Quantity {
                        value: self.diastolic as f64,
                        unit: "mmHg".to_string(),
                        system: "http://unitsofmeasure.org".to_string(),
                        code: "mm[Hg]".to_string(),
                    },
                },
            ]),
        }
    }
}

// Main EMR functionality
pub struct EMR {
    pub bundles: HashMap<String, Bundle>,
    pub audit_log: File,
}

impl EMR {
    pub fn new() -> Result<Self> {
        // Create/open audit log file
        let audit_log = OpenOptions::new()
            .append(true)
            .create(true)
            .open("audit.log")
            .context("Failed to open audit log")?;

        Ok(EMR {
            bundles: HashMap::new(),
            audit_log,
        })
    }

    // Log an audit event
    pub fn log_audit(&mut self, event: &str, patient_id: &str) -> Result<()> {
        let timestamp = Utc::now().to_rfc3339();
        let log_entry = format!("{} - Patient#{}: {}\n", timestamp, patient_id, event);
        
        self.audit_log.write_all(log_entry.as_bytes())
            .context("Failed to write to audit log")?;
        
        Ok(())
    }
}

    // Create a new patient
    pub fn create_patient(&mut self, id: &str, given_name: &str, family_name: &str, 
                        gender: &str, birth_date: &str) -> Result<()> {
        let patient = Patient {
            id: id.to_string(),
            identifier: vec![Identifier {
                system: "https://charcot.emr/patients".to_string(),
                value: id.to_string(),
            }],
            name: vec![HumanName {
                given: vec![given_name.to_string()],
                family: Some(family_name.to_string()),
                prefix: None,
                suffix: None,
            }],
            gender: gender.to_string(),
            birth_date: birth_date.to_string(),
        };

        let bundle = Bundle {
            resource_type: "Bundle".to_string(),
            id: Uuid::new_v4().to_string(),
            type_field: "collection".to_string(),
            entry: vec![
                BundleEntry {
                    resource_type: "Patient".to_string(),
                    resource: Resource::Patient(patient),
                }
            ],
            version_history: vec![
                VersionEntry {
                    timestamp: Utc::now(),
                    message: "Patient created".to_string(),
                    hash: "".to_string(), // Will be filled in by save_patient
                }
            ],
        };

        self.bundles.insert(id.to_string(), bundle);
        self.log_audit("Patient created", id)?;
        
        Ok(())
    }

    // Add blood pressure reading
    pub fn add_blood_pressure(&mut self, patient_id: &str, 
                             systolic: i32, diastolic: i32) -> Result<()> {
        // Validate blood pressure values
        let bp = BloodPressure::new(systolic, diastolic)?;
        let observation = bp.to_observation(patient_id);

        // Add observation to patient bundle
        let bundle = self.bundles.get_mut(patient_id)
            .ok_or_else(|| anyhow!("Patient not found: {}", patient_id))?;

        bundle.entry.push(BundleEntry {
            resource_type: "Observation".to_string(),
            resource: Resource::Observation(observation),
        });

        self.log_audit(&format!("Added BP: {}/{}", systolic, diastolic), patient_id)?;
        
        Ok(())
    }

    // Prescribe medication
    pub fn prescribe_medication(&mut self, patient_id: &str, medication: &str, 
                               dose_mg: f64, frequency: &str) -> Result<()> {
        // Basic validation
        if dose_mg <= 0.0 {
            return Err(anyhow!("Invalid dose: {} mg", dose_mg));
        }

        // Create medication request
        let med_request = MedicationRequest {
            id: Uuid::new_v4().to_string(),
            status: "active".to_string(),
            medication_codeable_concept: Coding {
                system: "http://www.nlm.nih.gov/research/umls/rxnorm".to_string(),
                code: "1234".to_string(), // Placeholder - would use actual RxNorm code
                display: medication.to_string(),
            },
            subject: Reference {
                reference: format!("Patient/{}", patient_id),
            },
            authored_on: Utc::now().to_rfc3339(),
            dosage_instruction: vec![
                DosageInstruction {
                    text: format!("{} mg {}", dose_mg, frequency),
                    timing: Timing {
                        repeat: Some(Repeat {
                            frequency: Some(1),
                            period: Some(1.0),
                            period_unit: Some("d".to_string()), // daily
                        }),
                    },
                    dose_and_rate: vec![
                        DoseAndRate {
                            dose_quantity: Some(Quantity {
                                value: dose_mg,
                                unit: "mg".to_string(),
                                system: "http://unitsofmeasure.org".to_string(),
                                code: "mg".to_string(),
                            }),
                        }
                    ],
                }
            ],
        };

        // Add medication request to patient bundle
        let bundle = self.bundles.get_mut(patient_id)
            .ok_or_else(|| anyhow!("Patient not found: {}", patient_id))?;

        bundle.entry.push(BundleEntry {
            resource_type: "MedicationRequest".to_string(),
            resource: Resource::MedicationRequest(med_request),
        });

        self.log_audit(&format!("Prescribed: {} {}mg {}", 
                               medication, dose_mg, frequency), patient_id)?;
        
        Ok(())
    }

    // Commit changes to patient record with versioning
    pub fn commit_changes(&mut self, patient_id: &str, message: &str) -> Result<()> {
        let bundle = self.bundles.get_mut(patient_id)
            .ok_or_else(|| anyhow!("Patient not found: {}", patient_id))?;
        
        // Create a hash of the current state
        let bundle_json = serde_json::to_string(&bundle.entry)?;
        let mut hasher = Sha256::new();
        hasher.update(bundle_json.as_bytes());
        let hash = format!("{:x}", hasher.finalize());
        
        // Add to version history
        bundle.version_history.push(VersionEntry {
            timestamp: Utc::now(),
            message: message.to_string(),
            hash,
        });
        
        self.log_audit(&format!("Committed changes: {}", message), patient_id)?;
        
        Ok(())
    }

    // Save patient data to .med file
    pub fn save_patient(&self, patient_id: &str, key: &str) -> Result<()> {
        let bundle = self.bundles.get(patient_id)
            .ok_or_else(|| anyhow!("Patient not found: {}", patient_id))?;
        
        // Serialize the bundle to JSON
        let bundle_json = serde_json::to_string(bundle)?;
        
        // Calculate hash of unencrypted data
        let mut hasher = Sha256::new();
        hasher.update(bundle_json.as_bytes());
        let hash = format!("{:x}", hasher.finalize());
        
        // Generate a key from the password
        let mut key_hasher = Sha256::new();
        key_hasher.update(key.as_bytes());
        let key_bytes = key_hasher.finalize();
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        
        // Generate a random 96-bit nonce
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        
        // Encrypt the data
        let cipher = Aes256Gcm::new(key);
        let encrypted_data = cipher.encrypt(&nonce, bundle_json.as_bytes())
            .map_err(|e| anyhow!("Encryption failed: {:?}", e))?;
        
        // Create the MedFile structure
        let med_file = MedFile {
            iv: general_purpose::STANDARD.encode(nonce),
            data: general_purpose::STANDARD.encode(encrypted_data),
            hash,
            created: bundle.version_history[0].timestamp,
            modified: Utc::now(),
        };
        
        // Serialize and write to file
        let med_json = serde_json::to_string(&med_file)?;
        let filename = format!("patient_{}.med", patient_id);
        fs::write(&filename, med_json)?;
        
        Ok(())
    }

    // Load patient data from .med file
    pub fn load_patient(&mut self, filename: &str, key: &str) -> Result<String> {
        // Read the .med file
        let med_json = fs::read_to_string(filename)?;
        let med_file: MedFile = serde_json::from_str(&med_json)?;
        
        // Generate key from password
        let mut key_hasher = Sha256::new();
        key_hasher.update(key.as_bytes());
        let key_bytes = key_hasher.finalize();
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        
        // Decode IV and encrypted data
        let iv = general_purpose::STANDARD.decode(&med_file.iv)?;
        let encrypted_data = general_purpose::STANDARD.decode(&med_file.data)?;
        
        // Create nonce from IV
        let nonce = Nonce::from_slice(&iv);
        
        // Decrypt the data
        let cipher = Aes256Gcm::new(key);
        let decrypted_data = cipher.decrypt(nonce, encrypted_data.as_ref())
            .map_err(|e| anyhow!("Decryption failed: {:?}", e))?;
        
        // Verify hash
        let mut hasher = Sha256::new();
        hasher.update(&decrypted_data);
        let calculated_hash = format!("{:x}", hasher.finalize());
        
        if calculated_hash != med_file.hash {
            return Err(anyhow!("Hash verification failed - file may be corrupted"));
        }
        
        // Deserialize to bundle
        let bundle: Bundle = serde_json::from_slice(&decrypted_data)?;
        
        // Extract patient ID
        let patient_id = match &bundle.entry[0].resource {
            Resource::Patient(patient) => patient.id.clone(),
            _ => return Err(anyhow!("First resource is not a Patient")),
        };
        
        // Add to EMR
        self.bundles.insert(patient_id.clone(), bundle);
        self.log_audit(&format!("Loaded patient from {}", filename), &patient_id)?;
        
        Ok(patient_id)
    }

    // Mock device integration
    pub fn connect_device(&mut self, patient_id: &str, device_type: &str) -> Result<()> {
        // This is just a stub for now
        self.log_audit(&format!("Connected device: {}", device_type), patient_id)?;
        println!("Mock device {} connected for patient {}", device_type, patient_id);
        
        Ok(())
    }
}
