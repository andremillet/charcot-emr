rust
#[cfg(test)]
mod tests {
    use crate::auth::rbac::is_route_protected;

    #[test]
    fn test_is_route_protected_public() {
        assert_eq!(is_route_protected("/patient/profile"), false);
    }

    #[test]
    fn test_is_route_protected_prescription_send() {
        assert_eq!(is_route_protected("/prescription/send"), true);
    }

    #[test]
    fn test_is_route_protected_patient_medications() {
        assert_eq!(is_route_protected("/patient/medications"), true);
    }
}