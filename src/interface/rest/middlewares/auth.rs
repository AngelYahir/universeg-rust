use crate::infrastructure::errors::ApiError;
use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use axum_extra::extract::TypedHeader;
use axum_extra::headers::{Authorization, authorization::Bearer};
use uuid::Uuid;

use crate::interface::rest::state::CoreState;

pub async fn auth_middleware(
    State(core): State<CoreState>,
    auth_header: Option<TypedHeader<Authorization<Bearer>>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, ApiError> {
    let auth_header =
        auth_header.ok_or_else(|| ApiError::unauthorized("Missing Authorization header"))?;
    let TypedHeader(Authorization(bearer)) = auth_header;
    let token = bearer.token();

    let user_id: Uuid = core
        .jwt_service
        .verify(token)
        .await
        .map_err(|e| ApiError::unauthorized(format!("Invalid or expired token: {e}")))?;

    req.extensions_mut().insert(user_id);

    Ok(next.run(req).await)
}
