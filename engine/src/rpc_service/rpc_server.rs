use std::sync::Arc;
use jsonrpc_core::IoHandler;
use std::sync::Mutex;

use crate::{runtime::Runtime, utils::network_utility::parse_ip_address};

use jsonrpc_http_server::ServerBuilder;

use super::routes::RPCRouter;

pub struct RPCServer {
    ip_address: String,
    port: u16,
    auth_token: String,
    endpoint: String,
    rpc_registry: IoHandler,
    runtime: Arc<Mutex<Runtime>>
}

impl RPCServer {
    pub async fn singleton(ip_address: String, port: u16, auth_token: String, runtime: Arc<Mutex<Runtime>>) -> Self {
        let parsed_ip_address = parse_ip_address(ip_address.clone());
        let endpoint = format!("{}:{}", parsed_ip_address, port);

        let router: RPCRouter = RPCRouter::build(runtime.clone());

        let mut rpc_registry: IoHandler = IoHandler::default();
        rpc_registry = router.bind(rpc_registry.clone(), runtime.clone()).await;

        Self {
            ip_address,
            port,
            auth_token,
            endpoint,
            rpc_registry,
            runtime
        }
    }

    /// Public: This function is repsonsible of booting process of  JRPC Server
    pub async fn start(&self) {
        let server = ServerBuilder::new(self.rpc_registry.clone())
            .start_http(&self.endpoint.parse().unwrap())
            .expect("JRPC Server failed to start.");

        println!("JRPC Server running on {}", &self.endpoint);

        server.wait();
    }

}
