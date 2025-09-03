use crate::app::usecases::user::get_info::GetInfoCommand;
use crate::infrastructure::errors::ApiError;
use crate::interface::rest::state::UserState;
use axum::Extension;
use axum::{Json, extract::State};
use serde_json::json;
use uuid::Uuid;

type Result<T> = std::result::Result<T, ApiError>;

pub async fn get_info(
    State(user_state): State<UserState>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<serde_json::Value>> {
    let user = user_state
        .get_info_handler
        .handle(GetInfoCommand { user_id })
        .await
        .map_err(ApiError::from)?;
    Ok(Json(json!({
        "id": user.id,
        "username": user.username,
        "email": user.email,
        "is_email_verified": user.is_email_verified,
    })))
}
