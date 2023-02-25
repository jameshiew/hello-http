use axum::{
    http::{Request, Response, StatusCode},
    response::Html,
    routing::get,
    Router,
};
use eyre::Result;
use std::{env, time::Duration};
use tower_http::trace::TraceLayer;
use tracing::Span;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(index))
        .route("/readyz", get(readyz))
        .route("/livez", get(livez))
        .layer(
            TraceLayer::new_for_http()
                .on_request(|request: &Request<_>, _span: &Span| {
                    let uri = request.uri();
                    tracing::info!(%uri, "Received request");
                })
                .on_response(|response: &Response<_>, latency: Duration, _span: &Span| {
                    let status = response.status();
                    tracing::info!(?latency, ?status, "Sent response");
                }),
        );

    let host = env::var("HTTP_HOST").unwrap_or("127.0.0.1".into());
    let port = env::var("HTTP_PORT").unwrap_or("3000".into());
    let addr = format!("{host}:{port}").parse()?;

    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn index() -> Html<&'static str> {
    Html("<p>Hello, World!</p>")
}

async fn livez() -> StatusCode {
    StatusCode::OK
}

async fn readyz() -> StatusCode {
    StatusCode::OK
}
