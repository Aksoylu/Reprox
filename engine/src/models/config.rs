use serde::{Deserialize, Serialize};

use super::{
    http_server_config::HttpServerConfig, https_server_config::HttpsServerConfig,
    rpc_server_config::RpcServerConfig,
};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Config {
    pub http_server: HttpServerConfig,
    pub https_server: HttpsServerConfig,
    pub rpc_server: RpcServerConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            http_server: Default::default(),
            https_server: Default::default(),
            rpc_server: Default::default(),
        }
    }
}

impl Config {
    pub fn validate(&self) -> Result<(), String> {
        if self.http_server.ip_address.is_empty() {
            return Err("HTTP server IP address is empty".into());
        }
        if self.http_server.port == 0 {
            return Err("HTTP server port is 0".into());
        }

        if self.https_server.ip_address.is_empty() {
            return Err("HTTPS server IP address is empty".into());
        }
        if self.https_server.port == 0 {
            return Err("HTTPS server port is 0".into());
        }

        if self.rpc_server.ip_address.is_empty(){
            return Err("RPC server IP address is empty".into());
        }

        if self.rpc_server.port == 0{
            return Err("RPC server port is 0".into());
        }

        if self.rpc_server.private_key.is_empty(){
            return Err("RPC server private key is empty".into());
        }

        Ok(())
    }
}
