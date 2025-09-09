use axum::{Json, extract::Query, extract::State, response::Html};
use serde_json::json;
use validator::Validate;

use crate::app::usecases::auth::{
    email_ver::VerifyTokenCommand, login::LoginCommand, register::RegisterCommand,
};
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

#[derive(serde::Deserialize)]
pub struct VerifyEmailQuery {
    token: String,
}

#[utoipa::path(
    get,
    path = "/auth/verify-email",
    params(
        ("token" = String, Query, description = "Email verification token")
    ),
    tag = "Auth",
    responses(
        (status = 200, description = "Email verified successfully", content_type = "text/html"),
        (status = 400, description = "Invalid or expired token", content_type = "text/html"),
        (status = 500, description = "Internal server error", content_type = "text/html"),
    )
)]
pub async fn verify_email(
    State(deps): State<AuthState>,
    Query(params): Query<VerifyEmailQuery>,
) -> Html<String> {
    match deps
        .verify_token_handler
        .handle(VerifyTokenCommand {
            token: params.token,
        })
        .await
    {
        Ok(_) => Html(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>Email Verified</title>
                <style>
                    body { font-family: Arial, sans-serif; text-align: center; padding: 50px; }
                    .success { color: #4CAF50; }
                    .container { max-width: 400px; margin: 0 auto; }
                </style>
            </head>
            <body>
                <div class="container">
                    <h1 class="success">✓ Email Verified Successfully!</h1>
                    <p>Your email has been verified. You can now access all features of your account.</p>
                    <a href="/">Go to Home</a>
                </div>
            </body>
            </html>
            "#.to_string()
        ),
        Err(err) => {
            let (title, message) = match err {
                crate::app::ports::AppError::InvalidToken => (
                    "Invalid Token",
                    "The verification token is invalid or malformed."
                ),
                crate::app::ports::AppError::TokenExpired => (
                    "Token Expired",
                    "The verification token has expired. Please request a new verification email."
                ),
                crate::app::ports::AppError::TokenAlreadyUsed => (
                    "Token Already Used",
                    "This verification token has already been used."
                ),
                _ => (
                    "Error",
                    "An error occurred while verifying your email. Please try again later."
                ),
            };

            Html(
                format!(
                    r#"
                    <!DOCTYPE html>
                    <html>
                    <head>
                        <title>{}</title>
                        <style>
                            body {{ font-family: Arial, sans-serif; text-align: center; padding: 50px; }}
                            .error {{ color: #f44336; }}
                            .container {{ max-width: 400px; margin: 0 auto; }}
                        </style>
                    </head>
                    <body>
                        <div class="container">
                            <h1 class="error">✗ {}</h1>
                            <p>{}</p>
                            <a href="/">Go to Home</a>
                        </div>
                    </body>
                    </html>
                    "#,
                    title, title, message
                )
            )
        }
    }
}
