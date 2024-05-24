use serde::{Serialize, Deserialize};

/// Request for hostname with service name
#[derive(Deserialize, Serialize, Debug)]
pub struct HostnameWithService {
    pub hostname: String,
    pub service: String,
}
