//! Shared HTTP errors used by app.rs handlers and query.rs validation.
use axum::{
    Json,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::Serialize;

// Methods exposed by app::build_app. Include this value in every 405 Allow header.
pub const ALLOWED_METHODS: &str = "GET, HEAD, OPTIONS";

#[derive(Debug, Serialize)]
// Identifies an invalid parameter and explains the problem.
// Use field "query" for malformed URL encoding or invalid UTF-8.
pub struct ErrorDetail {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
// The contents of the JSON error object defined in docs/api.md.
// 400 responses contain at least one detail; 404 and 405 responses use an empty list.
pub struct ErrorBody {
    pub code: &'static str,
    pub message: &'static str,
    pub details: Vec<ErrorDetail>,
}

#[derive(Serialize)]
// Wrap ErrorBody so the response has the shape { "error": { ... } }.
struct ErrorEnvelope {
    error: ErrorBody,
}

#[derive(Debug)]
// Keep the HTTP status separate from the serializable error body.
// app.rs handlers return this type; Axum calls its IntoResponse implementation.
pub struct ApiError {
    pub status: StatusCode,
    pub body: ErrorBody,
}

impl ApiError {
    // Build a 400 error with one field-specific detail.
    // Copy borrowed text into owned Strings so the response owns its messages.
    fn parameter(code: &'static str, message: &'static str, field: &str, detail: &str) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            body: ErrorBody {
                code,
                message,
                details: vec![ErrorDetail {
                    field: field.to_owned(),
                    message: detail.to_owned(),
                }],
            },
        }
    }

    // Called by query::parse_search_query and app::health for invalid input.
    pub fn invalid(field: &str, detail: &str) -> Self {
        Self::parameter("INVALID_QUERY", "查询参数不合法。", field, detail)
    }

    // Called by query::SearchParams::ensure_supported for valid but unavailable features.
    pub fn unsupported(field: &str, detail: &str) -> Self {
        Self::parameter(
            "UNSUPPORTED_PARAMETER",
            "当前阶段暂不支持该参数。",
            field,
            detail,
        )
    }

    // Used by app::not_found when no route matches.
    // A successful recipe search with no matches must return 200 with an empty list instead.
    pub fn not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            body: ErrorBody {
                code: "NOT_FOUND",
                message: "接口不存在。",
                details: vec![],
            },
        }
    }

    // Used by app::method_not_allowed when a route exists but rejects the HTTP method.
    pub fn method_not_allowed() -> Self {
        Self {
            status: StatusCode::METHOD_NOT_ALLOWED,
            body: ErrorBody {
                code: "METHOD_NOT_ALLOWED",
                message: "请求方法不受支持。",
                details: vec![],
            },
        }
    }
}

// Allow app.rs handlers to return Result<..., ApiError> directly.
impl IntoResponse for ApiError {
    // Consume the error and move its body into the JSON envelope.
    // Json serializes the body and sets Content-Type to application/json.
    fn into_response(self) -> Response {
        let mut response = (self.status, Json(ErrorEnvelope { error: self.body })).into_response();

        // Add the Allow header required for 405 responses.
        if self.status == StatusCode::METHOD_NOT_ALLOWED {
            response.headers_mut().insert(
                header::ALLOW,
                ALLOWED_METHODS.parse().expect("constant header"),
            );
        }

        response
    }
}
