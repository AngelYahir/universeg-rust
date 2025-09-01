use crate::interface::rest::state::ApiDeps;
use axum::{Router, routing::get};

pub mod auth;

pub fn routes() -> Router<ApiDeps> {
    Router::new()
        .route("/", get(|| async { "Hello there" }))
        .merge(auth::router())
}
