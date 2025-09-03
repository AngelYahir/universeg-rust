use crate::interface::rest::middlewares::auth::auth_middleware;
use axum::{Router, middleware, routing::get};
use std::sync::Arc;

#[derive(Clone)]
pub struct UserState {
    pub get_info_handler: Arc<dyn crate::app::usecases::user::get_info::GetInfoHandler>,
}

pub fn mount(core: crate::interface::rest::state::CoreState) -> Router<()> {
    let user_repo =
        crate::infrastructure::postgres::user_repo::PgUserRepository::new(core.pool.clone());
    use crate::app::usecases::user::get_info::GetInfoHandlerImpl;
    let get_info = GetInfoHandlerImpl::new(user_repo);

    let state = UserState {
        get_info_handler: Arc::new(get_info),
    };

    Router::new()
        .route(
            "/info",
            get(crate::interface::rest::controllers::user::get_info),
        )
        .layer(middleware::from_fn_with_state(
            core.clone(),
            auth_middleware,
        ))
        .with_state(state)
}
