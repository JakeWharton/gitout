# Cross-compile the app for musl to create a statically-linked binary for alpine.
FROM rust:1.62.0 AS rust
RUN rustup component add clippy rustfmt
WORKDIR /app
COPY Cargo.toml Cargo.lock .rustfmt.toml ./
COPY src ./src
RUN cargo build --release
RUN cargo clippy
RUN cargo test
RUN cargo fmt -- --check


FROM golang:1.18-alpine AS shell
RUN apk add --no-cache shellcheck
ENV GO111MODULE=on
RUN go install mvdan.cc/sh/v3/cmd/shfmt@latest
WORKDIR /overlay
COPY root/ ./
COPY .editorconfig /
RUN find . -type f | xargs shellcheck -e SC1008
RUN shfmt -d .


FROM debian:buster-slim
ADD https://github.com/just-containers/s6-overlay/releases/download/v2.2.0.1/s6-overlay-amd64-installer /tmp/
RUN chmod +x /tmp/s6-overlay-amd64-installer && /tmp/s6-overlay-amd64-installer /
ENV \
    # Fail if cont-init scripts exit with non-zero code.
    S6_BEHAVIOUR_IF_STAGE2_FAILS=2 \
    # Show full backtraces for crashes.
    RUST_BACKTRACE=full \
    CRON="" \
    HEALTHCHECK_ID="" \
    HEALTHCHECK_HOST="https://hc-ping.com" \
    PUID="" \
    PGID="" \
    GITOUT_ARGS=""
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        curl \
      && \
    rm -rf /var/lib/apt/lists/*
COPY root/ /
WORKDIR /app
COPY --from=rust /app/target/release/gitout ./
