FROM rust:1.67.1 AS builder

# run cargo search first just to update the crates.io cache
RUN cargo search

RUN cargo install --locked default-target
RUN curl \
  -v \
  -o /tmp/cargo-binstall.tgz \
  -L "https://github.com/cargo-bins/cargo-binstall/releases/download/v0.20.1/cargo-binstall-$(default-target).tgz" \
  && tar -xzvf /tmp/cargo-binstall.tgz -C $CARGO_HOME/bin
RUN cargo binstall --no-confirm cargo-chef@0.1.51

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
