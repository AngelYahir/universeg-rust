use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::extract::TypedHeader;
use axum_extra::headers::{Authorization, authorization::Bearer};
use serde::Serialize;
use serde_json::json;
use validator::Validate;

use crate::app::usecases::auth::{login::LoginCommand, register::RegisterCommand};
use crate::interface::rest::dto::auth::{AuthResponseDto, LoginDto, RegisterDto};
use crate::interface::rest::state::ApiDeps;

#[derive(Serialize)]
pub struct ApiError(serde_json::Value);
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, Json(self.0)).into_response()
    }
}

pub async fn login(
    State(deps): State<ApiDeps>,
    Json(dto): Json<LoginDto>,
) -> Result<Json<AuthResponseDto>, ApiError> {
    if let Err(errs) = dto.validate() {
        return Err(ApiError(
            json!({ "message": "Validation failed", "errors": errs.to_string() }),
        ));
    }
    let res = deps
        .login_handler
        .handle(LoginCommand {
            email: dto.email.clone(),
            password: dto.password,
        })
        .await
        .map_err(|e| ApiError(json!({ "message": e.to_string() })))?;
    Ok(Json(AuthResponseDto {
        token: res.jwt,
        username: res.username,
    }))
}

pub async fn register(
    State(deps): State<ApiDeps>,
    Json(dto): Json<RegisterDto>,
) -> Result<Json<AuthResponseDto>, ApiError> {
    if let Err(errs) = dto.validate() {
        return Err(ApiError(
            json!({ "message": "Validation failed", "errors": errs.to_string() }),
        ));
    }
    let res = deps
        .register_handler
        .handle(RegisterCommand {
            username: dto.username,
            email: dto.email,
            password: dto.password,
        })
        .await
        .map_err(|e| ApiError(json!({ "message": e.to_string() })))?;
    Ok(Json(AuthResponseDto {
        token: res.jwt,
        username: res.username,
    }))
}

pub async fn get_info(
    State(deps): State<ApiDeps>,
    auth_header: Option<TypedHeader<Authorization<Bearer>>>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let token = if let Some(header) = auth_header {
        header.token().to_string()
    } else {
        return Err(ApiError(
            json!({ "message": "Authorization header missing" }),
        ));
    };
    let res = deps
        .get_info_handler
        .handle(crate::app::usecases::auth::get_info::GetInfoCommand { id: token })
        .await
        .map_err(|e| ApiError(json!({ "message": e.to_string() })))?;
    Ok(Json(json!({
        "id": res.id,
        "username": res.username,
        "email": res.email,
        "is_email_verified": res.is_email_verified,
    })))
}
