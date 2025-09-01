use crate::interface::rest::{controllers::auth as ctrl, state::ApiDeps};
use axum::{Router, routing::post};

pub fn router() -> Router<ApiDeps> {
    Router::new()
        .route("/auth/login", post(ctrl::login))
        .route("/auth/register", post(ctrl::register))
}
