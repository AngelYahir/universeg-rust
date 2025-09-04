use axum::{Json, extract::State};
use serde_json::json;
use validator::Validate;

use crate::app::usecases::auth::{login::LoginCommand, register::RegisterCommand};
use crate::infrastructure::errors::ApiError;
use crate::interface::rest::dto::auth::{AuthResponseDto, LoginDto, RegisterDto};
use crate::interface::rest::routes::auth::AuthState;

type Result<T> = std::result::Result<T, ApiError>;

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginDto,
    tag = "Auth",
    responses(
        (status = 200, description = "User logged in successfully", body = AuthResponseDto),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError),
    )
)]
pub async fn login(
    State(deps): State<AuthState>,
    Json(dto): Json<LoginDto>,
) -> Result<Json<AuthResponseDto>> {
    if let Err(errs) = dto.validate() {
        return Err(ApiError::bad_request("Validation failed")
            .with_details(json!({ "errors": errs.to_string() })));
    }
    let res = deps
        .login_handler
        .handle(LoginCommand {
            email: dto.email.clone(),
            password: dto.password,
        })
        .await
        .map_err(ApiError::from)?;
    Ok(Json(AuthResponseDto {
        token: res.jwt,
        username: res.username,
    }))
}

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterDto,
    tag = "Auth",
    responses(
        (status = 200, description = "User registered successfully", body = AuthResponseDto),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError),
    )
)]
pub async fn register(
    State(deps): State<AuthState>,
    Json(dto): Json<RegisterDto>,
) -> Result<Json<AuthResponseDto>> {
    if let Err(errs) = dto.validate() {
        return Err(ApiError::bad_request("Validation failed")
            .with_details(json!({ "errors": errs.to_string() })));
    }
    let res = deps
        .register_handler
        .handle(RegisterCommand {
            username: dto.username,
            email: dto.email,
            password: dto.password,
        })
        .await
        .map_err(ApiError::from)?;
    Ok(Json(AuthResponseDto {
        token: res.jwt,
        username: res.username,
    }))
}
