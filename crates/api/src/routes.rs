//! Route definitions and Axum router construction.

use axum::{Router, routing::{get, post}};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::handlers;

/// API version prefix.
pub const API_PREFIX: &str = "/api/v1";

/// Build the complete API router.
pub fn build_router() -> Router {
    let api_routes = Router::new()
        .route("/health", get(handlers::health))
        .route("/evaluate", post(handlers::evaluate))
        .route("/applicability", post(handlers::applicability))
        .route("/sanctions", post(handlers::sanctions))
        .route("/obligations", post(handlers::obligations))
        .route("/report", post(handlers::report));

    Router::new()
        .nest(API_PREFIX, api_routes)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}
