//! HTTP request handlers for OIDC Hub

use crate::models::*;
use crate::oidc::OidcManager;
use axum::{extract::State, http::StatusCode, response::Json};
use chrono::Utc;
use std::sync::Arc;
use tracing::{error, info};

pub async fn health(State(manager): State<Arc<OidcManager>>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        providers: manager.get_providers().len(),
        timestamp: Utc::now(),
    })
}

pub async fn token_validate(
    State(manager): State<Arc<OidcManager>>,
    Json(payload): Json<ValidateRequest>,
) -> (StatusCode, Json<ValidateResponse>) {
    info!("Validating token for provider: {:?}", payload.provider);

    match manager
        .validate_token(&payload.token, payload.provider.as_deref())
        .await
    {
        Ok(claims) => {
            let expires_at = claims
                .get("exp")
                .and_then(|v| v.as_i64())
                .and_then(|exp| chrono::DateTime::from_timestamp(exp, 0));

            (
                StatusCode::OK,
                Json(ValidateResponse {
                    valid: true,
                    claims: Some(claims),
                    expires_at,
                    provider: payload.provider,
                    error: None,
                }),
            )
        }
        Err(e) => {
            // Log the full error chain server-side but return a generic message to clients
            // to avoid leaking internal details (JWKS URIs, network errors, key info).
            error!("Token validation failed: {:#}", e);
            (
                StatusCode::UNAUTHORIZED,
                Json(ValidateResponse {
                    valid: false,
                    claims: None,
                    expires_at: None,
                    provider: payload.provider,
                    error: Some("Token validation failed".to_string()),
                }),
            )
        }
    }
}

pub async fn token_exchange(
    State(_manager): State<Arc<OidcManager>>,
    Json(payload): Json<ExchangeRequest>,
) -> (StatusCode, Json<ExchangeResponse>) {
    info!(
        "Token exchange requested from {} to {} (not yet implemented)",
        payload.source_provider, payload.target_provider
    );

    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ExchangeResponse {
            token: None,
            expires_in: None,
            error: Some("Token exchange is not yet implemented".to_string()),
        }),
    )
}

pub async fn list_providers(State(manager): State<Arc<OidcManager>>) -> Json<ProvidersResponse> {
    let providers = manager
        .get_providers()
        .iter()
        .map(|p| ProviderInfo {
            name: p.name.clone(),
            issuer: p.issuer.clone(),
            audience: p.audience.clone(),
        })
        .collect();

    Json(ProvidersResponse { providers })
}
