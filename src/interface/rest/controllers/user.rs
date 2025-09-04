use axum::{
    Json,
    extract::{Extension, State},
};
use uuid::Uuid;

use crate::app::usecases::user::get_info::GetInfoCommand;
use crate::infrastructure::errors::ApiError;
use crate::interface::rest::dto::user::GetInfoResponseDto;
use crate::interface::rest::routes::user::UserState;

type Result<T> = std::result::Result<T, ApiError>;

#[utoipa::path(
    get,
    path = "/user/info",
    tag = "User",
    responses(
        (status = 200, description = "", body = GetInfoResponseDto),
        (status = 400, description = "Bad request", body = ApiError),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError),
    )
)]
pub async fn get_info(
    State(user_state): State<UserState>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<GetInfoResponseDto>> {
    let user = user_state
        .get_info_handler
        .handle(GetInfoCommand { user_id })
        .await
        .map_err(ApiError::from)?;
    Ok(Json(GetInfoResponseDto {
        id: user.id,
        username: user.username,
        email: user.email,
        is_email_verified: user.is_email_verified,
    }))
}
