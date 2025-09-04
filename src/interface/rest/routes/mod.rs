use axum::{Router, routing::get};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod auth;
pub mod user;

#[utoipa::path(
    get,
    path = "/",
    tag = "root",
    summary = "Health check",
    description = "Health check endpoint",
    responses(
        (status = 200, description = "Hello there"),
        (status = 400, description = "Error")
    )
)]
async fn health() -> &'static str {
    "/"
}

#[derive(OpenApi)]
#[openapi(
    paths(
        health,
    ),
    tags(
        (name = "root", description = "Root route"),
    )
)]
struct ApiDoc;

pub fn routes() -> Router<()> {
    Router::new()
        .route("/", get(|| async { "Hello there" }))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
