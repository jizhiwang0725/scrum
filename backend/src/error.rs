//! 统一 HTTP 错误：构造错误信息，并按接口文档返回嵌套的 JSON。
use axum::{
    Json,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::Serialize;

// 已知接口允许的方法；返回 405 时，通过 Allow 响应头告诉客户端。
pub const ALLOWED_METHODS: &str = "GET, HEAD, OPTIONS";

#[derive(Debug, Serialize)]
// 一条具体错误：field 指向出错参数，message 说明该参数的问题。
// 整段 URL 编码错误使用 field = "query"。
pub struct ErrorDetail {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
// 错误主体：code 供前端判断类型，message 用于展示，details 提供具体原因。
// 400 至少包含一条 details；404、405 的 details 为空数组。
pub struct ErrorBody {
    pub code: &'static str,
    pub message: &'static str,
    pub details: Vec<ErrorDetail>,
}

#[derive(Serialize)]
// 外层包装，使响应符合 { "error": { "code": ..., "message": ..., "details": [...] } }。
struct ErrorEnvelope {
    error: ErrorBody,
}

#[derive(Debug)]
// 程序内部使用的错误类型：HTTP 状态码与 JSON 错误内容分开保存。
// handler 返回 Err(ApiError) 时，由下面的 IntoResponse 完成响应转换。
pub struct ApiError {
    pub status: StatusCode,
    pub body: ErrorBody,
}

impl ApiError {
    // 集中构造参数类错误，保证状态为 400，并且 details 至少有一项。
    // 传入的 &str 在这里转成 String，错误对象可以独立持有这些文字。
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

    // 参数本身不合法：例如未知参数、重复键、超长或错误编码。
    pub fn invalid(field: &str, detail: &str) -> Self {
        Self::parameter("INVALID_QUERY", "查询参数不合法。", field, detail)
    }

    // 参数内容合法，但对应功能尚未接入：例如耗时排序、饮食标签。
    pub fn unsupported(field: &str, detail: &str) -> Self {
        Self::parameter(
            "UNSUPPORTED_PARAMETER",
            "当前阶段暂不支持该参数。",
            field,
            detail,
        )
    }

    // 路径不存在时使用；搜索无结果应返回 200 和空列表，不能使用此错误。
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

    // 路径存在但 HTTP 方法不支持时使用，例如对 health 接口发送 POST。
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

// 实现 Axum 的响应转换约定，让 ApiError 能直接作为 handler 的错误返回值。
impl IntoResponse for ApiError {
    // self 按值传入：转换响应时消耗这个错误对象，将 body 移入 JSON 包装。
    // Json 负责序列化，并设置 Content-Type: application/json。
    fn into_response(self) -> Response {
        let mut response = (self.status, Json(ErrorEnvelope { error: self.body })).into_response();

        // 405 除了 JSON 错误正文，还必须带上 Allow 响应头。
        if self.status == StatusCode::METHOD_NOT_ALLOWED {
            response.headers_mut().insert(
                header::ALLOW,
                ALLOWED_METHODS.parse().expect("constant header"),
            );
        }

        response
    }
}
