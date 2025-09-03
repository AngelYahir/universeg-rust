use std::sync::Arc;

use crate::app::ports::{AppError, JwtService, PasswordHasher, UserRepository};
use crate::domain::user::{Email, Username};
use async_trait::async_trait;

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
}

impl<R, H> RegisterHandlerImpl<R, H> {
    pub fn new(repo: R, hasher: H, jwt: Arc<dyn JwtService>) -> Self {
        Self { repo, hasher, jwt }
    }
}

#[async_trait]
impl<R, H> RegisterHandler for RegisterHandlerImpl<R, H>
where
    R: UserRepository,
    H: PasswordHasher,
{
    async fn handle(&self, cmd: RegisterCommand) -> Result<RegisterResult, AppError> {
        let email = Email::parse(&cmd.email).map_err(|_| AppError::Conflict)?;
        let username = Username::parse(&cmd.username).map_err(|_| AppError::Conflict)?;

        if self.repo.find_by_email(&email).await?.is_some() {
            return Err(AppError::Conflict);
        }

        let hash = self.hasher.hash(&cmd.password).await?;
        let user = self.repo.create(email, username, hash).await?;

        let token = self.jwt.sign(user.id).await?;
        Ok(RegisterResult {
            jwt: token,
            username: user.username.as_str().to_owned(),
        })
    }
}
