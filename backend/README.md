# AI Agent OIDC Hub

**Auto-OIDC Hub** - Distributed trust and identity management for service-to-service authentication.

## Overview

The OIDC Hub provides centralized OIDC token validation, exchange, and management for the Lornu AI platform. It enables secure service-to-service authentication using industry-standard OIDC protocols.

## Features

- **Token Validation**: Validate OIDC tokens from multiple providers (GitHub, Google, etc.)
- **Provider Management**: Centralized configuration of OIDC providers
- **Token Exchange**: Exchange tokens between different providers (future)
- **Caching**: Efficient JWKS caching for performance
- **Cloud Run Ready**: Optimized for serverless deployment

## Architecture

### Technology Stack
- **Backend**: Rust + Axum
- **Authentication**: jsonwebtoken
- **Build System**: Dagger + Bun (optimized Rust caching)
- **Deployment**: Google Cloud Run

### Build Pipeline
This project uses **Dagger with Bun** for CI/CD, providing:
- 60-80% faster Rust builds via persistent cache volumes
- Local-to-cloud parity (same pipeline everywhere)
- Programmable TypeScript-based builds

## API Endpoints

### Health Check
```bash
GET /health

Response:
{
  "status": "healthy",
  "providers": 2,
  "timestamp": "2026-01-20T10:00:00Z"
}
```

### Token Validation
```bash
POST /token/validate
Content-Type: application/json

{
  "token": "eyJhbGciOiJSUzI1NiIs...",
  "provider": "github"
}

Response:
{
  "valid": true,
  "claims": {
    "iss": "https://token.actions.githubusercontent.com",
    "sub": "repo:lornu-ai/lornu.ai:ref:refs/heads/main",
    "aud": "https://github.com/lornu-ai",
    "exp": 1737379200
  },
  "expires_at": "2026-01-20T11:00:00Z",
  "provider": "github"
}
```

### List Providers
```bash
GET /providers

Response:
{
  "providers": [
    {
      "name": "github",
      "issuer": "https://token.actions.githubusercontent.com",
      "audience": "https://github.com/lornu-ai"
    },
    {
      "name": "google",
      "issuer": "https://accounts.google.com",
      "audience": "lornu-ai"
    }
  ]
}
```

## Development

### Prerequisites
- Rust 1.81+
- Bun (for Dagger builds)
- Dagger CLI

### Local Development
```bash
# Run locally
cargo run

# Run tests
cargo test

# Build with Dagger (optimized caching)
bun install
bun run build
```

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `8080` | HTTP server port |
| `OIDC_PROVIDERS` | (see config.rs) | JSON array of provider configurations |
| `CACHE_TTL` | `300` | JWKS cache TTL in seconds |
| `RUST_LOG` | `info` | Logging level |

### Provider Configuration

Providers can be configured via the `OIDC_PROVIDERS` environment variable:

```json
[
  {
    "name": "github",
    "issuer": "https://token.actions.githubusercontent.com",
    "audience": "https://github.com/lornu-ai",
    "jwks_uri": "https://token.actions.githubusercontent.com/.well-known/jwks"
  }
]
```

## Deployment

### Build with Dagger
```bash
# Build only
bun run build

# Build and publish to GAR
GCP_PROJECT=lornu-ai \
GCP_REGION=us-central1 \
GAR_REPO=ai-agents \
IMAGE_TAG=v1.0.0 \
PUBLISH=true \
bun run build:publish
```

### Deploy to Cloud Run
```bash
gcloud run deploy oidc-hub \
  --image us-central1-docker.pkg.dev/lornu-ai/ai-agents/oidc-hub:latest \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated \
  --set-env-vars PORT=8080
```

## Dagger Build Advantages

### Why Dagger + Bun?
1. **Persistent Caching**: Cargo registry and target directory cached across builds
2. **60-80% Faster**: Rust compilation time dramatically reduced
3. **Local Parity**: `dagger run` works identically on CI and local
4. **Programmable**: Full TypeScript control vs YAML limitations

### Cache Volumes
The Dagger build uses two critical cache volumes:
- `cargo-registry`: Caches downloaded crates
- `oidc-hub-target`: Caches compiled artifacts

This means after the first build, subsequent builds only recompile changed code.

## Integration

### With Other Services
```rust
// Example: Validate token in another service
let client = reqwest::Client::new();
let response = client
    .post("http://oidc-hub:8080/token/validate")
    .json(&serde_json::json!({
        "token": github_token,
        "provider": "github"
    }))
    .send()
    .await?;
```

## Security

- Tokens are validated using provider JWKS
- Signature verification with RSA
- Expiration checking
- Audience and issuer validation
- No secrets stored in code

## Related Issues

- Issue #199: Auto-OIDC Hub and Dockworker.ai integration

## License

Proprietary - Lornu AI
