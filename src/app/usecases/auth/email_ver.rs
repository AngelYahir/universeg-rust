use crate::app::ports::{AppError, EmailService, UserRepository, VerificationRepository};
use crate::domain::user::Email;
use async_trait::async_trait;
use chrono::{Duration, Utc};
use sha2::{Digest, Sha256};
use tracing::{debug, error, info, warn};

#[derive(Debug)]
pub struct EmailVerCommand {
    pub email: String,
}

pub struct EmailVerResult;

#[async_trait]
pub trait EmailVerHandler: Send + Sync {
    async fn handle(&self, cmd: EmailVerCommand) -> Result<EmailVerResult, AppError>;
}

pub struct EmailVerHandlerImpl<R, E, U> {
    verification_repo: R,
    email_service: E,
    user_repo: U,
}

impl<R, E, U> EmailVerHandlerImpl<R, E, U> {
    pub fn new(verification_repo: R, email_service: E, user_repo: U) -> Self {
        Self {
            verification_repo,
            email_service,
            user_repo,
        }
    }
}

#[async_trait]
impl<R, E, U> EmailVerHandler for EmailVerHandlerImpl<R, E, U>
where
    R: VerificationRepository + Send + Sync,
    E: EmailService + Send + Sync,
    U: UserRepository + Send + Sync,
{
    async fn handle(&self, cmd: EmailVerCommand) -> Result<EmailVerResult, AppError> {
        info!("Starting email verification process for email: {}", cmd.email);

        let email = Email::parse(&cmd.email).map_err(|e| {
            error!("Failed to parse email address '{}': {:?}", cmd.email, e);
            AppError::InvalidCredentials
        })?;

        debug!("Email address parsed successfully: {}", email.as_str());

        let user = self
            .user_repo
            .find_by_email(&email)
            .await
            .map_err(|e| {
                error!("Database error while finding user by email '{}': {:?}", email.as_str(), e);
                e
            })?
            .ok_or_else(|| {
                warn!("User not found for email: {}", email.as_str());
                AppError::NotFound
            })?;

        info!("User found for email verification, user_id: {}", user.id);

        // Generate secure random token bytes
        debug!("Generating secure random token");
        let token_bytes: [u8; 32] = rand::random();

        debug!("Hashing token for storage");
        let mut hasher = Sha256::new();
        hasher.update(&token_bytes);
        let token_hash = hasher.finalize();
        let token_string = hex::encode(&token_bytes);
        let token_hash_string = hex::encode(token_hash);
        let expiration = Utc::now() + Duration::hours(24);

        debug!("Storing verification token in database, expires at: {}", expiration);
        self.verification_repo
            .add_ver_token(user.id, token_hash_string, expiration)
            .await
            .map_err(|e| {
                error!("Failed to store verification token for user {}: {:?}", user.id, e);
                e
            })?;

        info!("Verification token stored successfully, sending email to: {}", email.as_str());
        self.email_service
            .send_verification_email(&email, &token_string)
            .await
            .map_err(|e| {
                error!("Failed to send verification email to '{}': {:?}", email.as_str(), e);
                e
            })?;

        info!("Email verification process completed successfully for: {}", email.as_str());
        Ok(EmailVerResult)
    }
}

// Separate handler for token verification
#[derive(Debug)]
pub struct VerifyTokenCommand {
    pub token: String,
}

pub struct VerifyTokenResult;

#[async_trait]
pub trait VerifyTokenHandler: Send + Sync {
    async fn handle(&self, cmd: VerifyTokenCommand) -> Result<VerifyTokenResult, AppError>;
}

pub struct VerifyTokenHandlerImpl<R> {
    repo: R,
}

impl<R> VerifyTokenHandlerImpl<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R> VerifyTokenHandler for VerifyTokenHandlerImpl<R>
where
    R: VerificationRepository + Send + Sync,
{
    async fn handle(&self, cmd: VerifyTokenCommand) -> Result<VerifyTokenResult, AppError> {
        info!("Starting token verification process for token: {}...", &cmd.token[..8.min(cmd.token.len())]);

        let token_bytes = hex::decode(&cmd.token).map_err(|e| {
            error!("Failed to decode hex token: {:?}", e);
            AppError::InvalidToken
        })?;

        debug!("Token decoded successfully, hashing for verification");
        let mut hasher = Sha256::new();
        hasher.update(&token_bytes);
        let token_hash = hasher.finalize();
        let token_hash_string = hex::encode(token_hash);

        debug!("Verifying token hash in database");
        self.repo.verify_email(token_hash_string).await.map_err(|e| {
            error!("Token verification failed: {:?}", e);
            e
        })?;

        info!("Token verification completed successfully");
        Ok(VerifyTokenResult)
    }
}
