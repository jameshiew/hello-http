FROM rust:1.67.1 AS builder
RUN cargo install --locked cargo-chef@0.1.51

WORKDIR /build

COPY recipe.json .

RUN cargo chef cook --release

COPY Cargo.toml .
COPY Cargo.lock .
COPY src/ src/

RUN cargo build --release

FROM debian:bookworm-20230208-slim

ADD docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh

COPY --from=builder /build/target/release/hello-http /usr/local/bin/hello-http

ENV HTTP_HOST="0.0.0.0"

ENTRYPOINT ["docker-entrypoint.sh"]
