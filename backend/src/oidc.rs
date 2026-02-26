//! OIDC token validation and management

use crate::config::{Config, OidcProvider};
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use jsonwebtoken::{decode, decode_header, jwk::JwkSet, Algorithm, DecodingKey, Validation};
use serde_json::Value;
use std::collections::HashMap;
use tracing::{debug, warn};

pub struct OidcManager {
    config: Config,
}

impl OidcManager {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn get_providers(&self) -> &[OidcProvider] {
        &self.config.providers
    }

    pub async fn validate_token(
        &self,
        token: &str,
        provider_name: Option<&str>,
    ) -> Result<HashMap<String, Value>> {
        // Find matching provider
        let provider = if let Some(name) = provider_name {
            self.config
                .providers
                .iter()
                .find(|p| p.name == name)
                .ok_or_else(|| anyhow!("Provider not found: {}", name))?
        } else {
            // Try to auto-detect provider from token claims
            self.detect_provider(token)?
        };

        debug!("Validating token with provider: {}", provider.name);

        // Fetch JWKS (JSON Web Key Set) from provider
        let jwks = self.fetch_jwks(&provider.jwks_uri).await?;

        // SECURITY: Pin algorithm to provider config — NEVER trust the token's own "alg" header.
        // Trusting token-declared algorithm enables algorithm confusion attacks where an
        // attacker crafts a token with "alg":"HS256" and signs with the RSA public key.
        let algorithm = parse_algorithm(&provider.algorithm)?;
        let mut validation = Validation::new(algorithm);
        validation.set_audience(&[&provider.audience]);
        validation.set_issuer(&[&provider.issuer]);

        // Match the token's kid header to the correct JWK from the JWKS
        let header = decode_header(token)?;
        let decoding_key = find_decoding_key(&jwks, header.kid.as_deref())?;
        let token_data = decode::<HashMap<String, Value>>(token, &decoding_key, &validation)?;

        Ok(token_data.claims)
    }

    async fn fetch_jwks(&self, jwks_uri: &str) -> Result<JwkSet> {
        // TODO: Cache JWKS with TTL to avoid fetching on every validation
        let response = reqwest::get(jwks_uri).await?;
        let jwks: JwkSet = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse JWKS from {}: {}", jwks_uri, e))?;
        Ok(jwks)
    }

    fn detect_provider(&self, token: &str) -> Result<&OidcProvider> {
        // Decode without validation to peek at claims
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(anyhow!("Invalid token format"));
        }

        // JWTs use base64url encoding (RFC 7515), not standard base64
        let payload = general_purpose::URL_SAFE_NO_PAD.decode(parts[1])?;
        let claims: HashMap<String, Value> = serde_json::from_slice(&payload)?;

        // Match issuer to provider
        if let Some(iss) = claims.get("iss").and_then(|v| v.as_str()) {
            for provider in &self.config.providers {
                if provider.issuer == iss {
                    return Ok(provider);
                }
            }
        }

        Err(anyhow!("Could not detect provider from token"))
    }

    #[allow(dead_code)]
    pub async fn exchange_token(
        &self,
        _source_token: &str,
        _source_provider: &str,
        _target_provider: &str,
    ) -> Result<String> {
        // Token exchange logic would go here
        // This is a placeholder for future implementation
        warn!("Token exchange not yet implemented");
        Err(anyhow!("Token exchange not implemented"))
    }
}

/// Find the correct decoding key from a JWKS by matching the token's `kid` header.
/// If no `kid` is present and the JWKS has exactly one key, use that key.
fn find_decoding_key(jwks: &JwkSet, kid: Option<&str>) -> Result<DecodingKey> {
    let jwk = if let Some(kid) = kid {
        jwks.find(kid)
            .ok_or_else(|| anyhow!("No JWK found for kid: {}", kid))?
    } else if jwks.keys.len() == 1 {
        &jwks.keys[0]
    } else {
        return Err(anyhow!(
            "Token has no kid and JWKS has {} keys — cannot determine which key to use",
            jwks.keys.len()
        ));
    };

    DecodingKey::from_jwk(jwk).map_err(|e| anyhow!("Failed to create decoding key from JWK: {}", e))
}

/// Parse algorithm string from config into jsonwebtoken Algorithm enum.
/// Only allows asymmetric algorithms suitable for OIDC (RSA, EC).
fn parse_algorithm(alg: &str) -> Result<Algorithm> {
    match alg {
        "RS256" => Ok(Algorithm::RS256),
        "RS384" => Ok(Algorithm::RS384),
        "RS512" => Ok(Algorithm::RS512),
        "ES256" => Ok(Algorithm::ES256),
        "ES384" => Ok(Algorithm::ES384),
        "PS256" => Ok(Algorithm::PS256),
        "PS384" => Ok(Algorithm::PS384),
        "PS512" => Ok(Algorithm::PS512),
        // SECURITY: Explicitly reject HMAC algorithms — they must never be used with
        // asymmetric OIDC providers as it enables algorithm confusion attacks.
        "HS256" | "HS384" | "HS512" => Err(anyhow!(
            "HMAC algorithms are not allowed for OIDC providers"
        )),
        _ => Err(anyhow!("Unsupported algorithm: {}", alg)),
    }
}
