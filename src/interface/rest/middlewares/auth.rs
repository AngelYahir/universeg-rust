use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::extract::TypedHeader;
use axum_extra::headers::{Authorization, authorization::Bearer};
use uuid::Uuid;

use crate::interface::rest::state::ApiDeps;

pub async fn auth_middleware(
    State(deps): State<ApiDeps>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = bearer.token();

    let user_id: Uuid = deps
        .jwt_service
        .verify(token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(user_id);

    Ok(next.run(req).await)
}
