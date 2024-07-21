use crate::rpc_service::models::add_http_route_model::{AddHttpRouteRequest, AddHttpRouteResponse};
use crate::{models::route::HttpRoute, runtime::Runtime};
use jsonrpc_core::{Error, Params, Value};
use std::sync::Arc;
use std::sync::Mutex;
use jsonrpc_core::ErrorCode;

pub fn add_http_route(runtime: Arc<Mutex<Runtime>>, params: Params) -> Result<Value, Error> {
    let request = match AddHttpRouteRequest::new(params) {
        Ok(req) => req,
        Err(_) => {
            return Err(Error {
                code: ErrorCode::InternalError,
                message: "Invalid request parameters for JRPC function: add_http_route".into(),
                data: None,
            });
        }
    };
    
    let new_route = HttpRoute {
        source: request.get_source(),
        target: request.get_target()
    };

    let runtime_snapshot = runtime.lock().unwrap().router.clone();
    let updated_runtime_snapshot = runtime_snapshot.add_http_route( new_route);

    runtime.lock().unwrap().router = updated_runtime_snapshot;

    Ok(AddHttpRouteResponse::build("ok".to_string(), None))
}
