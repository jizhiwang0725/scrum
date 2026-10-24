use crate::recipes::{get_recipe_by_id, get_recipes, search_recipes};
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

// Axum's State extractor provides the Arc<AppState> installed by build_app.
// Read recipe_names without copying the dataset or introducing a lock.
async fn inspect_state(State(state): State<Arc<AppState>>) -> Json<StateSummary> {
    Json(StateSummary {
        recipe_count: state.recipes.len(),
    })
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[derive(Serialize)]
struct QueryEcho {
    demo: bool,
    params: SearchParams,
}

async fn echo_query(RawQuery(raw): RawQuery) -> Result<Json<QueryEcho>, ApiError> {
    // Validate every parameter first. The ? operator returns ApiError immediately on failure.
    let params = parse_search_query(raw.as_deref())?;

    params.ensure_supported(false)?;
    Ok(Json(QueryEcho { demo: true, params }))
}

// Report HTTP availability without checking data loading or external dependencies.
// Reject a nonempty raw query; a missing or empty query string contains no parameters.
async fn health(RawQuery(raw): RawQuery) -> Result<Json<HealthResponse>, ApiError> {
    if raw.as_deref().is_some_and(|query| !query.is_empty()) {
        return Err(ApiError::invalid(
            "query",
            "This endpoint does not accept query parameters.",
        ));
    }
    Ok(Json(HealthResponse { status: "ok" }))
}

// Return the route-level OPTIONS response with no body and an Allow header.
// http::with_middleware installs CorsLayer, which may handle OPTIONS before this handler.
// http::preflight_no_content converts successful CORS preflight responses to 204.
async fn options() -> impl axum::response::IntoResponse {
    (StatusCode::NO_CONTENT, [(header::ALLOW, ALLOWED_METHODS)])
}

// Delegate unmatched-path errors to error::ApiError::not_found.
async fn not_found() -> ApiError {
    ApiError::not_found()
}

// Delegate unsupported-method errors to error::ApiError::method_not_allowed.
async fn method_not_allowed() -> ApiError {
    ApiError::method_not_allowed()
}

pub fn build_app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/health", get(health).options(options))
        .route("/api/debug/query", get(echo_query).options(options))
        .route("/api/debug/state", get(inspect_state).options(options))
        .route("/api/recipes", get(get_recipes))
        .route("/api/recipes/{id}", get(get_recipe_by_id))
        .route("/api/recipes/search/{keyword}", get(search_recipes))
        .fallback(not_found)
        .method_not_allowed_fallback(method_not_allowed)
        .with_state(state)
}

// Compile app_tests.rs only when tests are enabled.
#[cfg(test)]
#[path = "app_tests.rs"]
mod tests;
