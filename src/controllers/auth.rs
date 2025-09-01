use crate::dto::auth_dto::{LoginDto, RegisterDto};
use axum::{Json, http::StatusCode, response::IntoResponse, response::Response};
use serde::Serialize;
use serde_json::json;
use validator::Validate;

#[derive(Serialize)]
pub struct AuthedUser {
    message: String,
}

pub struct ApiError(serde_json::Value);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, Json(self.0)).into_response()
    }
}

pub(crate) async fn login(Json(dto): Json<LoginDto>) -> Result<Json<AuthedUser>, ApiError> {
    if let Err(errs) = dto.validate() {
        let err_body = json!({
            "message": "Validation failed",
            "errors": errs.to_string(),
        });
        return Err(ApiError(err_body));
    }

    Ok(Json(AuthedUser {
        message: format!("Bienvenido, {}", dto.email),
    }))
}

pub(crate) async fn register(Json(dto): Json<RegisterDto>) -> Result<Json<AuthedUser>, ApiError> {
    if let Err(errs) = dto.validate() {
        let err_body = json!({
            "message": "Validation failed",
            "errors": errs.to_string(),
        });
        return Err(ApiError(err_body));
    }

    Ok(Json(AuthedUser {
        message: "Register".to_string(),
    }))
}
