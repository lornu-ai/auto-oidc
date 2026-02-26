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
            error!("Token validation failed: {}", e);
            (
                StatusCode::UNAUTHORIZED,
                Json(ValidateResponse {
                    valid: false,
                    claims: None,
                    expires_at: None,
                    provider: payload.provider,
                    error: Some(e.to_string()),
                }),
            )
        }
    }
}

pub async fn token_exchange(
    State(manager): State<Arc<OidcManager>>,
    Json(payload): Json<ExchangeRequest>,
) -> (StatusCode, Json<ExchangeResponse>) {
    info!(
        "Exchanging token from {} to {}",
        payload.source_provider, payload.target_provider
    );

    match manager
        .exchange_token(
            &payload.source_token,
            &payload.source_provider,
            &payload.target_provider,
        )
        .await
    {
        Ok(token) => (
            StatusCode::OK,
            Json(ExchangeResponse {
                token: Some(token),
                expires_in: Some(3600),
                error: None,
            }),
        ),
        Err(e) => {
            error!("Token exchange failed: {}", e);
            (
                StatusCode::BAD_REQUEST,
                Json(ExchangeResponse {
                    token: None,
                    expires_in: None,
                    error: Some(e.to_string()),
                }),
            )
        }
    }
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
