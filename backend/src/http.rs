use axum::{
    Router,
    body::Body,
    extract::Request,
    http::{HeaderValue, Method, StatusCode, header},
    middleware::{self, Next},
    response::Response,
};
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultOnResponse, TraceLayer},
};
use tracing::Level;

pub fn with_middleware(router: Router, origins: Vec<HeaderValue>) -> Router {
    // Let our chosen frontend pages read data from the backend.
    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::HEAD, Method::OPTIONS])
        .allow_headers([header::ACCEPT]);

    router
        .layer(cors)
        // Wrap the CORS layer so we can adjust its response afterwards.
        .layer(middleware::from_fn(preflight_no_content))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request| {
                    // Log the request method and path, but leave out the query parameters.
                    tracing::info_span!(
                        "http_request",
                        method = %request.method(),
                        path = %request.uri().path(),
                    )
                })
                .on_request(())
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
}

// When the browser checks CORS permissions, return 204 with an empty body.
// CORS accepts 200 too, but docs/API.md requires 204 with no body.
async fn preflight_no_content(request: Request, next: Next) -> Response {
    // Both headers are needed to distinguish a preflight from ordinary OPTIONS.
    let is_preflight = request.method() == Method::OPTIONS
        && request.headers().contains_key(header::ORIGIN)
        && request
            .headers()
            .contains_key(header::ACCESS_CONTROL_REQUEST_METHOD);

    // Let the CORS layer add its response headers first.
    let mut response = next.run(request).await;

    // Only convert 200 preflight responses; keep error responses unchanged.
    if is_preflight && response.status() == StatusCode::OK {
        *response.status_mut() = StatusCode::NO_CONTENT;

        // A 204 response cannot have a body. Keep the CORS headers.
        *response.body_mut() = Body::empty();
        response.headers_mut().remove(header::CONTENT_LENGTH);
        response.headers_mut().remove(header::CONTENT_TYPE);
    }

    response
}
