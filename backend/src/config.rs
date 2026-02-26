//! Configuration module for OIDC Hub

use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
    pub providers: Vec<OidcProvider>,
    pub cache_ttl: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OidcProvider {
    pub name: String,
    pub issuer: String,
    pub audience: String,
    pub jwks_uri: String,
    /// Expected JWT signing algorithm — MUST be pinned per-provider to prevent
    /// algorithm confusion attacks (e.g., attacker sends "alg":"HS256" with RSA public key).
    #[serde(default = "default_algorithm")]
    pub algorithm: String,
}

fn default_algorithm() -> String {
    "RS256".to_string()
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()?;

        let cache_ttl = env::var("CACHE_TTL")
            .unwrap_or_else(|_| "300".to_string())
            .parse()?;

        // Load providers from JSON env var or use defaults
        let providers = if let Ok(providers_json) = env::var("OIDC_PROVIDERS") {
            serde_json::from_str(&providers_json)?
        } else {
            Self::default_providers()
        };

        Ok(Config {
            port,
            providers,
            cache_ttl,
        })
    }

    fn default_providers() -> Vec<OidcProvider> {
        vec![
            OidcProvider {
                name: "github".to_string(),
                issuer: "https://token.actions.githubusercontent.com".to_string(),
                audience: "https://github.com/lornu-ai".to_string(),
                jwks_uri: "https://token.actions.githubusercontent.com/.well-known/jwks"
                    .to_string(),
                algorithm: "RS256".to_string(),
            },
            OidcProvider {
                name: "google".to_string(),
                issuer: "https://accounts.google.com".to_string(),
                audience: "lornu-ai".to_string(),
                jwks_uri: "https://www.googleapis.com/oauth2/v3/certs".to_string(),
                algorithm: "RS256".to_string(),
            },
        ]
    }
}
