use crate::state::AppState;
use axum::http::StatusCode;
use axum::{
    Json, Router,
    extract::{Query, State},
    routing::get,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize)]
struct StateSummary {
    recipe_count: usize,
}

/// Reports how many recipte names are available in shared state
async fn inspect_state(State(state): State<Arc<AppState>>) -> Json<StateSummary> {
    Json(StateSummary {
        recipe_count: state.recipe_names.len(),
    })
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

/// Query parameters
#[derive(Deserialize)]
struct SearchParams {
    q: Option<String>,
}

/// Echoes the received query value for inspection
#[derive(Serialize)]
struct QueryEcho {
    q: Option<String>,
}

async fn echo_query(
    query: Query<SearchParams>,
) -> Result<Json<QueryEcho>, (StatusCode, &'static str)> {
    let params = query.0;

    if let Some(q) = &params.q {
        if q.chars().count() > 100 {
            return Err((StatusCode::BAD_REQUEST, "q must be at most 100 chs"));
        }
    }
    Ok(Json(QueryEcho { q: params.q }))
}

/// Reports HTTP server availability without checking external dependencies
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

///Builds the application's HTTP routes
pub fn build_app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route("/api/debug/query", get(echo_query))
        .route("/api/debug/state", get(inspect_state))
        .with_state(state)
}
