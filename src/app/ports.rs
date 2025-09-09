use crate::domain::user::errors::DomainError as AuthDomainError;
use crate::domain::user::{Email, User, Username};
use async_trait::async_trait;
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("not found")]
    NotFound,
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("conflict")]
    Conflict,
    #[error("verification token expired")]
    TokenExpired,
    #[error("verification token already used")]
    TokenAlreadyUsed,
    #[error("invalid verification token")]
    InvalidToken,
    #[error(transparent)]
    Infra(#[from] anyhow::Error),
    #[error(transparent)]
    AuthDomain(#[from] AuthDomainError),
}

//? Users
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, AppError>;
    async fn create(
        &self,
        email: Email,
        username: Username,
        password_hash: String,
    ) -> Result<User, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
}

//? auth - email verification

#[async_trait]
pub trait PasswordHasher: Send + Sync {
    async fn hash(&self, password: &str) -> Result<String, AppError>;
    async fn verify(&self, password: &str, hash: &str) -> Result<bool, AppError>;
}

#[async_trait::async_trait]
pub trait JwtService: Send + Sync {
    async fn sign(&self, user_id: Uuid) -> Result<String, AppError>;
    async fn verify(&self, token: &str) -> Result<Uuid, AppError>;
}

#[async_trait]
pub trait VerificationRepository: Send + Sync {
    async fn verify_email(&self, token: String) -> Result<(), AppError>;
    async fn add_ver_token(
        &self,
        user_id: Uuid,
        token: String,
        expiration: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), AppError>;
}

//? email services
#[async_trait]
pub trait EmailService: Send + Sync {
    async fn send_verification_email(&self, to: &Email, token: &str) -> Result<(), AppError>;
}
