use crate::interface::rest::state::ApiDeps;
use axum::{Router, routing::get};

pub mod auth;

pub fn routes(deps: ApiDeps) -> Router<()> {
    Router::new()
        .route("/", get(|| async { "Hello there" }))
        .merge(auth::routes(deps))
}
