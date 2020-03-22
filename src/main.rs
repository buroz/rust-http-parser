mod config;
mod request;
mod response;

use dotenv::dotenv;
use std::net::TcpListener;
use std::thread;

extern crate httparse;

fn main() {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();
    let http = format!("{}:{}", config.server.host, config.server.port);
    let listener = TcpListener::bind(http).unwrap();

    loop {
        match listener.accept() {
            Ok((stream, addr)) => thread::spawn(move || {
                request::handle_request(stream, addr);
            }),
            Err(e) => thread::spawn(move || println!("Connection failed: {:?}", e)),
        };
    }
}
