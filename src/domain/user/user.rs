use super::vo::{Email, PasswordHash, Username};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: Email,
    pub username: Username,
    pub password_hash: PasswordHash,
    pub is_email_verified: bool,
}

impl User {
    pub fn new(
        id: Uuid,
        email: Email,
        username: Username,
        password_hash: PasswordHash,
        is_email_verified: bool,
    ) -> Self {
        Self {
            id,
            email,
            username,
            password_hash,
            is_email_verified,
        }
    }
}
