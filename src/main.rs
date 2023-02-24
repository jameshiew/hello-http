use axum::{response::Html, routing::get, Router};
use eyre::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(index));

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
