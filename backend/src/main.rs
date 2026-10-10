mod app;
mod state;

use std::sync::Arc;
use crate::state::AppState;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let state = Arc::new(AppState{
        recipe_names: vec![
            "番茄炒蛋".to_string(),
            "土豆炖牛肉".to_string(),
        ],
    });

    let app = app::build_app(state);
    
    let bind_addr = std::env::var("BIND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    /// Listen only on the local maching during development
    let listener =
        tokio::net::TcpListener::bind(bind_addr.as_str()).await?;

    println!("Server: http://{bind_addr}");
    axum::serve(listener, app).await
}

