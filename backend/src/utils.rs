/// Utility functions for the application

use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

/// Create CORS layer for the application
pub fn create_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any)
}

/// Setup tracing for the application
pub fn setup_tracing() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
}
