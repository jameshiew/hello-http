use axum::{
    extract::State,
    http::{Request, Response, StatusCode},
    response::Html,
    routing::get,
    Json, Router,
};
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::{
    env,
    sync::{Arc, RwLock},
    time::Duration,
};
use tower_http::trace::TraceLayer;
use tracing::Span;
use tracing_subscriber::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct App {
    ready: bool,
    live: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            ready: true,
            live: true,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let format = fmt::format()
        .with_thread_ids(true)
        .with_thread_names(true)
        .compact();
    tracing_subscriber::fmt().event_format(format).init();

    let shared_state = Arc::new(RwLock::new(App::default()));

    let router = Router::new()
        .route("/", get(index).post(update))
        .route("/readyz", get(readyz))
        .route("/livez", get(livez))
        .layer(
            TraceLayer::new_for_http()
                .on_request(|request: &Request<_>, _span: &Span| {
                    let uri = request.uri();
                    let method = request.method();
                    tracing::info!(%uri, %method, "Received request");
                })
                .on_response(|response: &Response<_>, latency: Duration, _span: &Span| {
                    let status = response.status();
                    tracing::info!(?latency, ?status, "Sent response");
                }),
        )
        .with_state(shared_state);

    let host = env::var("HTTP_HOST").unwrap_or("127.0.0.1".into());
    let port = env::var("HTTP_PORT").unwrap_or("3000".into());
    let addr = format!("{host}:{port}").parse()?;

    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn index() -> Html<&'static str> {
    Html("<p>Hello, World!</p>")
}

async fn update(app: State<Arc<RwLock<App>>>, Json(new): Json<App>) -> StatusCode {
    let mut app = app.as_ref().write().unwrap();
    let App { ready, live } = new;
    app.ready = ready;
    app.live = live;
    StatusCode::OK
}

async fn livez(State(state): State<Arc<RwLock<App>>>) -> StatusCode {
    let live = state.read().unwrap().live;
    if live {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

async fn readyz(State(state): State<Arc<RwLock<App>>>) -> StatusCode {
    let ready = state.read().unwrap().ready;
    if ready {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}
