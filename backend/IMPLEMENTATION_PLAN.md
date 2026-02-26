# Auto-OIDC Hub Implementation Plan

## Overview
Auto-OIDC Hub is a distributed trust and identity management service for service-to-service authentication in the Lornu AI platform.

## Architecture

### Components
1. **OIDC Manager** - Core OIDC token validation and management
2. **Provider Registry** - Manages configured OIDC providers (GitHub, Google, etc.)
3. **Token Exchange** - Exchanges tokens between different providers
4. **Validation Service** - Validates OIDC tokens against configured providers

### Technology Stack
- **Backend**: Rust + Axum
- **Authentication**: jsonwebtoken crate
- **Deployment**: Google Cloud Run
- **Storage**: In-memory cache + optional Redis

## API Endpoints

### Health Check
```
GET /health
Response: { "status": "healthy", "providers": 3 }
```

### Token Validation
```
POST /token/validate
Request: { "token": "eyJ...", "provider": "github" }
Response: { "valid": true, "claims": {...}, "expires_at": "..." }
```

### Token Exchange
```
POST /token/exchange
Request: { "source_token": "eyJ...", "target_provider": "google" }
Response: { "token": "eyJ...", "expires_in": 3600 }
```

### List Providers
```
GET /providers
Response: { "providers": ["github", "google", "gitlab"] }
```

## Configuration

### Environment Variables
- `PORT` - HTTP server port (default: 8080)
- `OIDC_PROVIDERS` - JSON array of provider configurations
- `CACHE_TTL` - Token cache TTL in seconds (default: 300)
- `RUST_LOG` - Logging level

### Provider Configuration Format
```json
{
  "name": "github",
  "issuer": "https://token.actions.githubusercontent.com",
  "audience": "https://github.com/lornu-ai",
  "jwks_uri": "https://token.actions.githubusercontent.com/.well-known/jwks"
}
```

## Implementation Status

### Completed
- [x] Project structure
- [x] Cargo.toml with dependencies
- [x] Main.rs skeleton
- [x] Config module
- [x] Models (Token, Provider, Claims)
- [x] OIDC Manager
- [x] HTTP Handlers
- [x] Dockerfile
- [x] README
- [x] JWKS JSON parsing (PR #2147)
- [x] Base64url decoding fix (PR #2147)
- [x] Algorithm confusion prevention
- [x] HMAC algorithm rejection

### TODO
- [ ] OpenAPI spec
- [ ] Unit tests
- [ ] Integration tests
- [ ] Deployment manifests

## Integration Points

### With Existing Services
- **ai-agent-auth-manager**: Uses OIDC Hub for token validation
- **Vault**: Stores provider secrets
- **Cloud Run**: Deployment target

### Future Integrations
- **Dockworker.ai**: Will use OIDC Hub for authentication
- **Other AI Agents**: Can validate service tokens

## Security Considerations

1. **Token Validation**
   - Verify signature using provider's JWKS
   - Check expiration
   - Validate audience and issuer

2. **Rate Limiting**
   - Implement per-provider rate limits
   - Prevent token flooding attacks

3. **Secrets Management**
   - Provider secrets stored in Vault
   - No hardcoded credentials

## Deployment

### Cloud Run Configuration
```yaml
apiVersion: serving.knative.dev/v1
kind: Service
metadata:
  name: oidc-hub
spec:
  template:
    spec:
      containers:
      - image: gcr.io/lornu-ai/oidc-hub:latest
        env:
        - name: PORT
          value: "8080"
        - name: OIDC_PROVIDERS
          valueFrom:
            secretKeyRef:
              name: oidc-providers
              key: config
```
