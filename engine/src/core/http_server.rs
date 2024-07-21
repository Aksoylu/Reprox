use hyper::{
    header,
    service::{make_service_fn, service_fn},
    Body, Client, Request, Response, Server,
};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

use hyper::client::HttpConnector;

use crate::{models::route::HttpRoute, utils::network_utility::parse_ip_address};

#[derive(Debug, Clone)]
pub struct HttpServer {
    socket: SocketAddr,
    routes: HashMap<String, HttpRoute>,
}

impl HttpServer {
    pub fn singleton(ip_address: String, port: u16, routes: HashMap<String, HttpRoute>) -> Self {
        let socket = SocketAddr::from((parse_ip_address(ip_address.clone()), port));

        Self { socket, routes }
    }

    pub async fn start(&self) {
        let http_server = Arc::new(Mutex::new(self.clone()));

        let make_svc = make_service_fn(|_conn| {
            let http_server = Arc::clone(&http_server);

            async move {
                Ok::<_, hyper::Error>(service_fn(move |req| {
                    let http_server = Arc::clone(&http_server);
                    async move {
                        let data = http_server.lock().await;

                        match data.handle_request(req).await {
                            Ok(response) => Ok::<_, hyper::Error>(response),
                            Err(_) => Ok::<_, hyper::Error>(Response::new(Body::from(
                                "Error processing request",
                            ))),
                        }
                    }
                }))
            }
        });

        println!("Vanguard Engine Http server started on {:?}", &self.socket);

        if let Err(e) = Server::bind(&self.socket).serve(make_svc).await {
            eprintln!("Server error: {}", e);
        }
    }

    async fn handle_request(&self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        let request_host = req
            .headers()
            .get(header::HOST)
            .and_then(|host| host.to_str().ok())
            .map_or_else(|| "/".to_string(), |host_value| host_value.to_string());

        if !self.routes.contains_key(&request_host) {
            let response = Response::new(Body::from(
                "Requested domain has not registered on Vanguard",
            ));
            return Ok(response);
        }

        if !self.routes.contains_key(&request_host) {
            let response = Response::new(Body::from(
                "Requested URL is not configured properly. Please contact your system administrator",
            ));
            return Ok(response);
        }

        let current_route = self.routes.get(&request_host).unwrap();

        if String::is_empty(&current_route.source) {
            let response = Response::new(Body::from(
                "Requested domain is not registered on Vanguard Engine.",
            ));
            return Ok(response);
        }

        self.navigate_url(&current_route.target, req).await
    }

    async fn navigate_url(
        &self,
        endpoint_to_navigate: &String,
        req: Request<Body>,
    ) -> Result<Response<Body>, hyper::Error> {
        let original_uri = req.uri().clone();

        let mut new_uri = format!("http://{}{}", endpoint_to_navigate, original_uri.path());
        if let Some(query) = original_uri.query() {
            new_uri.push('?');
            new_uri.push_str(query);
        }

        let (mut parts, body) = req.into_parts();
        parts.uri = new_uri.parse().unwrap();

        let new_request = Request::from_parts(parts, body);

        let http = HttpConnector::new();
        let client: Client<HttpConnector> = Client::builder().build(http);

        let response = client.request(new_request).await?;

        Ok(response)
    }
}
