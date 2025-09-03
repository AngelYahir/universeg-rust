use crate::interface::rest::{controllers::auth as ctrl, state::AuthState};
use axum::Router;
use axum::routing::post;

pub fn routes() -> Router<AuthState> {
    Router::new()
        .route("/login", post(ctrl::login))
        .route("/register", post(ctrl::register))
}
