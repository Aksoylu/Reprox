use std::{fs::File, io::Read};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub http_server: HttpServerConfig,
    pub rpc_server: Option<RpcServerConfig>,
}

#[derive(Debug, Deserialize)]
pub struct HttpServerConfig {
    pub ip_address: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct RpcServerConfig {
    pub port: u16,
    pub private_key: String,
}

impl Config {
    const CONFIG_PATH: &'static str = "../variables/settings.json";

    pub fn load() -> Self {

        let file = File::open(Config::CONFIG_PATH).expect("Unable to open file");

        let mut reader = std::io::BufReader::new(file);

        let mut content = String::new();
        reader
            .read_to_string(&mut content)
            .expect("Unable to read file");

        serde_json::from_str(&content).expect("Unable to parse JSON")
    }
}
