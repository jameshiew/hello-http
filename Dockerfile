FROM ubuntu:jammy-20230126 AS build

RUN apt-get update && apt-get install -y \
  clang \
  curl \
  git \
  llvm \
  mold

# install rustup (no default toolchain)
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain none

ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /build

COPY rust-toolchain.toml .
RUN rustup show

RUN mkdir .cargo/
COPY cargo/config.toml .cargo/config.toml

RUN cargo search
RUN cargo install --locked cargo-chef@0.1.51

COPY recipe.json .

RUN cargo chef cook --release

COPY Cargo.toml .
COPY Cargo.lock .
COPY src/ src/

RUN cargo build --release

FROM debian:bookworm-20230208-slim

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

COPY --from=build /build/target/release/hello-http /usr/local/bin/hello-http

ENTRYPOINT ["docker-entrypoint.sh"]
