use axum::Router;
use std::sync::Arc;
use utoipa_swagger_ui::SwaggerUi;
use crate::interface::rest::routes::ApiDoc;
use utoipa::OpenApi;

pub async fn build_app() -> anyhow::Result<Router> {
    // Core infra
    let cfg = Arc::new(crate::infrastructure::config::Config::from_env()?);
    crate::infrastructure::logger::init_tracing(cfg.debug);
    let pool = crate::infrastructure::postgres::pool::init_db_pool(&cfg.database_url).await?;
    let mongo_db =
        crate::infrastructure::mongo::connection::init_mongo(&cfg.mongo_uri, &cfg.mongo_db_name)
            .await?;
    let jwt = Arc::new(crate::infrastructure::security::jwt::Hs256Jwt::new(
        &cfg.jwt_secret,
        cfg.jwt_exp_hours,
    ));

    // CoreState
    let core = crate::interface::rest::state::CoreState {
        pool,
        jwt_service: jwt,
        mongo_db,
    };

    // Mount routers
    let auth_router = crate::interface::rest::routes::auth::mount(core.clone());
    let user_router = crate::interface::rest::routes::user::mount(core.clone());
    let swagger_router = SwaggerUi::new("/api/docs")
        .url("/api/docs/openapi.json", ApiDoc::openapi());

    // Root router
    let app: Router = crate::interface::rest::routes::routes()
        .merge(swagger_router)
        .nest("/auth", auth_router)
        .nest("/user", user_router);

    Ok(app)
}
