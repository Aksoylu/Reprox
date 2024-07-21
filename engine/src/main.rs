mod constants;
mod core;
mod models;
mod rpc_service;
mod runtime;
mod utils;

use core::{http_server::HttpServer, https_server::HttpsServer};
use std::sync::Arc;
use std::sync::Mutex;

use tokio::sync::{ watch};
use rpc_service::rpc_server::RPCServer;
use runtime::Runtime;
    // TODO: make runtime dynamically changable with jrpc
    // TODO: make runtime uses constant paths. If not exist, create folders and files by using defaults
#[tokio::main]
async fn main() {
    let runtime = Arc::new(Mutex::new(Runtime::init()));

    let http_runtime = runtime.clone();
    let http_server = {
        let rt = http_runtime.lock().unwrap();

        HttpServer::singleton(
            rt.config.http_server.ip_address.clone(),
            rt.config.http_server.port,
            rt.router.get_http_routes()
        )
    };

    tokio::spawn(async move {
        http_server.start().await;
    });

    let https_runtime = runtime.clone();
    let https_server = {
        let rt = https_runtime.lock().unwrap();
        HttpsServer::singleton(
            rt.config.https_server.ip_address.clone(),
            rt.config.https_server.port,
            rt.router.get_https_routes(),
        )
    };

    tokio::spawn(async move {
        https_server.start().await;
    });

    let jsonrpc_runtime = runtime.clone();
    let jsonrpc_server = {
        let rt = jsonrpc_runtime.lock().unwrap();
        RPCServer::singleton(
            rt.config.rpc_server.ip_address.clone(),
            rt.config.rpc_server.port,
            rt.config.rpc_server.private_key.clone(),
            runtime.clone()
        )
    }.await;

    tokio::spawn(async move {
        jsonrpc_server.start().await;
    });

     
    // Keep the main task running (adjust as necessary for your application)
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
