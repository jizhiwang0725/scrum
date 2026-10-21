//! 组装 HTTP 路由：接收请求、调用 B 的校验模块，并返回 JSON。
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
// 共享状态调试响应，仅返回菜名数量，不返回完整菜谱数据。
struct StateSummary {
    recipe_count: usize,
}

// State 提取应用共享状态；Arc 让多个请求共享同一份数据的所有权。
// 这里仅只读访问，因此不需要为读取菜名数量额外加锁。
async fn inspect_state(State(state): State<Arc<AppState>>) -> Json<StateSummary> {
    Json(StateSummary {
        recipe_count: state.recipe_names.len(),
    })
}

#[derive(Serialize)]
// 探活接口的成功响应，序列化后为 { "status": "ok" }。
struct HealthResponse {
    status: &'static str,
}

/// Debug-only response: not the public RecipeList response.
#[derive(Serialize)]
// 调试接口的成功响应：demo 明确标记为回显结果。
// params 是校验和标准化后的参数，不是正式 /api/recipes 的菜谱列表。
struct QueryEcho {
    demo: bool,
    params: SearchParams,
}

// RawQuery 保留 URL 中问号后的原始查询字符串。
// 必须交给严格解析器检查，才能识别重复键、非法百分号编码和无效 UTF-8。
async fn echo_query(RawQuery(raw): RawQuery) -> Result<Json<QueryEcho>, ApiError> {
    // 先完整解析、校验全部参数；? 遇到错误会立即返回 Err，由 ApiError 转成 JSON。
    let params = parse_search_query(raw.as_deref())?;
    // Current state contains recipe names only; ingredient search is not wired yet.
    // 全部校验通过后，再检查功能是否支持。
    // false 表示当前尚未接入食材搜索；合法的非空食材条件也不能被悄悄忽略。
    params.ensure_supported(false)?;
    Ok(Json(QueryEcho { demo: true, params }))
}

// 探活只检查 HTTP 服务可访问，不检查数据加载或数据库。
// 按文档拒绝查询参数；空的原始查询字符串视为未传参数。
async fn health(RawQuery(raw): RawQuery) -> Result<Json<HealthResponse>, ApiError> {
    if raw.as_deref().is_some_and(|query| !query.is_empty()) {
        return Err(ApiError::invalid("query", "此接口不接受查询参数。"));
    }
    Ok(Json(HealthResponse { status: "ok" }))
}

// 路由层对 OPTIONS 返回 204，无正文，并告知允许的方法。
// 外层 CorsLayer 可能先处理 OPTIONS；跨域预检由现有 http.rs 中间件配合处理。
async fn options() -> impl axum::response::IntoResponse {
    (StatusCode::NO_CONTENT, [(header::ALLOW, ALLOWED_METHODS)])
}

// 路由没有匹配到路径时，使用统一的 404 JSON 错误。
async fn not_found() -> ApiError {
    ApiError::not_found()
}

// 匹配到路径但方法不受支持时，使用统一的 405 JSON 错误。
async fn method_not_allowed() -> ApiError {
    ApiError::method_not_allowed()
}

// A 在 main.rs 中调用此函数，得到带有共享状态的路由。
// get 同时支持对应 HEAD 请求：状态和响应头一致，但不返回正文。
pub fn build_app(state: Arc<AppState>) -> Router {
    Router::new()
        // 注册探活及两个调试接口；正式 /api/recipes 仍需 A、C 与 #28 接入。
        .route("/api/health", get(health).options(options))
        .route("/api/debug/query", get(echo_query).options(options))
        .route("/api/debug/state", get(inspect_state).options(options))
        // 路径不存在时执行 fallback。
        .fallback(not_found)
        // 为上面已经注册的路径设置方法错误处理，因此要放在 route 调用之后。
        .method_not_allowed_fallback(method_not_allowed)
        // 把共享状态提供给需要 State 提取器的 handler。
        .with_state(state)
}

// 仅在 cargo test 等启用测试的编译中加载 HTTP 测试模块。
#[cfg(test)]
#[path = "app_tests.rs"]
mod tests;
