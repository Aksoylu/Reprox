use serde::{Deserialize, Serialize};

use crate::constants::Constants;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RpcServerConfig {
    pub ip_address: String,
    pub port: u16,
    pub private_key: String,
}

impl Default for RpcServerConfig {
    fn default() -> Self {
        Self {
            ip_address: Constants::DEFAULT_RPC_IP.to_string(),
            port: Constants::DEFAULT_RPC_PORT,
            private_key: Constants::DEFAULT_PRIVATE_KEY.to_string(),
        }
    }
}
