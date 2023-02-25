# hello-http

This is a simple HTTP server packaged inside a Docker image, for testing/demonstration purposes. See [the OpenAPI spec](openapi.yml) for details of endpoints.

## Configuration

| Environment variable | Description                                     | Default Value |
| -------------------- | ----------------------------------------------- | ------------- |
| `HTTP_HOST`          | The IP address to which the server should bind. | `127.0.0.1`   |
| `HTTP_PORT`          | The port number to listen on.                   | `3000`        |
