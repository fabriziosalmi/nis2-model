//! Route definitions and Axum router construction.

use axum::http::{HeaderValue, Method, header};
use axum::{Router, middleware, routing::{get, post}};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::handlers;

/// API version prefix.
pub const API_PREFIX: &str = "/api/v1";

/// Security headers middleware — adds protective response headers.
async fn security_headers(
    request: axum::extract::Request,
    next: middleware::Next,
) -> axum::response::Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert("X-Content-Type-Options", HeaderValue::from_static("nosniff"));
    headers.insert("X-Frame-Options", HeaderValue::from_static("DENY"));
    headers.insert("Referrer-Policy", HeaderValue::from_static("strict-origin-when-cross-origin"));
    headers.insert("X-Permitted-Cross-Domain-Policies", HeaderValue::from_static("none"));
    response
}

/// Build the complete API router.
///
/// CORS policy: restricted by default (same-origin only).
/// Set the `CORS_ORIGIN` environment variable to allow a specific external origin
/// (e.g. `CORS_ORIGIN=https://your-app.example.com`).
pub fn build_router() -> Router {
    let api_routes = Router::new()
        .route("/health", get(handlers::health))
        .route("/evaluate", post(handlers::evaluate))
        .route("/applicability", post(handlers::applicability))
        .route("/sanctions", post(handlers::sanctions))
        .route("/obligations", post(handlers::obligations))
        .route("/report", post(handlers::report));

    // SECURITY: CORS restricted by default. Set CORS_ORIGIN env var for cross-origin access.
    let cors = match std::env::var("CORS_ORIGIN") {
        Ok(origin) => CorsLayer::new()
            .allow_origin(origin.parse::<HeaderValue>().expect("Invalid CORS_ORIGIN value"))
            .allow_methods([Method::GET, Method::POST])
            .allow_headers([header::CONTENT_TYPE]),
        Err(_) => CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_headers([header::CONTENT_TYPE]),
    };

    Router::new()
        .nest(API_PREFIX, api_routes)
        .layer(middleware::from_fn(security_headers))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
}
