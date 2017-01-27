//! Thearesia a tool to maintain GitHub Repos and Organizations
#![cfg_attr(feature = "dev", plugin(clippy))]
#![cfg_attr(feature = "dev", plugin)]

extern crate hyper;
extern crate futures;
extern crate serde;
extern crate serde_json;

use futures::{Future, Stream};
use futures::future;
use hyper::server::{Service, Http};
use hyper::server;
use hyper::error;
use hyper::Method::Post;
use hyper::status::StatusCode::MethodNotAllowed;
use serde_json::Value;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn main() {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let _ = Http::new()
        .bind(&socket, || Ok(Webhook))
        .map(|server| server.run())
        .map_err(|e| println!("Server failed to setup: {}", e));
}

struct Webhook;

impl Service for Webhook {
    type Request = server::Request;
    type Response = server::Response;
    type Error = error::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;
    fn call(&self, req: server::Request) -> Self::Future {
        let (method, _, _, _headers, body) = req.deconstruct();

        // Make sure we only recieve POST requests from Github
        if method == Post {

            // Get all of the chunks streamed to us in our request
            // GitHub gives us a lot of data so there might be
            // more than one Chunk
            body.fold(Vec::new(), |mut v, chunk| {
                    v.extend(&chunk[..]);
                    future::ok::<_, error::Error>(v)
                })
                // If there is JSON do things with it
                // Send to the server that we got the data
                .map(move |buffer| {
                    if !buffer.is_empty() {
                        println!("{:#?}", serde_json::from_slice::<Value>(&buffer).unwrap());
                    }

                    server::Response::new()
                }).boxed()

        } else {

            let mut res = server::Response::new();
            res.set_status(MethodNotAllowed);
            future::ok(res).boxed()

        }
    }
}
