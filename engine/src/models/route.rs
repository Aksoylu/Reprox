use serde::{Deserialize, Serialize};

use super::ssl_path::SslPath;

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct HttpRoute {
    pub source: String,
    pub target: String,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct HttpsRoute {
    pub source: String,
    pub target: String,
    pub ssl_path: SslPath,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct JsonRoute {
    pub protocol: String,
    pub source: String,
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl: Option<SslPath>,
}
