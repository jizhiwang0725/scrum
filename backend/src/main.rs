mod app;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = app::build_app();
    
    let bind_addr = std::env::var("BIND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    /// Listen only on the local maching during development
    let listener =
        tokio::net::TcpListener::bind(bind_addr.as_str()).await?;

    println!("Server: http://{bind_addr}");
    axum::serve(listener, app).await
}
