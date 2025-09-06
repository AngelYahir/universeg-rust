use axum::{Router, routing::get};
use utoipa::OpenApi;

use crate::infrastructure::swagger::config::SecurityAddon;
use crate::interface::rest::controllers as ctrls;

pub mod auth;
pub mod user;

#[derive(OpenApi)]
#[openapi(
    paths(
        ctrls::auth::login,
        ctrls::auth::register,
        ctrls::user::get_info,
    ),
    tags(
        (name = "Auth", description = "Authentication routes"),
        (name = "User", description = "User profile routes"),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

pub fn routes() -> Router<()> {
    Router::new()
        .route("/", get(|| async { "Hello there" }))

}
