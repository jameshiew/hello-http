# hello-http [![Docker Hub pulls](https://img.shields.io/docker/pulls/jameshiew/hello-http?style=flat-square)](https://hub.docker.com/r/jameshiew/hello-http/tags)

This is a simple HTTP server packaged inside a Docker image, for testing/demonstration purposes. See [the OpenAPI spec](openapi.yml) for details of endpoints.

## Configuration

| Environment variable | Description                                              | Default Value |
| -------------------- | -------------------------------------------------------- | ------------- |
| `HTTP_HOST`          | The IP address to which the server should bind.          | `127.0.0.1`   |
| `HTTP_PORT`          | The port number to listen on.                            | `3000`        |
| `HTTP_LOG_ANSI`      | Whether ANSI colors should be enabled for the log output | `1`           |
| `RUST_LOG`           | The level at which to emit logs                          | `info`        |
