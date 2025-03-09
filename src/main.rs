mod config;
mod handlers;
mod logging;
mod models;
mod shutdown;

use std::sync::{Arc, RwLock};
use std::time::Duration;

use anyhow::Result;
use axum::Router;
use axum::http::{Request, Response};
use axum::routing::get;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::Span;

use crate::config::{LIVEZ_ROUTE, READYZ_ROUTE};
use crate::handlers::{index, livez, readyz, update};
use crate::logging::init_logger;
use crate::models::App;
use crate::shutdown::shutdown_handler;

#[tokio::main]
async fn main() -> Result<()> {
    init_logger();
    // output runtime os and architecture
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let cwd = std::env::current_dir()?;
    let cwd = cwd.to_string_lossy();
    tracing::info!(%os, %arch, %cwd, "Starting up");

    let shared_state = Arc::new(RwLock::new(App::default()));

    let router = Router::new()
        .route("/", get(index).post(update))
        .route(READYZ_ROUTE, get(readyz))
        .route(LIVEZ_ROUTE, get(livez))
        .layer(
            TraceLayer::new_for_http()
                .on_request(|request: &Request<_>, _span: &Span| {
                    let uri = request.uri();
                    if uri.path() == READYZ_ROUTE || uri.path() == LIVEZ_ROUTE {
                        return;
                    }
                    let method = request.method();
                    tracing::info!(%uri, %method, "Received request");
                })
                .on_response(|response: &Response<_>, latency: Duration, _span: &Span| {
                    let status = response.status();
                    tracing::debug!(?latency, ?status, "Sent response");
                }),
        )
        .with_state(shared_state);

    let addr = config::get_server_addr();
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("listening on {}", addr);

    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_handler())
        .await?;

    Ok(())
}
