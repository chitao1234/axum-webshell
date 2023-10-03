#![feature(async_fn_in_trait)]
mod adapter;
mod handler;

use std::net::SocketAddr;

use axum::{routing::get, Router};
use tower_http::services::ServeDir;

use handler::ws_handler;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/", ServeDir::new("html"))
        .route("/ws", get(ws_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
