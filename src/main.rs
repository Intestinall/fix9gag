mod axum_runner;
mod config;
mod core;
mod errors;
mod page_manipulation;
mod shared;

use crate::axum_runner::{direct_route, oembed_route};
use crate::config::get_config;
use axum::Router;
pub use shared::ResponseResult;

#[tokio::main]
async fn main() {
    let config = get_config();
    let app = Router::new()
        .merge(direct_route(config.clone()))
        .merge(oembed_route(config.clone()));

    let server_address = format!("{}:{}", config.host, config.port);
    println!("Running server at {server_address}");

    let listener = tokio::net::TcpListener::bind(server_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
