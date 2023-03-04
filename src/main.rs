#![deny(warnings)]

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn rustless(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, Rustless!".into()))
}

#[tokio::main]
async fn main() {
    // Bind to 127.0.0.1:8080
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    // A `Service` is needed for every connection, so this
    // creates one from our `rustless` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(rustless))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
