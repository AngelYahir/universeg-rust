use crate::app::ports::{AppError, JwtService, PasswordHasher, UserRepository};
use crate::domain::auth::Email;
use async_trait::async_trait;

pub struct LoginCommand {
    pub email: String,
    pub password: String,
}

pub struct LoginResult {
    pub jwt: String,
    pub username: String,
}

#[async_trait]
pub trait LoginHandler: Send + Sync {
    async fn handle(&self, cmd: LoginCommand) -> Result<LoginResult, AppError>;
}

pub struct LoginHandlerImpl<R, H, J> {
    repo: R,
    hasher: H,
    jwt: J,
}

impl<R, H, J> LoginHandlerImpl<R, H, J> {
    pub fn new(repo: R, hasher: H, jwt: J) -> Self {
        Self { repo, hasher, jwt }
    }
}

#[async_trait]
impl<R, H, J> LoginHandler for LoginHandlerImpl<R, H, J>
where
    R: UserRepository,
    H: PasswordHasher,
    J: JwtService,
{
    async fn handle(&self, cmd: LoginCommand) -> Result<LoginResult, AppError> {
        let email = Email::parse(&cmd.email).map_err(|_| AppError::InvalidCredentials)?;
        let user = self
            .repo
            .find_by_email(&email)
            .await?
            .ok_or(AppError::InvalidCredentials)?;
        let ok = self
            .hasher
            .verify(&cmd.password, user.password_hash.as_str())
            .await?;
        if !ok {
            return Err(AppError::InvalidCredentials);
        }
        let token = self.jwt.sign(user.id).await?;

        Ok(LoginResult {
            jwt: token,
            username: user.username.as_str().to_owned(),
        })
    }
}
