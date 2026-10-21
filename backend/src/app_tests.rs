use super::*;
use axum::{
    body::{Body, to_bytes},
    http::{Method, Request},
    response::Response,
};
use serde_json::{Value, json};
use tower::ServiceExt;

async fn request(method: Method, uri: &str, headers: &[(&str, &str)]) -> Response {
    let state = Arc::new(AppState {
        recipe_names: vec!["番茄炒蛋".to_owned(), "土豆炖牛肉".to_owned()],
    });
    // Test with the team's actual CORS middleware, not only the bare Router.
    let app = crate::http::with_middleware(
        build_app(state),
        vec!["http://localhost:5173".parse().unwrap()],
    );
    let mut builder = Request::builder().method(method).uri(uri);
    for &(key, value) in headers {
        builder = builder.header(key, value);
    }
    app.oneshot(builder.body(Body::empty()).unwrap())
        .await
        .unwrap()
}

async fn json_body(response: Response) -> Value {
    assert_eq!(response.headers()[header::CONTENT_TYPE], "application/json");
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn health_succeeds_and_rejects_query_parameters() {
    let response = request(Method::GET, "/api/health", &[]).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(json_body(response).await, json!({"status": "ok"}));

    let response = request(Method::GET, "/api/health?q=", &[]).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = json_body(response).await;
    assert_eq!(body["error"]["code"], "INVALID_QUERY");
    assert_eq!(body["error"]["details"][0]["field"], "query");
}

#[tokio::test]
async fn debug_route_exposes_validated_parameters_without_claiming_to_search() {
    let response = request(Method::GET, "/api/debug/query?q=+TOMATO++Soup+", &[]).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        json_body(response).await,
        json!({
            "demo": true,
            "params": {"q": "tomato soup", "ingredients": [], "sort": "relevance", "dietary_tags": []}
        })
    );
}

#[tokio::test]
async fn empty_q_is_valid_over_http() {
    let response = request(Method::GET, "/api/debug/query?q=", &[]).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(json_body(response).await["params"]["q"], "");
}

#[tokio::test]
async fn malformed_duplicate_and_unknown_queries_use_the_error_envelope() {
    for (uri, field) in [
        ("/api/debug/query?q=a&q=b", "q"),
        ("/api/debug/query?q=a&%71=b", "q"),
        ("/api/debug/query?unknown=x", "unknown"),
        ("/api/debug/query?q=%GG", "query"),
        ("/api/debug/query?q=%FF", "query"),
        ("/api/debug/query?sort=", "sort"),
    ] {
        let response = request(Method::GET, uri, &[]).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST, "{uri}");
        let body = json_body(response).await;
        assert_eq!(body["error"]["code"], "INVALID_QUERY", "{uri}");
        assert_eq!(body["error"]["details"][0]["field"], field);
        assert!(body["error"]["message"].is_string());
    }
}

#[tokio::test]
async fn unavailable_features_are_rejected_after_all_validation() {
    for uri in [
        "/api/debug/query?sort=time_asc",
        "/api/debug/query?ingredients=salt",
        "/api/debug/query?dietary_tags=vegan",
    ] {
        let response = request(Method::GET, uri, &[]).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            json_body(response).await["error"]["code"],
            "UNSUPPORTED_PARAMETER"
        );
    }
    let response = request(
        Method::GET,
        "/api/debug/query?sort=time_asc&dietary_tags=unknown",
        &[],
    )
    .await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(json_body(response).await["error"]["code"], "INVALID_QUERY");
}

#[tokio::test]
async fn unknown_api_path_and_unsupported_method_use_json() {
    let response = request(Method::GET, "/api/does-not-exist", &[]).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = json_body(response).await;
    assert_eq!(body["error"]["code"], "NOT_FOUND");
    assert_eq!(body["error"]["details"], json!([]));

    let response = request(Method::POST, "/api/health", &[]).await;
    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(response.headers()[header::ALLOW], ALLOWED_METHODS);
    let body = json_body(response).await;
    assert_eq!(body["error"]["code"], "METHOD_NOT_ALLOWED");
    assert_eq!(body["error"]["details"], json!([]));
}

#[tokio::test]
async fn head_matches_get_status_and_headers_without_a_body() {
    for uri in [
        "/api/health",
        "/api/health?q=x",
        "/api/debug/query?q=",
        "/api/debug/query?sort=bad",
    ] {
        let get = request(Method::GET, uri, &[]).await;
        let head = request(Method::HEAD, uri, &[]).await;
        assert_eq!(head.status(), get.status(), "{uri}");
        assert_eq!(
            head.headers()[header::CONTENT_TYPE],
            get.headers()[header::CONTENT_TYPE]
        );
        assert_eq!(
            head.headers().get(header::CONTENT_LENGTH),
            get.headers().get(header::CONTENT_LENGTH)
        );
        assert!(to_bytes(head.into_body(), 1024).await.unwrap().is_empty());
    }
}

#[tokio::test]
async fn options_and_cors_preflight_have_no_body() {
    let response = request(Method::OPTIONS, "/api/health", &[]).await;
    // CorsLayer may answer ordinary OPTIONS directly with 200; the contract
    // requires 204 specifically for successful CORS preflights.
    assert!(response.status().is_success());
    assert!(
        to_bytes(response.into_body(), 1024)
            .await
            .unwrap()
            .is_empty()
    );

    let response = request(
        Method::OPTIONS,
        "/api/debug/query",
        &[
            ("origin", "http://localhost:5173"),
            ("access-control-request-method", "GET"),
            ("access-control-request-headers", "accept"),
        ],
    )
    .await;
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    assert_eq!(
        response.headers()[header::ACCESS_CONTROL_ALLOW_ORIGIN],
        "http://localhost:5173"
    );
    assert!(
        to_bytes(response.into_body(), 1024)
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn cors_headers_are_present_on_parameter_errors() {
    let response = request(
        Method::GET,
        "/api/debug/query?sort=bad",
        &[("origin", "http://localhost:5173")],
    )
    .await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        response.headers()[header::ACCESS_CONTROL_ALLOW_ORIGIN],
        "http://localhost:5173"
    );
    assert_eq!(json_body(response).await["error"]["code"], "INVALID_QUERY");
}

#[tokio::test]
async fn shared_state_debug_route_still_works() {
    let response = request(Method::GET, "/api/debug/state", &[]).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(json_body(response).await, json!({"recipe_count": 2}));
}
