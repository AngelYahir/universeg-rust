use crate::interface::rest::middlewares::auth::auth_middleware;
use crate::interface::rest::{controllers::auth as ctrl, state::ApiDeps};
use axum::{
    Router, middleware,
    routing::{get, post},
};

pub fn routes(deps: ApiDeps) -> Router<()> {
    Router::new()
        .route("/auth/login", post(ctrl::login))
        .route("/auth/register", post(ctrl::register))
        .route(
            "/auth/me",
            get(ctrl::get_info).layer(middleware::from_fn_with_state(
                deps.clone(),
                auth_middleware,
            )),
        )
        .with_state(deps)
}
