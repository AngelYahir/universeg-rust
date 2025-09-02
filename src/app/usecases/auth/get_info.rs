use crate::app::ports::{AppError, JwtService, UserRepository};
use async_trait::async_trait;

pub struct GetInfoCommand {
    pub id: String,
}

pub struct GetInfoResult {
    pub username: String,
    pub email: String,
    pub id: String,
    pub is_email_verified: bool,
}

#[async_trait]
pub trait GetInfoHandler: Send + Sync {
    async fn handle(&self, cmd: GetInfoCommand) -> Result<GetInfoResult, AppError>;
}

pub struct GetInfoHandlerImpl<R, J> {
    repo: R,
    jwt: J,
}

impl<R, J> GetInfoHandlerImpl<R, J> {
    pub fn new(repo: R, jwt: J) -> Self {
        Self { repo, jwt }
    }
}

#[async_trait]
impl<R, J> GetInfoHandler for GetInfoHandlerImpl<R, J>
where
    R: UserRepository,
    J: JwtService,
{
    async fn handle(&self, cmd: GetInfoCommand) -> Result<GetInfoResult, AppError> {
        let id = self
            .jwt
            .verify(&cmd.id)
            .await
            .map_err(|_| AppError::InvalidCredentials)?;
        let user = self.repo.find_by_id(id).await?.ok_or(AppError::NotFound)?;

        Ok(GetInfoResult {
            username: user.username.as_str().to_owned(),
            email: user.email.as_str().to_owned(),
            id: user.id.to_string(),
            is_email_verified: user.is_email_verified,
        })
    }
}
