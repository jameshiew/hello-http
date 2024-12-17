FROM rust:1.83.0 AS builder

WORKDIR /opt

COPY Cargo.toml .
COPY Cargo.lock .
COPY src/ src/

RUN cargo build --release

FROM debian:bookworm-20241202-slim

RUN apt-get update && apt-get install -y curl

COPY docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh
COPY healthcheck.sh /usr/local/bin/healthcheck.sh

HEALTHCHECK --interval=10s --timeout=10s --retries=3 \
  CMD "healthcheck.sh"

ARG HTTP_PORT=3000
ENV HTTP_PORT=$HTTP_PORT
# default is 0.0.0.0 as 127.0.0.1 may not work in some cases inside a Docker container
ARG HTTP_HOST="0.0.0.0"
ENV HTTP_HOST=$HTTP_HOST

COPY --from=builder /opt/target/release/hello-http /usr/local/bin/hello-http

ENTRYPOINT ["docker-entrypoint.sh"]
