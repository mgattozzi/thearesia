// A tiny async echo server with tokio-core

extern crate hyper;
extern crate futures;

use futures::Future;
use hyper::server::{Service, NewService, Http};
use hyper::server;
use hyper::error;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn main() {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    Http::new()
        .bind(&socket, Webhook)
        .unwrap()
        .run()
        .unwrap()
}

struct Webhook;

impl Service for Webhook {
    type Request = server::Request;
    type Response = server::Response;
    type Error = error::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;
    fn call(&self, req: server::Request) -> Self::Future {
        let resp = server::Response::new();
        println!("I have a connection");
        futures::finished(resp).boxed()
    }
}

impl NewService for Webhook {
    type Request = server::Request;
    type Response = server::Response;
    type Error = error::Error;
    type Instance = Webhook;
    fn new_service(&self) -> Result<Self::Instance, std::io::Error> {
        Ok(Webhook)
    }
}
