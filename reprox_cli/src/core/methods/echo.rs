use crate::core::rpc_client::{self, RpcClient};

pub async fn echo(rpc_client: Option<&RpcClient>) {
    if let Some(rpc) = rpc_client {
        let test: bool = rpc_echo(rpc).await;
        if test {
            println!("Successfully connected to Reprox Engine");
        } else {
            println!("Failed to establish connection to Reprox Engine");
        }
    } else {
        println!("Please run 'Connect' for  connect Reprox Engine manually. ");
    }
}

async fn rpc_echo(rpc_client: &RpcClient) -> bool {
    match rpc_client.send_rpc("echo".to_string(), None).await {
        Ok(_) => true,
        Err(_) => false,
    }
}

// @todo
fn help() {}
