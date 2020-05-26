# Cross-compile the app for musl to create a statically-linked binary for alpine.
FROM clux/muslrust:1.43.1-stable AS rust
RUN rustup component add clippy rustfmt
WORKDIR /app
COPY Cargo.toml Cargo.lock .rustfmt.toml ./
COPY src ./src
RUN cargo build --release
RUN cargo clippy
RUN cargo test
RUN cargo fmt -- --check


FROM golang:alpine AS shell
RUN apk add --no-cache shellcheck
ENV GO111MODULE=on
RUN go get mvdan.cc/sh/v3/cmd/shfmt
WORKDIR /overlay
COPY root/ ./
COPY .editorconfig /
RUN find . -type f | xargs shellcheck -e SC1008
RUN shfmt -d .


FROM oznu/s6-alpine:3.11
ENV \
    # Fail if cont-init scripts exit with non-zero code.
    S6_BEHAVIOUR_IF_STAGE2_FAILS=2 \
    # Show full backtraces for crashes.
    RUST_BACKTRACE=full \
    CRON="" \
    HEALTHCHECK_ID="" \
    PUID="" \
    PGID="" \
    GITOUT_ARGS=""
RUN apk update && \
    apk add ca-certificates curl && \
    rm -rf /var/cache/apk/*
COPY root/ /
WORKDIR /app
COPY --from=rust /app/target/x86_64-unknown-linux-musl/release/gitout ./
