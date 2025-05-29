FROM rust:latest AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
# Copy .sqlx prepared statements
COPY .sqlx ./.sqlx

# Create minimal src to build dependencies
RUN mkdir -p src && \
    echo "fn main() {}" > src/main.rs && \
    cargo fetch && \
    cargo build --release && \
    rm -rf src

# Copy actual source and rebuild
COPY src src/
RUN cargo build --release --locked && \
    strip target/release/rust

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd -r appuser && useradd -r -g appuser appuser

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/ogonek ./app

# Use non-root user
USER appuser

# Configure environment variables
ENV RUST_LOG=info

# Run the application
CMD ["./app"]