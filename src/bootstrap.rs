use axum::Router;
use std::sync::Arc;

pub async fn build_app() -> anyhow::Result<Router> {
    let cfg = crate::infrastructure::config::Config::from_env()?;
    crate::infrastructure::logger::init_tracing(cfg.debug);
    let pool = crate::infrastructure::postgres::pool::init_db_pool(&cfg.database_url).await?;

    // Adapters
    let user_repo = crate::infrastructure::postgres::user_repo::PgUserRepository::new(pool);
    let hasher = crate::infrastructure::security::password::BcryptHasher;
    let jwt =
        crate::infrastructure::security::jwt::Hs256Jwt::new(&cfg.jwt_secret, cfg.jwt_exp_hours);

    // Use cases
    use crate::app::usecases::auth::{login::LoginHandlerImpl, register::RegisterHandlerImpl};

    let login = LoginHandlerImpl::new(user_repo.clone(), hasher, jwt);
    let register = RegisterHandlerImpl::new(
        user_repo,
        crate::infrastructure::security::password::BcryptHasher,
        crate::infrastructure::security::jwt::Hs256Jwt::new(&cfg.jwt_secret, cfg.jwt_exp_hours),
    );

    // State
    let deps = crate::interface::rest::state::ApiDeps {
        login_handler: Arc::new(login),
        register_handler: Arc::new(register),
    };

    let app: Router = crate::interface::rest::routes::routes().with_state(deps);

    Ok(app)
}
