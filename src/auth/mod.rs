pub mod rbac;

pub fn check_authorization(role: &str, route: &str) -> bool {
    if !rbac::is_route_protected(route) {
        return true;
    }
    match route {
        "/prescription/send" => role == "Admin",
        "/patient/medications" => role == "Admin" || role == "Doctor",
        _ => false,
    }
}