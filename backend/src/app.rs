use axum::{extract::Query, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use axum::http::StatusCode;

#[derive(Serialize)]
struct HealthResponse{
    status: &'static str,
}

/// Query parameters
#[derive(Deserialize)]
struct SearchParams{
    q: Option<String>,
}

/// Echoes the received query value for inspection
#[derive(Serialize)]
struct QueryEcho{
    q: Option<String>,
}

async fn echo_query(
    query: Query<SearchParams>,
) -> Result<Json<QueryEcho>, (StatusCode, &'static str)> {
    let params = query.0;
    
    if let Some(q) = &params.q{
        if q.chars().count() > 100 {
            return Err((
                    StatusCode::BAD_REQUEST,
                    "q must be at most 100 chs"
            ))
        }
    }
    Ok(Json(QueryEcho { q: params.q}))
}

/// Reports HTTP server availability without checking external dependencies
async fn health() -> Json<HealthResponse>{
    Json(HealthResponse { status: "ok"})
}

///Builds the application's HTTP routes
pub fn build_app() -> Router{
    Router::new()
        .route("/api/health", get(health))
        .route("/api/debug/query", get(echo_query))
}
