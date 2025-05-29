# Use specific Rust version for reproducibility
FROM rust:1.75-slim AS builder

# Install only essential build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy manifests and prepare for dependency caching
COPY Cargo.toml Cargo.lock ./
COPY .sqlx ./.sqlx

# Create a dummy project structure that matches your real one
RUN mkdir -p src/bin && \
    echo "fn main() {println!(\"dummy\");}" > src/main.rs

# Build dependencies only - this layer will be cached
RUN cargo build --release
RUN rm src/main.rs

# Copy real source code
COPY src/ src/

# Build with dependencies already compiled
# Touch main.rs to ensure it rebuilds
RUN touch src/main.rs && \
    cargo build --release --locked

# Runtime stage - use smaller base image
FROM debian:bookworm-slim

# Install only runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd -r appuser && useradd -r -g appuser appuser

WORKDIR /app

# Copy binary (fix the binary name - should be 'ogonek' not 'rust')
COPY --from=builder /usr/src/app/target/release/ogonek ./app

USER appuser
ENV RUST_LOG=info
CMD ["./app"]