#![allow(dead_code)]

use server::Server;
use std::env;
use handler::RequesteHandler;

mod http;
mod server;
mod handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("Public path: {}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(RequesteHandler::new(public_path));
}
