use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use crate::{
    constants::Constants,
    models::route::{HttpRoute, HttpsRoute, JsonRoute},
    utils::file_utility::{get_runtime_path, load_json, save_json},
};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Router {
    http_route_table: HashMap<String, HttpRoute>,
    https_route_table: HashMap<String, HttpsRoute>,
    save_path: PathBuf,
}

impl Default for Router {
    fn default() -> Self {
        let http_route_table: HashMap<String, HttpRoute> = HashMap::new();
        let https_route_table: HashMap<String, HttpsRoute> = HashMap::new();

        let mut save_path = get_runtime_path().clone();
        save_path.push(Constants::ROUTER_FILENAME);

        Self {
            http_route_table,
            https_route_table,
            save_path,
        }
    }
}

impl Router {
    /* File / Constructor Functions */
    pub fn load(load_path: PathBuf) -> Router {
        let read_route_operation = load_json::<Vec<JsonRoute>>(&load_path);
        if read_route_operation.is_err() {
            eprintln!("Could not load Router file on Path: {}.\nVanguard will starting up with no routing", load_path.to_str().unwrap_or_default());
            return Router::default();
        }

        let route_list = read_route_operation.unwrap();
        let router = Router::build(load_path, route_list);
        if router.is_ok() {
            return router.unwrap();
        } else {
            let error_text = router.err().unwrap_or_default();
            eprintln!("Invalid Router Session: {}\nUsing default", error_text);
            return Router::default();
        }
    }

    pub fn save(&self) {
        let export_data = self.convert_to_json_route_vec();
        let write_operation = save_json(&self.save_path, &export_data);
        
        if write_operation.is_err() {
            let save_path_as_string = &self.save_path.to_str().unwrap_or_default();
            eprintln!(
                "Could not write Router file on Path: {}.\nPlease check your runtime path is exist and Vanguard has correct permissions", 
                save_path_as_string
            );
        }
    }

    fn build(save_path: PathBuf, route_list: Vec<JsonRoute>) -> Result<Self, String> {
        let mut http_route_table: HashMap<String, HttpRoute> = HashMap::new();
        let mut https_route_table: HashMap<String, HttpsRoute> = HashMap::new();

        for each_route in route_list {
            let protocol_name = each_route.protocol.clone().to_lowercase();

            if protocol_name == "http" {
                let new_http_route = HttpRoute {
                    source: each_route.source.clone(),
                    target: each_route.target.clone(),
                };
                http_route_table.insert(each_route.source.clone(), new_http_route.clone());
            } else if protocol_name == "https" {
                let new_https_route = HttpsRoute {
                    source: each_route.source.clone(),
                    target: each_route.target.clone(),
                    ssl_path: each_route.ssl.clone().unwrap_or_default(),
                };

                https_route_table.insert(each_route.source.clone(), new_https_route.clone());
            } else {
                return Err(format!("Error: Unsupported protocol: {}", protocol_name).into());
            }
        }

        Ok(Router {
            http_route_table,
            https_route_table,
            save_path,
        })
    }

    fn convert_to_json_route_vec(&self) -> Vec<JsonRoute> {
        let mut json_route_vec = <Vec<JsonRoute>>::new();

        for each_http_entity in &self.http_route_table {
            let each_http_route = each_http_entity.1;

            json_route_vec.push(JsonRoute {
                protocol: "http".to_string(),
                source: each_http_route.source.clone(),
                target: each_http_route.target.clone(),
                ssl: None,
            });
        }

        for each_https_entity in &self.https_route_table {
            let each_https_route = each_https_entity.1;

            json_route_vec.push(JsonRoute {
                protocol: "https".to_string(),
                source: each_https_route.source.clone(),
                target: each_https_route.target.clone(),
                ssl: Some(each_https_route.ssl_path.clone()),
            })
        }

        json_route_vec
    }

    pub fn get_http_routes(&self) -> HashMap<String, HttpRoute> {
        self.http_route_table.clone()
    }

    /* Service Functions */

    pub fn list_http_routes(&self) -> Vec<JsonRoute> {
        let export_data = self.convert_to_json_route_vec();

        export_data
    }

    pub fn add_http_route(mut self, route_body: HttpRoute) -> Self {
        let route_name = route_body.source.clone();

        if self.http_route_table.contains_key(&route_name) {
            self.http_route_table.remove(&route_name);
        }

        self.http_route_table.insert(route_name, route_body);

        self.save();
        self
    }

    pub fn delete_http_route(mut self, source: String) -> Self {
        if self.http_route_table.contains_key(&source) {
            self.http_route_table.remove(&source);
        }

        self.save();
        self
    }

    pub fn get_https_routes(&self) -> HashMap<String, HttpsRoute> {
        self.https_route_table.clone()
    }
}
