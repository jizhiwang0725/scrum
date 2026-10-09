mod app;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = app::build_app();
    
    /// Listen only on the local maching during development
    let listener =
        tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    println!("Server: http://127.0.0.1:3000");
    axum::serve(listener, app).await
}
