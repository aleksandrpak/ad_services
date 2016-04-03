#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate micro;
extern crate serde;
extern crate serde_json;

mod service;

use service::Service;
use micro::transport::http::Server;

fn main() {
    let service = Service::new();

    let mut server = Server::new("/geoip");

    server.post("/lookup", move |x| service.lookup(x));

    server.start("0.0.0.0:8080").unwrap();
}
