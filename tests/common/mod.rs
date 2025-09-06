use axum::Router;
use dotenvy::dotenv;
use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize test environment only once
pub fn init_test_env() {
    INIT.call_once(|| {
        dotenv().ok();
        // Initialize tracing only if not already initialized
        let _ = tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .try_init();
    });
}

/// Build app for testing without tracing conflicts
pub async fn build_test_app() -> Result<Router, Box<dyn std::error::Error + Send + Sync>> {
    init_test_env();

    // Build the app without calling init_tracing again
    let config = universeg_api::infrastructure::config::Config::from_env()?;
    let pool = universeg_api::infrastructure::postgres::pool::init_db_pool(&config.database_url).await?;
    let mongo_db = universeg_api::infrastructure::mongo::connection::init_mongo(&config.mongo_uri, &config.mongo_db_name).await?;
    let jwt_service = std::sync::Arc::new(
        universeg_api::infrastructure::security::jwt::Hs256Jwt::new(&config.jwt_secret, config.jwt_exp_hours)
    );

    let core_state = universeg_api::interface::rest::state::CoreState {
        pool: pool.clone(),
        mongo_db,
        jwt_service,
    };

    // Mount routers like in bootstrap
    let auth_router = universeg_api::interface::rest::routes::auth::mount(core_state.clone());
    let user_router = universeg_api::interface::rest::routes::user::mount(core_state.clone());

    // Root router
    let app: axum::Router = universeg_api::interface::rest::routes::routes()
        .nest("/auth", auth_router)
        .nest("/user", user_router);

    Ok(app)
}
