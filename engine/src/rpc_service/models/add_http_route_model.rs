use crate::utils::rpc_utility::RpcParameter;
use jsonrpc_core::{Error, Params, Value};
use serde::Deserialize;
use serde::Serialize;

pub struct AddHttpRouteRequest {
    source: String,
    target: String,
}

impl AddHttpRouteRequest {
    pub fn new(params: Params) -> Result<Self, Error> {
        let source: Option<String> = RpcParameter::extract_string("source", params.clone());
        let target = RpcParameter::extract_string("target", params.clone());

        if source.is_none() {
            return Err(Error {
                code: jsonrpc_core::ErrorCode::ServerError(500),
                message: "Route name is not valid".into(),
                data: None,
            });
        }

        if target.is_none() {
            return Err(Error {
                code: jsonrpc_core::ErrorCode::ServerError(500),
                message: "Route name is not valid".into(),
                data: None,
            });
        }

        Ok(Self {
            source: source.unwrap(),
            target: target.unwrap(),
        })
    }

    pub fn get_source(&self) -> String {
        self.source.clone()
    }

    pub fn get_target(&self) -> String {
        self.target.clone()
    }
}

#[derive(Serialize, Deserialize)]
pub struct AddHttpRouteResponse {
    code: i32,
    message: String,
    data: Option<Value>,
}

impl AddHttpRouteResponse {
    pub fn build(message: String, data: Option<Value>) -> jsonrpc_core::Value {
        let response = AddHttpRouteResponse {
            code: 200,
            message,
            data,
        };

        let serialized_json = match serde_json::to_string(&response) {
            Ok(text) => text,
            Err(error) => error.to_string(),
        };

        Value::String(serialized_json)
    }
}
