FROM rust:1-alpine AS builder

# Install musl target and static deps
RUN apk add --no-cache \
    musl-dev \
    pkgconf \
    openssl-dev \
    openssl-libs-static

# Add musl target for static linking
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app

# Copy your project files
COPY Cargo.toml Cargo.lock ./
COPY .sqlx ./.sqlx
COPY migrations ./migrations
COPY src/ src/

# Build with musl target for fully static binary
ENV RUSTFLAGS="-C target-feature=+crt-static"
ENV PKG_CONFIG_ALL_STATIC=1
ENV PKG_CONFIG_ALL_DYNAMIC=0

RUN cargo build --release --target x86_64-unknown-linux-musl --locked

# Distroless final stage - no libc dependencies needed
FROM gcr.io/distroless/static-debian12:nonroot

WORKDIR /app
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/ogonek ./app

USER nonroot:nonroot
EXPOSE 3000

CMD ["./app"]