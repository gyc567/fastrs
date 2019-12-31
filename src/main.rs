extern crate hyper;
use hyper::server::{Request, Response, Server};

fn main() {
    let server = Server::http("127.0.0.1:9090").unwrap();
    let _guard = server.handle(pipe_through);
    println!("Listening on http://127.0.0.1:9090");
}

fn pipe_through(request: Request, mut response: Response) {}
