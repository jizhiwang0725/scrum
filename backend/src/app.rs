//! HTTP routes connecting Axum extractors, query.rs validation, error.rs responses and state.rs.
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
// Debug response containing only the number of names in state::AppState.
struct StateSummary {
    recipe_count: usize,
}

// Axum's State extractor provides the Arc<AppState> installed by build_app.
// Read recipe_names without copying the dataset or introducing a lock.
async fn inspect_state(State(state): State<Arc<AppState>>) -> Json<StateSummary> {
    Json(StateSummary {
        recipe_count: state.recipe_names.len(),
    })
}

#[derive(Serialize)]
// Serialize the health response as { "status": "ok" }.
struct HealthResponse {
    status: &'static str,
}

#[derive(Serialize)]
// Echo normalized parameters for debugging; this is not the public recipe-list response.
// The demo field makes the temporary response explicit.
struct QueryEcho {
    demo: bool,
    params: SearchParams,
}

// RawQuery provides the original query string without permissive decoding.
// Pass it to query::parse_search_query to detect invalid escapes, UTF-8 and duplicate keys.
async fn echo_query(RawQuery(raw): RawQuery) -> Result<Json<QueryEcho>, ApiError> {
    // Validate every parameter first. The ? operator returns ApiError immediately on failure.
    let params = parse_search_query(raw.as_deref())?;
    // Then call query::SearchParams::ensure_supported.
    // state::AppState currently contains names only, so ingredient filtering is unavailable.
    // Keep this false until a handler actually applies the validated ingredient conditions.
    params.ensure_supported(false)?;
    Ok(Json(QueryEcho { demo: true, params }))
}

// Report HTTP availability without checking data loading or external dependencies.
// Reject a nonempty raw query; a missing or empty query string contains no parameters.
async fn health(RawQuery(raw): RawQuery) -> Result<Json<HealthResponse>, ApiError> {
    if raw.as_deref().is_some_and(|query| !query.is_empty()) {
        return Err(ApiError::invalid("query", "此接口不接受查询参数。"));
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

// Called by main.rs to build routes with Arc<state::AppState>.
// main.rs subsequently applies http::with_middleware for CORS and request logging.
pub fn build_app(state: Arc<AppState>) -> Router {
    Router::new()
        // Register health and debug endpoints. Axum's get also supports HEAD without a body.
        // The public /api/recipes handler is not registered in this module yet.
        .route("/api/health", get(health).options(options))
        .route("/api/debug/query", get(echo_query).options(options))
        .route("/api/debug/state", get(inspect_state).options(options))
        // Use a JSON 404 response when no route matches.
        .fallback(not_found)
        // Install the JSON 405 handler after registering routes, so it applies to those routes.
        .method_not_allowed_fallback(method_not_allowed)
        // Provide the shared state required by inspect_state's State extractor.
        .with_state(state)
}

// Compile app_tests.rs only when tests are enabled.
#[cfg(test)]
#[path = "app_tests.rs"]
mod tests;
