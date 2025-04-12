rust
#[cfg(test)]
mod tests {
    use crate::api::e_prescribing::send_prescription;

    #[test]
    fn test_send_prescription() {
        let prescription = send_prescription(String::from("Test Patient"), String::from("Test Medication"), String::from("Test Dosage"), 1);
        assert_eq!(prescription.patient_name, "Test Patient");
        assert_eq!(prescription.medication_name, "Test Medication");
        assert_eq!(prescription.dosage, "Test Dosage");
        assert_eq!(prescription.refill_quantity, 1);
        assert_eq!(prescription.doctor_name, "Dr. Smith");
        assert_eq!(prescription.id, 1);
    }
}

    #[test