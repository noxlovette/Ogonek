FROM rust:1-slim AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy files needed for build
COPY Cargo.toml Cargo.lock ./
COPY .sqlx ./.sqlx
COPY migrations ./migrations
COPY src/ src/

# Build everything
RUN cargo build --release --locked

# Runtime
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/ogonek ./app

CMD ["./app"]
