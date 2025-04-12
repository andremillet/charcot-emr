rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PatientProfile {
    pub name: String,
    pub age: u32,
    pub address: String,
}


pub fn get_patient_profile() -> PatientProfile {
    PatientProfile {
        name: String::from("John Doe"),
        age: 30,
        address: String::from("123 Main St"),
    }
}

pub fn get_medication_list() -> Vec<String> {
    vec![
        String::from("Metformin"),
        String::from("Lisinopril"),
        String::from("Atorvastatin"),
    ]
}