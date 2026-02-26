//! AI Agent OIDC Hub - Distributed Trust and Identity Management
//!
//! This service provides OIDC token validation, exchange, and management
//! for service-to-service authentication in a distributed system.

mod config;
mod handlers;
mod models;
mod oidc;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::{
    cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;

use config::Config;
use handlers::{health, list_providers, token_exchange, token_validate};
use oidc::OidcManager;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .init();

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");
    info!("Starting OIDC Hub on port {}", config.port);

    // Initialize OIDC manager
    let oidc_manager = Arc::new(OidcManager::new(config.clone()));

    // Build router
    let app = Router::new()
        .route("/health", get(health))
        .route("/token/validate", post(token_validate))
        .route("/token/exchange", post(token_exchange))
        .route("/providers", get(list_providers))
        // SECURITY: Restrict CORS — permissive() allowed any origin to use this
        // service as a token validation oracle. Default to same-origin only;
        // configure ALLOWED_ORIGINS env var for specific trusted origins.
        .layer({
            let origins = std::env::var("ALLOWED_ORIGINS").unwrap_or_default();
            if origins.is_empty() {
                CorsLayer::new()
                    .allow_methods(AllowMethods::list([
                        axum::http::Method::GET,
                        axum::http::Method::POST,
                    ]))
                    .allow_headers(AllowHeaders::list([
                        axum::http::header::CONTENT_TYPE,
                        axum::http::header::AUTHORIZATION,
                    ]))
            } else {
                let allowed: Vec<axum::http::HeaderValue> = origins
                    .split(',')
                    .filter_map(|o| o.trim().parse().ok())
                    .collect();
                CorsLayer::new()
                    .allow_origin(AllowOrigin::list(allowed))
                    .allow_methods(AllowMethods::list([
                        axum::http::Method::GET,
                        axum::http::Method::POST,
                    ]))
                    .allow_headers(AllowHeaders::list([
                        axum::http::header::CONTENT_TYPE,
                        axum::http::header::AUTHORIZATION,
                    ]))
            }
        })
        .layer(TraceLayer::new_for_http())
        .with_state(oidc_manager);

    // Start server
    let addr = format!("0.0.0.0:{}", config.port);
    info!("OIDC Hub listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
