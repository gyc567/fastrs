#[macro_use]
extern crate serde_derive;
extern crate hyper;
extern crate pretty_env_logger;

use hyper::client::IntoUrl;
use hyper::header::Host;
use hyper::server::{Handler, Listening, Request, Response, Server};
use hyper::status::StatusCode;
use hyper::uri::RequestUri;
use hyper::Client;
use std::error::Error;
use std::io;

pub mod fastrs_config;

// We need to define a handler because we need to pass in an upstream port
// number the HTTP client will connect to.
struct ProxyHandler {
    upstream_port: u16,
}

impl Handler for ProxyHandler {
    fn handle(&self, request: Request, response: Response) {
        pipe_through(request, response, self.upstream_port);
    }
}

pub fn start_server(port: u16, upstream_port: u16) -> Listening {
    let address = "127.0.0.1:".to_owned() + &port.to_string();
    let server = Server::http(&address).unwrap();
    let listening = server
        .handle(ProxyHandler {
            upstream_port: upstream_port,
        })
        .unwrap();
    println!("Listening on {}", address);

    listening
}

fn pipe_through(request: Request, mut response: Response, upstream_port: u16) {
    let path = match request.uri {
        RequestUri::AbsolutePath(p) | RequestUri::Authority(p) => p,
        RequestUri::AbsoluteUri(url) => url.path().to_string(),
        RequestUri::Star => "*".to_string(),
    };
    let host = match request.headers.get::<Host>() {
        None => {
            return error_page(
                "No host header in request",
                StatusCode::BadRequest,
                response,
            )
        }
        Some(h) => h,
    };
    let hostname = host.hostname.to_string();
    // String concatenation is complicated in Rust. In order to create a new variable which
    // concatenates 3 strings we first have to allocate memory by making the first variable a
    // string.
    let protocol = "http://".to_string();
    let url_string = protocol + &hostname + ":" + &upstream_port.to_string() + &path;
    let url = match url_string.into_url() {
        Ok(u) => u,
        Err(e) => {
            return error_page(
                &format!(
                    "Error parsing Host header '{}': {}",
                    url_string,
                    e.description()
                ),
                StatusCode::InternalServerError,
                response,
            )
        }
    };

    // @todo Add proxy config so that requests always go to localhost and this is not an open
    // relay.
    let client = Client::new();

    let request_builder = client
        .request(request.method, url)
        .headers(request.headers.clone());
    let mut upstream_response = request_builder.send().unwrap();
    *response.status_mut() = upstream_response.status;
    // Cloning is quite useless here, we actually just want to move the headers. But how?
    *response.headers_mut() = upstream_response.headers.clone();

    // Forward the body of the upstream response in our response body.
    io::copy(&mut upstream_response, &mut response.start().unwrap()).unwrap();
}

/**
 * Sets an error response.
 */
fn error_page(message: &str, http_code: StatusCode, mut response: Response) {
    println!("{}", message);
    *response.status_mut() = http_code;
    // @todo set response body with the message.
}
