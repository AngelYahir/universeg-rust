use crate::interface::rest::{controllers::user as ctrl, state::UserState};
use axum::Router;
use axum::routing::get;

pub fn routes() -> Router<UserState> {
    Router::new().route("/info", get(ctrl::get_info))
}
