use std::sync::Arc;

use crate::app::ports::{AppError, JwtService, PasswordHasher, UserRepository};
use crate::app::usecases::auth::email_ver::{EmailVerHandler, EmailVerCommand};
use crate::domain::user::{Email, Username};
use async_trait::async_trait;
use tracing::{debug, error, info, warn};

pub struct RegisterCommand {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct RegisterResult {
    pub jwt: String,
    pub username: String,
}

#[async_trait]
pub trait RegisterHandler: Send + Sync {
    async fn handle(&self, cmd: RegisterCommand) -> Result<RegisterResult, AppError>;
}

pub struct RegisterHandlerImpl<R, H> {
    repo: R,
    hasher: H,
    jwt: Arc<dyn JwtService>,
    email_ver_handler: Arc<dyn EmailVerHandler>,
}

impl<R, H> RegisterHandlerImpl<R, H> {
    pub fn new(repo: R, hasher: H, jwt: Arc<dyn JwtService>, email_ver_handler: Arc<dyn EmailVerHandler>) -> Self {
        Self { repo, hasher, jwt, email_ver_handler }
    }
}

#[async_trait]
impl<R, H> RegisterHandler for RegisterHandlerImpl<R, H>
where
    R: UserRepository,
    H: PasswordHasher,
{
    async fn handle(&self, cmd: RegisterCommand) -> Result<RegisterResult, AppError> {
        info!("Starting user registration process for username: {} and email: {}", cmd.username, cmd.email);

        let email = Email::parse(&cmd.email).map_err(|e| {
            error!("Failed to parse email address '{}' during registration: {:?}", cmd.email, e);
            AppError::Conflict
        })?;

        let username = Username::parse(&cmd.username).map_err(|e| {
            error!("Failed to parse username '{}' during registration: {:?}", cmd.username, e);
            AppError::Conflict
        })?;

        debug!("Email and username parsed successfully");

        debug!("Checking if user already exists with email: {}", email.as_str());
        if self.repo.find_by_email(&email).await?.is_some() {
            warn!("Registration attempt for existing email: {}", email.as_str());
            return Err(AppError::Conflict);
        }

        info!("Email is available, proceeding with user creation");
        debug!("Hashing password for user: {}", username.as_str());
        let hash = self.hasher.hash(&cmd.password).await.map_err(|e| {
            error!("Failed to hash password during registration: {:?}", e);
            e
        })?;

        debug!("Password hashed successfully, creating user in database");
        let user = self.repo.create(email.clone(), username, hash).await.map_err(|e| {
            error!("Failed to create user in database: {:?}", e);
            e
        })?;

        info!("User created successfully with ID: {}, now sending verification email", user.id);

        // Send verification email
        debug!("Calling email verification handler for user: {}", user.id);
        self.email_ver_handler.handle(EmailVerCommand {
            email: email.as_str().to_owned(),
        }).await.map_err(|e| {
            error!("Failed to send verification email for user {}: {:?}", user.id, e);
            e
        })?;

        info!("Verification email sent successfully for user: {}", user.id);

        debug!("Generating JWT token for user: {}", user.id);
        let token = self.jwt.sign(user.id).await.map_err(|e| {
            error!("Failed to generate JWT token for user {}: {:?}", user.id, e);
            e
        })?;

        info!("User registration completed successfully for: {}", user.username.as_str());
        Ok(RegisterResult {
            jwt: token,
            username: user.username.as_str().to_owned(),
        })
    }
}
