use jsonrpc_core::Params;
use serde_json::Value;

pub struct RpcParameter {}

impl RpcParameter {
    pub fn extract_string(parameter_name: &str, params: Params) -> Option<String> {
        if let Ok(value) = params.parse::<Value>() {
            if let Some(param_value) = value.get(parameter_name) {
                if let Some(string_value) = param_value.as_str() {
                    return Some(string_value.to_string());
                }
            }
        }
        None
    }
}
