use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

static USERNAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_]{3,16}$").unwrap());

static PASSWORD_ALLOWED_CHARS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Za-z\d@$!%*?&]+$").unwrap());

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterDto {
    #[schema(example = "johndoe")]
    #[validate(regex(
        path = *USERNAME_REGEX,
        message = "Username must be 3-16 characters long and can contain letters, numbers, and underscores"
    ))]
    pub username: String,

    #[schema(example = "gamer@universeg.gg")]
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[schema(example = "P@ssw0rd!")]
    #[validate(custom(
        function = validate_password,
        message = "Password must be ≥8 chars and include upper, lower, digit, and special (@$!%*?&)"
    ))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginDto {
    #[schema(example = "gamer@universeg.gg")]
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[schema(example = "P@ssw0rd!")]
    #[validate(custom(
        function = validate_password,
        message = "Password must be ≥8 chars and include upper, lower, digit, and special (@$!%*?&)"
    ))]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponseDto {
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub token: String,

    #[schema(example = "johndoe")]
    pub username: String,
}

pub fn validate_password(pwd: &str) -> Result<(), ValidationError> {
    if pwd.len() < 8 {
        return Err(ValidationError::new("password_length"));
    }
    if !PASSWORD_ALLOWED_CHARS.is_match(pwd) {
        return Err(ValidationError::new("password_charset"));
    }
    if !pwd.chars().any(|c| c.is_ascii_lowercase()) {
        return Err(ValidationError::new("password_lowercase"));
    }
    if !pwd.chars().any(|c| c.is_ascii_uppercase()) {
        return Err(ValidationError::new("password_uppercase"));
    }
    if !pwd.chars().any(|c| c.is_ascii_digit()) {
        return Err(ValidationError::new("password_digit"));
    }
    const SPECIALS: &[char] = &['@', '$', '!', '%', '*', '?', '&'];
    if !pwd.chars().any(|c| SPECIALS.contains(&c)) {
        return Err(ValidationError::new("password_special"));
    }
    Ok(())
}
