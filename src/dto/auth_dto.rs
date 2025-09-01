use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use validator::{Validate, ValidationError};

static USERNAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_]{3,16}$").unwrap());

static PASSWORD_ALLOWED_CHARS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Za-z\d@$!%*?&]+$").unwrap());

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterDto {
    #[validate(regex(
        path = *USERNAME_REGEX,
        message = "Username must be 3-16 characters long and can contain letters, numbers, and underscores"
    ))]
    pub username: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(custom(
        function = validate_password,
        message = "Password must be ≥8 chars and include upper, lower, digit, and special (@$!%*?&)"
    ))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginDto {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(custom(
        function = validate_password,
        message = "Password must be ≥8 chars and include upper, lower, digit, and special (@$!%*?&)"
    ))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponseDto {
    pub token: String,
    pub username: String,
}

fn validate_password(pwd: &str) -> Result<(), ValidationError> {
    // Largo mínimo
    if pwd.len() < 8 {
        return Err(ValidationError::new("password_length"));
    }
    // Solo caracteres permitidos (opcional, si quieres restringirlos)
    if !PASSWORD_ALLOWED_CHARS.is_match(pwd) {
        return Err(ValidationError::new("password_charset"));
    }
    // Al menos una minúscula
    if !pwd.chars().any(|c| c.is_ascii_lowercase()) {
        return Err(ValidationError::new("password_lowercase"));
    }
    // Al menos una mayúscula
    if !pwd.chars().any(|c| c.is_ascii_uppercase()) {
        return Err(ValidationError::new("password_uppercase"));
    }
    // Al menos un dígito
    if !pwd.chars().any(|c| c.is_ascii_digit()) {
        return Err(ValidationError::new("password_digit"));
    }
    // Al menos un especial del set
    const SPECIALS: &[char] = &['@', '$', '!', '%', '*', '?', '&'];
    if !pwd.chars().any(|c| SPECIALS.contains(&c)) {
        return Err(ValidationError::new("password_special"));
    }

    Ok(())
}
