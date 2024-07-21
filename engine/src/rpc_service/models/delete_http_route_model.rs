use crate::utils::rpc_utility::RpcParameter;
use jsonrpc_core::{Error, Params, Value};
use serde::Deserialize;
use serde::Serialize;

pub struct DeleteHttpRouteRequest {
    source: String,
}

impl DeleteHttpRouteRequest {
    pub fn new(params: Params) -> Result<Self, Error> {
        let source: Option<String> = RpcParameter::extract_string("source", params.clone());

        if source.is_none() {
            return Err(Error {
                code: jsonrpc_core::ErrorCode::ServerError(500),
                message: "Provide at least 'source' parameter".into(),
                data: None,
            });
        }

        Ok(Self {source: source.unwrap()})
    }

    pub fn get_source(&self) -> String {
        self.source.clone()
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeleteHttpRouteResponse {
    code: i32,
    message: String
}

impl DeleteHttpRouteResponse {
    pub fn build() -> jsonrpc_core::Value {
        let response = DeleteHttpRouteResponse {
            code: 200,
            message: "ok".to_owned()
        };

        let serialized_json = match serde_json::to_string(&response) {
            Ok(text) => text,
            Err(error) => error.to_string(),
        };

        Value::String(serialized_json)
    }
}
