use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("invalid email")]
    InvalidEmail,
    #[error("invalid username")]
    InvalidUsername,
    #[error("unsupported password hash")]
    UnsupportedHash,
}
