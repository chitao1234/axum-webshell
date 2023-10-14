mod adapter;
mod db;
mod handler;

use std::net::SocketAddr;

use axum::{routing::get, Extension, Router};
use axum_login::{
    axum_sessions::{async_session::MemoryStore, SessionLayer},
    AuthLayer, PostgresStore, RequireAuthorizationLayer,
};
use rand::Rng;
use tower_http::services::ServeDir;

use db::create_pool;
use handler::auth::{login_handler, logout_handler, protected_handler, User};
use handler::ws_handler;

#[tokio::main]
async fn main() {
    let mut secret = [0u8; 64];
    rand::thread_rng().fill(&mut secret);
    let secret = secret;

    let session_store = MemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret).with_secure(false);

    let pool = create_pool(env!("DATABASE_URL")).await;
    let user_store = PostgresStore::<User>::new(pool.clone());
    let auth_layer = AuthLayer::new(user_store, &secret);

    let app = Router::new()
        .nest_service("/", ServeDir::new("html/dist"))
        .route("/ws", get(ws_handler))
        .route("/control", get(ws_handler))
        .route("/protected", get(protected_handler))
        .route_layer(RequireAuthorizationLayer::<i64, User>::login())
        .route("/login", get(login_handler))
        .route("/logout", get(logout_handler))
        .layer(auth_layer)
        .layer(session_layer)
        .layer(Extension(pool.clone()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
