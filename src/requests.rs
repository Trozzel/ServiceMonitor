use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct HostnameWithService {
    hostname: &str,
    service: &str,
}
