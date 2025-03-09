use std::sync::{Arc, RwLock};

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::Json;

use crate::models::App;

pub async fn index() -> Html<&'static str> {
    Html("<p>Hello, World!</p>")
}

pub async fn update(app: State<Arc<RwLock<App>>>, Json(new): Json<App>) -> StatusCode {
    let mut app = app.as_ref().write().unwrap();
    let App { ready, live } = new;
    app.ready = ready;
    app.live = live;
    StatusCode::OK
}

pub async fn livez(State(state): State<Arc<RwLock<App>>>) -> StatusCode {
    let live = state.read().unwrap().live;
    if live {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn readyz(State(state): State<Arc<RwLock<App>>>) -> StatusCode {
    let ready = state.read().unwrap().ready;
    if ready {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}