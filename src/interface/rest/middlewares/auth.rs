use crate::infrastructure::errors::ApiError;
use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use axum_extra::extract::TypedHeader;
use axum_extra::headers::{Authorization, authorization::Bearer};
use uuid::Uuid;

use crate::interface::rest::state::CoreState;

pub async fn auth_middleware(
    State(core): State<CoreState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, ApiError> {
    let token = bearer.token();

    let user_id: Uuid = core
        .jwt_service
        .verify(token)
        .await
        .map_err(|e| ApiError::unauthorized(format!("Invalid or expired token: {e}")))?;

    req.extensions_mut().insert(user_id);

    Ok(next.run(req).await)
}
