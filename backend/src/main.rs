mod app;
mod config;
mod http;
mod state;
mod models;
mod data;
mod recipes;

use crate::state::AppState;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Show logs in the terminal.
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let recipes = data::load_recipes().expect("Failed to load recipes");

    let state = Arc::new(AppState {
        recipes,
    });

    let app = app::build_app(state);

    // Add CORS and request logs to the existing routes.
    let app = http::with_middleware(app, config::frontend_origins()?);

    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    // Listen only on the local maching during development
    let listener = tokio::net::TcpListener::bind(bind_addr.as_str()).await?;

    println!("Server: http://{bind_addr}");
    axum::serve(listener, app).await
}
