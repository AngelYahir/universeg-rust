use axum::{Router, routing::get};

pub mod auth;
pub mod user;

pub fn routes() -> Router<()> {
    Router::new().route("/", get(|| async { "Hello there" }))
}
