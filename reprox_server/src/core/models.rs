use serde::{Deserialize, Serialize};
use rustls::{PrivateKey, Certificate};
use tokio_rustls::TlsAcceptor;

#[derive(Debug, PartialEq, Clone)]

pub struct Ssl{
    pub cert: Vec<Certificate>,
    pub private_key: PrivateKey,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct SslPath {
    pub cert: String,
    pub private_key: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct HttpRoute {
    pub source: String,
    pub target: String,
}

#[derive(Clone)]
pub struct HttpsRoute {
    pub source: String,
    pub target: String,
    pub ssl_path: SslPath
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct JsonRoute{
    pub protocol: String,
    pub source: String,
    pub target: String,
    pub ssl: Option<SslPath>
}