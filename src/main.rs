use axum::{routing::get, Router};
use dotenv::dotenv;

mod config;
mod db;
pub mod handlers;
mod models;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(health_check));
    let app = routes::get_routes(app);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .expect("Failed to bind");

    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.expect("Failed to serve");
}

#[tracing::instrument(name = "Health check")]
async fn health_check() -> &'static str {
    tracing::info!("I'm healthy!");
    "I'm healthy!"
}
