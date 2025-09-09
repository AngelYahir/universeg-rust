use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthState {
    pub login_handler: Arc<dyn crate::app::usecases::auth::login::LoginHandler>,
    pub register_handler: Arc<dyn crate::app::usecases::auth::register::RegisterHandler>,
    pub verify_token_handler: Arc<dyn crate::app::usecases::auth::email_ver::VerifyTokenHandler>,
}

pub fn mount(core: crate::interface::rest::state::CoreState) -> Router<()> {
    let user_repo =
        crate::infrastructure::postgres::repository::user::PgUserRepository::new(core.pool.clone());
    let hasher = crate::infrastructure::security::password::BcryptHasher;
    let jwt = core.jwt_service.clone();

    // Create verification repository
    let verification_repo =
        crate::infrastructure::postgres::repository::verification::PgVerificationRepository::new(
            core.pool.clone(),
        );

    // Use email service from CoreState
    let email_service = Arc::new(
        crate::infrastructure::email::service::SmtpEmailService::new(
            core.mailer.clone(),
            "http://localhost:3000".to_string(), // Base URL for verification links
        ),
    );

    use crate::app::usecases::auth::{
        email_ver::{EmailVerHandlerImpl, VerifyTokenHandlerImpl},
        login::LoginHandlerImpl,
        register::RegisterHandlerImpl,
    };

    let email_ver_handler = Arc::new(EmailVerHandlerImpl::new(
        verification_repo.clone(),
        email_service,
        user_repo.clone(),
    ));

    let login = LoginHandlerImpl::new(user_repo.clone(), hasher, jwt.clone());
    let register = RegisterHandlerImpl::new(
        user_repo,
        crate::infrastructure::security::password::BcryptHasher,
        jwt,
        email_ver_handler,
    );

    let verify_token = VerifyTokenHandlerImpl::new(verification_repo);

    let state = AuthState {
        login_handler: Arc::new(login),
        register_handler: Arc::new(register),
        verify_token_handler: Arc::new(verify_token),
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
        .route(
            "/verify-email",
            get(crate::interface::rest::controllers::auth::verify_email),
        )
        .with_state(state)
}
