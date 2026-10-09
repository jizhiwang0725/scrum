use axum::{routing::get, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse{
    status: &'static str,
}

async fn health() -> Json<HealthResponse>{
    Json(HealthResponse { status: "ok"})
}

pub fn build_app() -> Router{
    Router::new()
        .route("/api/health", get(health))
}
