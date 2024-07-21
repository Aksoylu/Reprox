use serde::{Deserialize, Serialize};

use crate::{
    constants::Constants,
    utils::{crypt_utility::generate_hash, time_utility::get_current_timestamp},
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RpcSession {
    pub ip_addr: String,
    pub port: u16,
    pub private_key: String,
    pub created_at: i64,
    pub hash: String,
}

impl Default for RpcSession {
    fn default() -> Self {
        let hash = generate_hash(Constants::DEFAULT_PRIVATE_KEY.to_string());

        Self {
            ip_addr: Constants::DEFAULT_RPC_IP.to_string(),
            port: Constants::DEFAULT_RPC_PORT,
            private_key: Constants::DEFAULT_PRIVATE_KEY.to_string(),
            created_at: get_current_timestamp(),
            hash: generate_hash(Constants::DEFAULT_PRIVATE_KEY.to_string()),
        }
    }
}

impl RpcSession {
    pub fn validate(&self) -> Result<(), String> {
        if self.ip_addr.is_empty() {
            return Err("RPC Connection Ip Address is empty".into());
        }
        if self.port == 0 {
            return Err("RPC Connection port is 0".into());
        }

        if self.private_key.is_empty() {
            return Err("RPC Connection private key is empty".into());
        }
        if self.hash.is_empty() {
            return Err("RPC Connection hash is empty.".into());
        }

        Ok(())
    }
}
