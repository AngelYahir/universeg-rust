use crate::interface::rest::middlewares::auth::auth_middleware;
use crate::interface::rest::routes::{auth, user};
use crate::interface::rest::state::{AuthState, CoreState, UserState};
use axum::{Router, middleware};
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
    use crate::app::usecases::user::get_info::GetInfoHandlerImpl;

    let login = LoginHandlerImpl::new(user_repo.clone(), hasher, jwt.clone());
    let register = RegisterHandlerImpl::new(
        user_repo.clone(),
        crate::infrastructure::security::password::BcryptHasher,
        jwt.clone(),
    );
    let get_info = GetInfoHandlerImpl::new(user_repo.clone());

    // States
    let core_state = CoreState {
        jwt_service: Arc::new(jwt.clone()),
    };

    let auth_state = AuthState {
        core: core_state.clone(),
        login_handler: Arc::new(login),
        register_handler: Arc::new(register),
    };

    let user_state = UserState {
        core: core_state.clone(),
        get_info_handler: Arc::new(get_info),
    };

    let auth_router = auth::routes().with_state(auth_state.clone());
    let user_router = user::routes()
        .layer(middleware::from_fn_with_state(
            core_state.clone(),
            auth_middleware,
        ))
        .with_state(user_state.clone());

    let app: Router = crate::interface::rest::routes::routes()
        .nest("/auth", auth_router)
        .nest("/user", user_router);

    Ok(app)
}
