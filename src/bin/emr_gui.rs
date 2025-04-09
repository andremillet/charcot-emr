// src/bin/emr_gui.rs
// A simple GUI for the Charcot EMR using egui

use charcot_emr::{EMR, Resource, BundleEntry};
use eframe::egui;
use egui::{TextEdit, Ui, Vec2};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use anyhow::Result;

// Make sure to add egui dependency to Cargo.toml:
// eframe = "0.19"

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(1024.0, 768.0)),
        min_window_size: Some(Vec2::new(800.0, 600.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "Charcot EMR",
        native_options,
        Box::new(|_cc| Box::new(EMRApp::default()))
    )
}

struct EMRApp {
    emr: Arc<Mutex<EMR>>,
    current_patient_id: String,
    patient_key: String,
    status_message: String,
    
    // Form fields
    new_patient: PatientForm,
    vital_signs: VitalSignsForm,
    medication: MedicationForm,
    
    // View state
    current_view: View,
    load_path: String,
}

impl eframe::App for EMRApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.render_menu_bar(ui);
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.status_message);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_view {
                View::Home => self.render_home_view(ui),
                View::CreatePatient => self.render_create_patient_view(ui),
                View::AddVitals => self.render_add_vitals_view(ui),
                View::Prescribe => self.render_prescribe_view(ui),
                View::ViewPatient => self.render_view_patient(ui),
                View::LoadPatient => self.render_load_patient_view(ui),
            }
        });
    }
}

impl EMRApp {
    fn render_menu_bar(&mut self, ui: &mut Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("New Patient").clicked() {
                    self.current_view = View::CreatePatient;
                    ui.close_menu();
                }
                if ui.button("Load Patient").clicked() {
                    self.current_view = View::LoadPatient;
                    ui.close_menu();
                }
                if ui.button("Exit").clicked() {
                    std::process::exit(0);
                }
            });
            
            if !self.current_patient_id.is_empty() {
                ui.menu_button("Patient", |ui| {
                    if ui.button("View Record").clicked() {
                        self.current_view = View::ViewPatient;
                        ui.close_menu();
                    }
                    if ui.button("Add Vitals").clicked() {
                        self.current_view = View::AddVitals;
                        ui.close_menu();
                    }
                    if ui.button("Prescribe Medication").clicked() {
                        self.current_view = View::Prescribe;
                        ui.close_menu();
                    }
                });
            }
            
            ui.menu_button("Help", |ui| {
                if ui.button("About").clicked() {
                    self.status_message = "Charcot EMR v0.1 - A medical programming language prototype".to_string();
                    ui.close_menu();
                }
            });
        });
    }
    
    fn render_home_view(&mut self, ui: &mut Ui) {
        ui.heading("Charcot EMR System");
        ui.add_space(20.0);
        
        ui.label("Welcome to the Charcot Electronic Medical Record System");
        ui.label("This is a prototype implementation of the EMR described in the Charcot medical programming language specification.");
        
        ui.add_space(20.0);
        
        if ui.button("Create New Patient").clicked() {
            self.current_view = View::CreatePatient;
        }
        
        if ui.button("Load Patient Record").clicked() {
            self.current_view = View::LoadPatient;
        }
        
        ui.add_space(20.0);
        
        if !self.current_patient_id.is_empty() {
            ui.label(format!("Current Patient: {}", self.current_patient_id));
            
            if ui.button("View Patient Record").clicked() {
                self.current_view = View::ViewPatient;
            }
            
            if ui.button("Add Vital Signs").clicked() {
                self.current_view = View::AddVitals;
            }
            
            if ui.button("Prescribe Medication").clicked() {
                self.current_view = View::Prescribe;
            }
        }
    }
    
    fn render_create_patient_view(&mut self, ui: &mut Ui) {
        ui.heading("Create New Patient");
        ui.add_space(10.0);
        
        ui.horizontal(|ui| {
            ui.label("Patient ID: ");
            ui.text_edit_singleline(&mut self.new_patient.id);
        });
        
        ui.horizontal(|ui| {
            ui.label("Given Name: ");
            ui.text_edit_singleline(&mut self.new_patient.given_name);
        });
        
        ui.horizontal(|ui| {
            ui.label("Family Name: ");
            ui.text_edit_singleline(&mut self.new_patient.family_name);
        });
        
        ui.horizontal(|ui| {
            ui.label("Gender: ");
            egui::ComboBox::from_id_source("gender_combo")
                .selected_text(&self.new_patient.gender)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.new_patient.gender, "male".to_string(), "Male");
                    ui.selectable_value(&mut self.new_patient.gender, "female".to_string(), "Female");
                    ui.selectable_value(&mut self.new_patient.gender, "other".to_string(), "Other");
                });
        });
        
        ui.horizontal(|ui| {
            ui.label("Birth Date (YYYY-MM-DD): ");
            ui.text_edit_singleline(&mut self.new_patient.birth_date);
        });
        
        ui.horizontal(|ui| {
            ui.label("Encryption Key: ");
            ui.add(TextEdit::singleline(&mut self.new_patient.key).password(true));
        });
        
        ui.add_space(10.0);
        
        if ui.button("Create Patient").clicked() {
            if self.new_patient.id.is_empty() || self.new_patient.given_name.is_empty() || 
               self.new_patient.family_name.is_empty() || self.new_patient.birth_date.is_empty() ||
               self.new_patient.key.is_empty() {
                self.status_message = "Error: All fields are required".to_string();
            } else {
                match self.emr.lock() {
                    Ok(mut emr) => {
                        match emr.create_patient(
                            &self.new_patient.id,
                            &self.new_patient.given_name,
                            &self.new_patient.family_name,
                            &self.new_patient.gender,
                            &self.new_patient.birth_date
                        ) {
                            Ok(_) => {
                                match emr.commit_changes(&self.new_patient.id, "Initial patient creation") {
                                    Ok(_) => {
                                        match emr.save_patient(&self.new_patient.id, &self.new_patient.key) {
                                            Ok(_) => {
                                                self.current_patient_id = self.new_patient.id.clone();
                                                self.patient_key = self.new_patient.key.clone();
                                                self.status_message = format!("Patient {} created successfully", self.current_patient_id);
                                                self.current_view = View::ViewPatient;
                                                
                                                // Reset form
                                                self.new_patient = PatientForm::default();
                                            },
                                            Err(e) => {
                                                self.status_message = format!("Error saving patient: {}", e);
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        self.status_message = format!("Error committing changes: {}", e);
                                    }
                                }
                            },
                            Err(e) => {
                                self.status_message = format!("Error creating patient: {}", e);
                            }
                        }
                    },
                    Err(_) => {
                        self.status_message = "Error accessing EMR".to_string();
                    }
                }
            }
        }
        
        if ui.button("Cancel").clicked() {
            self.current_view = View::Home;
            self.new_patient = PatientForm::default();
        }
    }
    
    fn render_add_vitals_view(&mut self, ui: &mut Ui) {
        ui.heading("Add Vital Signs");
        ui.add_space(10.0);
        
        ui.label(format!("Patient ID: {}", self.current_patient_id));
        ui.add_space(10.0);
        
        ui.horizontal(|ui| {
            ui.label("Blood Pressure - Systolic: ");
            ui.text_edit_singleline(&mut self.vital_signs.systolic);
        });
        
        ui.horizontal(|ui| {
            ui.label("Blood Pressure - Diastolic: ");
            ui.text_edit_singleline(&mut self.vital_signs.diastolic);
        });
        
        ui.add_space(10.0);
        
        if ui.button("Add Vital Signs").clicked() {
            if self.vital_signs.systolic.is_empty() || self.vital_signs.diastolic.is_empty() {
                self.status_message = "Error: Both systolic and diastolic values are required".to_string();
            } else {
                match (self.vital_signs.systolic.parse::<i32>(), self.vital_signs.diastolic.parse::<i32>()) {
                    (Ok(systolic), Ok(diastolic)) => {
                        match self.emr.lock() {
                            Ok(mut emr) => {
                                match emr.add_blood_pressure(&self.current_patient_id, systolic, diastolic) {
                                    Ok(_) => {
                                        match emr.commit_changes(&self.current_patient_id, &format!("Added BP: {}/{}", systolic, diastolic)) {
                                            Ok(_) => {
                                                match emr.save_patient(&self.current_patient_id, &self.patient_key) {
                                                    Ok(_) => {
                                                        self.status_message = format!("Blood pressure {}/{} added successfully", systolic, diastolic);
                                                        self.vital_signs = VitalSignsForm::default();
                                                        self.current_view = View::ViewPatient;
                                                    },
                                                    Err(e) => {
                                                        self.status_message = format!("Error saving patient: {}", e);
                                                    }
                                                }
                                            },
                                            Err(e) => {
                                                self.status_message = format!("Error committing changes: {}", e);
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        self.status_message = format!("Error adding blood pressure: {}", e);
                                    }
                                }
                            },
                            Err(_) => {
                                self.status_message = "Error accessing EMR".to_string();
                            }
                        }
                    },
                    _ => {
                        self.status_message = "Error: Blood pressure values must be numbers".to_string();
                    }
                }
            }
        }
        
        if ui.button("Cancel").clicked() {
            self.current_view = View::ViewPatient;
            self.vital_signs = VitalSignsForm::default();
        }
    }
    
    fn render_prescribe_view(&mut self, ui: &mut Ui) {
        ui.heading("Prescribe Medication");
        ui.add_space(10.0);
        
        ui.label(format!("Patient ID: {}", self.current_patient_id));
        ui.add_space(10.0);
        
        ui.horizontal(|ui| {
            ui.label("Medication Name: ");
            ui.text_edit_singleline(&mut self.medication.name);
        });
        
        ui.horizontal(|ui| {
            ui.label("Dose (mg): ");
            ui.text_edit_singleline(&mut self.medication.dose_mg);
        });
        
        ui.horizontal(|ui| {
            ui.label("Frequency: ");
            egui::ComboBox::from_id_source("frequency_combo")
                .selected_text(&self.medication.frequency)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.medication.frequency, "daily".to_string(), "Daily");
                    ui.selectable_value(&mut self.medication.frequency, "twice daily".to_string(), "Twice Daily");
                    ui.selectable_value(&mut self.medication.frequency, "three times daily".to_string(), "Three Times Daily");
                    ui.selectable_value(&mut self.medication.frequency, "as needed".to_string(), "As Needed");
                });
        });
        
        ui.add_space(10.0);
        
        if ui.button("Prescribe Medication").clicked() {
            if self.medication.name.is_empty() || self.medication.dose_mg.is_empty() {
                self.status_message = "Error: Medication name and dose are required".to_string();
            } else {
                match self.medication.dose_mg.parse::<f64>() {
                    Ok(dose) => {
                        match self.emr.lock() {
                            Ok(mut emr) => {
                                match emr.prescribe_medication(
                                    &self.current_patient_id,
                                    &self.medication.name,
                                    dose,
                                    &self.medication.frequency
                                ) {
                                    Ok(_) => {
                                        match emr.commit_changes(&self.current_patient_id, &format!(
                                            "Prescribed {} {}mg {}", 
                                            self.medication.name, 
                                            dose, 
                                            self.medication.frequency
                                        )) {
                                            Ok(_) => {
                                                match emr.save_patient(&self.current_patient_id, &self.patient_key) {
                                                    Ok(_) => {
                                                        self.status_message = format!(
                                                            "Prescribed {} {}mg {} successfully", 
                                                            self.medication.name, 
                                                            dose, 
                                                            self.medication.frequency
                                                        );
                                                        self.medication = MedicationForm::default();
                                                        self.current_view = View::ViewPatient;
                                                    },
                                                    Err(e) => {
                                                        self.status_message = format!("Error saving patient: {}", e);
                                                    }
                                                }
                                            },
                                            Err(e) => {
                                                self.status_message = format!("Error committing changes: {}", e);
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        self.status_message = format!("Error prescribing medication: {}", e);
                                    }
                                }
                            },
                            Err(_) => {
                                self.status_message = "Error accessing EMR".to_string();
                            }
                        }
                    },
                    Err(_) => {
                        self.status_message = "Error: Dose must be a number".to_string();
                    }
                }
            }
        }
        
        if ui.button("Cancel").clicked() {
            self.current_view = View::ViewPatient;
            self.medication = MedicationForm::default();
        }
    }
    
    fn render_view_patient(&mut self, ui: &mut Ui) {
        ui.heading("Patient Record");
        ui.add_space(10.0);
        
        match self.emr.lock() {
            Ok(emr) => {
                if let Some(bundle) = emr.bundles.get(&self.current_patient_id) {
                    // Display patient information
                    ui.heading("Patient Information");
                    
                    if let Some(entry) = bundle.entry.iter().find(|e| {
                        matches!(e.resource, Resource::Patient(_))
                    }) {
                        if let Resource::Patient(patient) = &entry.resource {
                            if let Some(name) = patient.name.first() {
                                let given = name.given.join(" ");
                                let family = name.family.clone().unwrap_or_default();
                                ui.label(format!("Name: {} {}", given, family));
                            }
                            ui.label(format!("Gender: {}", patient.gender));
                            ui.label(format!("Birth Date: {}", patient.birth_date));
                            ui.add_space(10.0);
                        }
                    }
                    
                    // Display vital signs
                    ui.collapsing("Vital Signs", |ui| {
                        let observations = bundle.entry.iter()
                            .filter_map(|e| {
                                if let Resource::Observation(obs) = &e.resource {
                                    if obs.code.display.contains("Blood pressure") {
                                        return Some(obs);
                                    }
                                }
                                None
                            })
                            .collect::<Vec<_>>();
                        
                        if observations.is_empty() {
                            ui.label("No vital signs recorded");
                        } else {
                            for obs in observations {
                                if let Some(components) = &obs.component {
                                    let systolic = components.iter()
                                        .find(|c| c.code.display.contains("Systolic"))
                                        .map(|c| c.value_quantity.value.to_string())
                                        .unwrap_or_else(|| "N/A".to_string());
                                    
                                    let diastolic = components.iter()
                                        .find(|c| c.code.display.contains("Diastolic"))
                                        .map(|c| c.value_quantity.value.to_string())
                                        .unwrap_or_else(|| "N/A".to_string());
                                    
                                    ui.label(format!("{} - BP: {}/{} mmHg", 
                                        obs.effective_date_time, systolic, diastolic));
                                }
                            }
                        }
                    });
                    
                    // Display medications
                    ui.collapsing("Medications", |ui| {
                        let medications = bundle.entry.iter()
                            .filter_map(|e| {
                                if let Resource::MedicationRequest(med) = &e.resource {
                                    return Some(med);
                                }
                                None
                            })
                            .collect::<Vec<_>>();
                        
                        if medications.is_empty() {
                            ui.label("No medications prescribed");
                        } else {
                            for med in medications {
                                let dosage_text = med.dosage_instruction.first()
                                    .map(|d| d.text.clone())
                                    .unwrap_or_else(|| "No dosage information".to_string());
                                
                                ui.label(format!("{} - {}: {}", 
                                    med.authored_on, med.medication_codeable_concept.display, dosage_text));
                            }
                        }
                    });
                    
                    // Display version history
                    ui.collapsing("Version History", |ui| {
                        for (i, version) in bundle.version_history.iter().enumerate() {
                            ui.label(format!("Version {}: {} - {}", 
                                i+1, version.timestamp, version.message));
                        }
                    });
                    
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        if ui.button("Add Vital Signs").clicked() {
                            self.current_view = View::AddVitals;
                        }
                        
                        if ui.button("Prescribe Medication").clicked() {
                            self.current_view = View::Prescribe;
                        }
                    });
                } else {
                    ui.label(format!("No data found for patient ID: {}", self.current_patient_id));
                }
            },
            Err(_) => {
                ui.label("Error accessing EMR");
            }
        }
        
        if ui.button("Back to Home").clicked() {
            self.current_view = View::Home;
        }
    }
    
    fn render_load_patient_view(&mut self, ui: &mut Ui) {
        ui.heading("Load Patient Record");
        ui.add_space(10.0);
        
        ui.horizontal(|ui| {
            ui.label("File Path: ");
            ui.text_edit_singleline(&mut self.load_path);
            
            if ui.button("Browse").clicked() {
                // In a real application, we would show a file dialog here
                self.status_message = "File browsing not implemented in this prototype".to_string();
            }
        });
        
        ui.horizontal(|ui| {
            ui.label("Encryption Key: ");
            ui.add(TextEdit::singleline(&mut self.patient_key).password(true));
        });
        
        ui.add_space(10.0);
        
        if ui.button("Load Patient").clicked() {
            if self.load_path.is_empty() || self.patient_key.is_empty() {
                self.status_message = "Error: File path and encryption key are required".to_string();
            } else {
                match self.emr.lock() {
                    Ok(mut emr) => {
                        match emr.load_patient(&self.load_path, &self.patient_key) {
                            Ok(patient_id) => {
                                self.current_patient_id = patient_id;
                                self.status_message = format!("Patient loaded successfully from {}", self.load_path);
                                self.current_view = View::ViewPatient;
                                self.load_path = String::new();
                            },
                            Err(e) => {
                                self.status_message = format!("Error loading patient: {}", e);
                            }
                        }
                    },
                    Err(_) => {
                        self.status_message = "Error accessing EMR".to_string();
                    }
                }
            }
        }
        
        if ui.button("Cancel").clicked() {
            self.current_view = View::Home;
            self.load_path = String::new();
        }
    }
}

struct PatientForm {
    id: String,
    given_name: String,
    family_name: String,
    gender: String,
    birth_date: String,
    key: String,
}

struct VitalSignsForm {
    systolic: String,
    diastolic: String,
}

struct MedicationForm {
    name: String,
    dose_mg: String,
    frequency: String,
}

enum View {
    Home,
    CreatePatient,
    AddVitals,
    Prescribe,
    ViewPatient,
    LoadPatient,
}

impl Default for PatientForm {
    fn default() -> Self {
        Self {
            id: String::new(),
            given_name: String::new(),
            family_name: String::new(),
            gender: String::from("male"),
            birth_date: String::new(),
            key: String::new(),
        }
    }
}

impl Default for VitalSignsForm {
    fn default() -> Self {
        Self {
            systolic: String::new(),
            diastolic: String::new(),
        }
    }
}

impl Default for MedicationForm {
    fn default() -> Self {
        Self {
            name: String::new(),
            dose_mg: String::new(),
            frequency: String::from("daily"),
        }
    }
}

impl Default for EMRApp {
    fn default() -> Self {
        Self {
            emr: Arc::new(Mutex::new(EMR::new().unwrap_or_else(|_| {
                eprintln!("Failed to create EMR. Using empty instance.");
                EMR {
                    bundles: HashMap::new(),
                    audit_log: std::fs::OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open("audit.log")
                        .expect("Failed to create audit log file"),
                }
            }))),
            current_patient_id: String::new(),
            patient_key: String::new(),
            status_message: String::from("Welcome to Charcot EMR"),
            new_patient: PatientForm::default(),
            vital_signs: VitalSignsForm::default(),
            medication: MedicationForm::default(),
            current_view: View::Home,
            load_path: String::new(),
        }
    }
}