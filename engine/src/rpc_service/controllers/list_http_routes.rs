use jsonrpc_core::{Error, Params, Value};
use std::sync::Mutex;
use std::sync::Arc;

use crate::rpc_service::models::get_http_routes_model::GetHttpRouteResponse;
use crate::runtime::Runtime;

pub fn list_http_routes(runtime: Arc<Mutex<Runtime>>, _params: Params) -> Result<Value, Error> {
    let runtime_snapshot = runtime.lock().unwrap().router.clone();
    let http_routes = runtime_snapshot.list_http_routes();
    
    Ok(GetHttpRouteResponse::build(http_routes))
}
