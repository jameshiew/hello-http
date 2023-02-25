# hello-http

This is a simple HTTP server packaged inside a Docker image, for testing/demonstration purposes.

## Configuration

| Environment variable | Description                                     | Default Value |
| -------------------- | ----------------------------------------------- | ------------- |
| `HTTP_HOST`          | The IP address to which the server should bind. | `127.0.0.1`   |
| `HTTP_PORT`          | The port number to listen on.                   | `3000`        |

## Endpoints

`GET /` - Hello world HTML index page
`GET /readyz` - 200 if ready, 503 if not
`GET /livez` - 200 if live, 500 if not
`POST /` - post a JSON like below to update the readiness/liveness of the service

```json
{
  "ready": false,
  "live": false
}
```
