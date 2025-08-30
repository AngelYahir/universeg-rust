use crate::state::AppState;
use axum::{Router, routing::get};

pub mod auth;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(|| async { "Hello there" }))
        .merge(auth::router())
}
