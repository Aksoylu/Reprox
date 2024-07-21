use std::collections::HashMap;

use crate::models::route::HttpRoute;
use crate::utils::rpc_utility::RpcParameter;
use jsonrpc_core::{Error, Params, Value};
use serde::Deserialize;
use serde::Serialize;
use serde_json::value;

pub struct EchoRequest {
    message: String,
}

impl EchoRequest {
    pub fn new(params: Params) -> Result<Self, Error> {
        let message: Option<String> = RpcParameter::extract_string("message", params.clone());

        if message.is_none() {
            return Err(Error {
                code: jsonrpc_core::ErrorCode::ServerError(500),
                message: "Please add a message".into(),
                data: None,
            });
        }

        Ok(Self {
            message: message.unwrap(),
        })
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }

}

#[derive(Serialize, Deserialize)]
pub struct EchoResponse {
    code: i32,
    message: String,
}

impl EchoResponse {
    pub fn build(message: String) -> jsonrpc_core::Value {
        let response = EchoResponse {
            code: 200,
            message: message,
        };

        let serialized_json: String = match serde_json::to_string(&response) {
            Ok(text) => text,
            Err(error) => error.to_string(),
        };

        Value::String(serialized_json)
    }
}
