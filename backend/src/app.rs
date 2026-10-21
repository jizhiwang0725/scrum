use crate::{
    error::{ALLOWED_METHODS, ApiError},
    query::{SearchParams, parse_search_query},
    state::AppState,
};
use axum::{
    Json, Router,
    extract::{RawQuery, State},
    http::{StatusCode, header},
    routing::get,
};
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
struct StateSummary {
    recipe_count: usize,
}

async fn inspect_state(State(state): State<Arc<AppState>>) -> Json<StateSummary> {
    Json(StateSummary {
        recipe_count: state.recipe_names.len(),
    })
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

/// Debug-only response: not the public RecipeList response.
#[derive(Serialize)]
struct QueryEcho {
    demo: bool,
    params: SearchParams,
}

async fn echo_query(RawQuery(raw): RawQuery) -> Result<Json<QueryEcho>, ApiError> {
    let params = parse_search_query(raw.as_deref())?;
    // Current state contains recipe names only; ingredient search is not wired yet.
    params.ensure_supported(false)?;
    Ok(Json(QueryEcho { demo: true, params }))
}

async fn health(RawQuery(raw): RawQuery) -> Result<Json<HealthResponse>, ApiError> {
    if raw.as_deref().is_some_and(|query| !query.is_empty()) {
        return Err(ApiError::invalid("query", "此接口不接受查询参数。"));
    }
    Ok(Json(HealthResponse { status: "ok" }))
}

async fn options() -> impl axum::response::IntoResponse {
    (StatusCode::NO_CONTENT, [(header::ALLOW, ALLOWED_METHODS)])
}

async fn not_found() -> ApiError {
    ApiError::not_found()
}

async fn method_not_allowed() -> ApiError {
    ApiError::method_not_allowed()
}

pub fn build_app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/health", get(health).options(options))
        .route("/api/debug/query", get(echo_query).options(options))
        .route("/api/debug/state", get(inspect_state).options(options))
        .fallback(not_found)
        .method_not_allowed_fallback(method_not_allowed)
        .with_state(state)
}

#[cfg(test)]
#[path = "app_tests.rs"]
mod tests;
