# Builder - keeping it lean 🔥
FROM rust:1.84-slim-bullseye as builder
WORKDIR /usr/src/app

# Pro-tip: Only copy what's needed for deps first
COPY Cargo.* ./

# Create dummy src for dependency caching
RUN mkdir src && \
    echo "fn main(){}" > src/main.rs && \
    cargo fetch

# Now yeet in the real code
COPY src src/
RUN cargo build --release

# Runner - keeping it minimal
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/YOUR_BINARY_NAME ./app

# Security first bestie
RUN groupadd -r appuser && useradd -r -g appuser appuser
USER appuser

CMD ["./app"]