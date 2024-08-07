use super::models::{HttpRoute, HttpsRoute, JsonRoute};
use crate::settings::Settings;
use std::{collections::HashMap, fs::File, io::Read};

#[derive(Clone)]
pub struct Router {
    http_route_table: HashMap<String, HttpRoute>,
    https_route_table: HashMap<String, HttpsRoute>,
}

impl Router {
    pub fn load() -> Self {
        let mut http_route_table: HashMap<String, HttpRoute> = HashMap::new();
        let mut https_route_table: HashMap<String, HttpsRoute> = HashMap::new();

        let route_list = Self::read_routing_json();
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
                panic!("Error: Unsupported protocol '{}'", protocol_name);
            }
        }

        Self {
            http_route_table,
            https_route_table,
        }
    }

    pub fn get_http_routes(&self) -> HashMap<String, HttpRoute> {
        self.http_route_table.clone()
    }

    pub fn get_https_routes(&self) -> HashMap<String, HttpsRoute> {
        self.https_route_table.clone()
    }

    fn read_routing_json() -> Vec<JsonRoute> {
        let open_file_result = File::open(Settings::ROUTER_PATH);
        if open_file_result.is_err(){
            panic!("Failed to read runtime/routing.json file.");
        }

        let mut file = open_file_result.unwrap();
        let mut file_contents = String::new();
        
        let read_file_result = file.read_to_string(&mut file_contents);
        if read_file_result.is_err(){
            panic!("Failed to parse runtime/routing.json content.");
        }

        match serde_json::from_str(&file_contents) {
            Ok(json_route_vector) => json_route_vector,
            Err(_) => {
                panic!("An error occured while parsing runtime/routing.json.");
            }
        }
    }
}
