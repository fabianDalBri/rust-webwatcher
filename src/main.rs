use axum::{routing::get, Router};
use axum::response::Json;
use serde::Serialize;
use std::net::SocketAddr;
use tracing_subscriber;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

#[tokio::main]
async fn main() {
    // Set up basic logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    // Build router
    let app = Router::new().route("/health", get(health));

    // Server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server running at http://{}/health", addr);

    // axum 0.7 style: use TcpListener + axum::serve
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("server error");
}
