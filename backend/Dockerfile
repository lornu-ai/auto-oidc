FROM rust:1.81-slim AS builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml ./

# Create dummy main to cache dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Build dependencies only
RUN cargo build --release && rm -rf src

# Copy actual source
COPY src ./src

# Build for release
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/release/ai-agent-oidc-hub /app/oidc-hub

ENV PORT=8080
EXPOSE 8080

ENTRYPOINT ["/app/oidc-hub"]
