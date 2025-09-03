use axum::{Router, routing::post};
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthState {
    pub login_handler: Arc<dyn crate::app::usecases::auth::login::LoginHandler>,
    pub register_handler: Arc<dyn crate::app::usecases::auth::register::RegisterHandler>,
}

pub fn mount(core: crate::interface::rest::state::CoreState) -> Router<()> {
    let user_repo =
        crate::infrastructure::postgres::user_repo::PgUserRepository::new(core.pool.clone());
    let hasher = crate::infrastructure::security::password::BcryptHasher;
    let jwt = core.jwt_service.clone();

    use crate::app::usecases::auth::{login::LoginHandlerImpl, register::RegisterHandlerImpl};
    let login = LoginHandlerImpl::new(user_repo.clone(), hasher, jwt.clone());
    let register = RegisterHandlerImpl::new(
        user_repo,
        crate::infrastructure::security::password::BcryptHasher,
        jwt,
    );

    let state = AuthState {
        login_handler: Arc::new(login),
        register_handler: Arc::new(register),
    };

    Router::new()
        .route(
            "/login",
            post(crate::interface::rest::controllers::auth::login),
        )
        .route(
            "/register",
            post(crate::interface::rest::controllers::auth::register),
        )
        .with_state(state)
}
