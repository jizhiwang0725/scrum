use axum::{
    Json,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub const ALLOWED_METHODS: &str = "GET, HEAD, OPTIONS";

#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub code: &'static str,
    pub message: &'static str,
    pub details: Vec<ErrorDetail>,
}

#[derive(Serialize)]
struct ErrorEnvelope {
    error: ErrorBody,
}

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub body: ErrorBody,
}

impl ApiError {
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

    pub fn invalid(field: &str, detail: &str) -> Self {
        Self::parameter("INVALID_QUERY", "查询参数不合法。", field, detail)
    }

    pub fn unsupported(field: &str, detail: &str) -> Self {
        Self::parameter(
            "UNSUPPORTED_PARAMETER",
            "当前阶段暂不支持该参数。",
            field,
            detail,
        )
    }

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

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let mut response = (self.status, Json(ErrorEnvelope { error: self.body })).into_response();

        if self.status == StatusCode::METHOD_NOT_ALLOWED {
            response.headers_mut().insert(
                header::ALLOW,
                ALLOWED_METHODS.parse().expect("constant header"),
            );
        }

        response
    }
}
