use crate::{controllers::auth as ctrl, state::AppState};
use axum::{Router, routing::get};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/login", get(ctrl::login))
        .route("/auth/register", get(ctrl::register))
}
