use crate::{controllers::auth as ctrl, state::AppState};
use axum::{Router, routing::post};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/login", post(ctrl::login))
        .route("/auth/register", post(ctrl::register))
}
