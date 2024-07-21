use jsonrpc_core::{Params, Value};

use crate::utils::text_utility::clear_punctation;

pub fn authorization(authorization_token: String, params: Params) -> Result<(), ()> {
    let auth_token = params
        .clone()
        .parse::<Value>()
        .ok()
        .and_then(|v| v.get("token").map(|t| t.to_string()));

    if let Some(auth_token) = auth_token {
        let cleared_auth_token = clear_punctation(auth_token);
        println!("{}", cleared_auth_token);

        if cleared_auth_token == authorization_token {
            return Ok(());
        }
    }

    Err(())
}
