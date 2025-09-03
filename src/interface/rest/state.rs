use crate::app::ports::JwtService;
use crate::app::usecases::auth::{login::LoginHandler, register::RegisterHandler};
use crate::app::usecases::user::get_info::GetInfoHandler;
use axum::extract::FromRef;
use std::sync::Arc;

#[derive(Clone)]
pub struct CoreState {
    pub jwt_service: Arc<dyn JwtService>,
}

#[derive(Clone)]
pub struct AuthState {
    pub core: CoreState,
    pub login_handler: Arc<dyn LoginHandler>,
    pub register_handler: Arc<dyn RegisterHandler>,
}

#[derive(Clone)]
pub struct UserState {
    pub core: CoreState,
    pub get_info_handler: Arc<dyn GetInfoHandler>,
}

impl FromRef<AuthState> for CoreState {
    fn from_ref(d: &AuthState) -> CoreState {
        d.core.clone()
    }
}
impl FromRef<UserState> for CoreState {
    fn from_ref(d: &UserState) -> CoreState {
        d.core.clone()
    }
}
