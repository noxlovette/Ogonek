FROM rust:slim-bookworm AS builder
WORKDIR /usr/src/app
COPY Cargo.* ./
RUN mkdir src && \
    echo "fn main(){}" > src/main.rs && \
    cargo fetch
COPY src src/
RUN cargo build --release --locked && \
    strip target/release/upload-service

FROM debian:bookworm-slim
WORKDIR /app
RUN groupadd -r appuser && useradd -r -g appuser appuser
COPY --from=builder /usr/src/app/target/release/upload-service ./app
USER appuser
CMD ["./app"]