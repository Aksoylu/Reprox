use hyper::client::HttpConnector;
use hyper::{header, server::conn::Http, service::service_fn, Body, Client, Request, Response};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};
use tokio_rustls::TlsAcceptor;

use crate::models::route::HttpsRoute;
use crate::utils::network_utility::parse_ip_address;
use crate::utils::tls_utility::create_ssl_context;

#[derive(Clone)]
pub struct HttpsServer {
    socket: SocketAddr,
    routes: HashMap<String, HttpsRoute>,
}

impl HttpsServer {
    pub fn singleton(ip_address: String, port: u16, routes: HashMap<String, HttpsRoute>) -> Self {
        let ip = parse_ip_address(ip_address.clone());
        let socket = SocketAddr::from((ip, port));

        Self { socket, routes }
    }

    pub async fn start(&self) {
        let https_server: Arc<Mutex<HttpsServer>> = Arc::new(Mutex::new(self.clone()));
        let ssl_context: TlsAcceptor = create_ssl_context(self.routes.clone()).await;

        println!("Vanguard Engine Https server started on {:?}", &self.socket);
        let listener: TcpListener = TcpListener::bind(&self.socket).await.unwrap();

        /* LifeCycle */
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let tls_acceptor: TlsAcceptor = ssl_context.clone();
            let https_server: Arc<Mutex<HttpsServer>> = Arc::clone(&https_server);
            let self_clone = self.clone();
            tokio::spawn(async move {
                self_clone
                    .lifecycle(tls_acceptor, stream, https_server)
                    .await;
            });
        }
    }

    async fn lifecycle(
        &self,
        tls_acceptor: TlsAcceptor,
        stream: tokio::net::TcpStream,
        https_server: Arc<Mutex<HttpsServer>>,
    ) {
        tokio::spawn(async move {
            let stream = match tls_acceptor.accept(stream).await {
                Ok(stream) => stream,
                Err(e) => {
                    eprintln!("TLS accept error: {:?}", e);
                    return;
                }
            };

            let service = service_fn(move |req: Request<Body>| {
                let https_server: Arc<Mutex<HttpsServer>> = Arc::clone(&https_server);
                async move {
                    let data: tokio::sync::MutexGuard<HttpsServer> = https_server.lock().await;
                    match data.handle_request(req).await {
                        Ok(response) => Ok::<_, hyper::Error>(response),
                        Err(_) => Ok::<_, hyper::Error>(Response::new(Body::from(
                            "Error processing request",
                        ))),
                    }
                }
            });

            let http = Http::new();
            if let Err(e) = http.serve_connection(stream, service).await {
                eprintln!("Server error: {}", e);
            }
        });
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
                "Requested domain has not registered on Vanguard",
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
