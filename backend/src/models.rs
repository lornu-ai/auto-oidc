//! Data models for OIDC Hub

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub providers: usize,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ValidateRequest {
    pub token: String,
    pub provider: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ValidateResponse {
    pub valid: bool,
    pub claims: Option<HashMap<String, serde_json::Value>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub provider: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExchangeRequest {
    pub source_token: String,
    pub source_provider: String,
    pub target_provider: String,
}

#[derive(Debug, Serialize)]
pub struct ExchangeResponse {
    pub token: Option<String>,
    pub expires_in: Option<u64>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProvidersResponse {
    pub providers: Vec<ProviderInfo>,
}

#[derive(Debug, Serialize)]
pub struct ProviderInfo {
    pub name: String,
    pub issuer: String,
    pub audience: String,
}
