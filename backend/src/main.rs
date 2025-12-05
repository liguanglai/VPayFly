use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("Starting server...");

    // Build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // Run it with hyper on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}