use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse{
    status: &'static str,
}

async fn health() -> Json<HealthResponse>{
    Json(HealthResponse { status: "ok"}) //Return Json Response
}

#[tokio::main]
async fn main() -> std::io::Result<()>{
    let app = Router::new().route("/api/health", get(health));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
        //listen to 3000 port

    println!("Server: http://127.0.0.1:3000");
    axum::serve(listener, app).await //Receive Request
}
