use serde::{Deserialize, Serialize};

use crate::constants::Constants;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct HttpsServerConfig {
    pub ip_address: String,
    pub port: u16,
}

impl Default for HttpsServerConfig {
    fn default() -> Self {
        Self {
            ip_address: Constants::DEFAULT_HTTPS_IP.to_string(),
            port: Constants::DEFAULT_HTTPS_PORT,
        }
    }
}
