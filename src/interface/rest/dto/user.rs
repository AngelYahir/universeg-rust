use serde::Serialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Validate, ToSchema)]
pub struct GetInfoResponseDto {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    pub id: String,

    #[schema(example = "johndoe")]
    pub username: String,

    #[schema(example = "gamer@universeg.gg")]
    pub email: String,

    #[schema(example = true)]
    pub is_email_verified: bool,
}
