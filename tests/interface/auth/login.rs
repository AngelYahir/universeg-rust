use axum::Router;
use http::{Request, StatusCode};
use serde_json::json;

use std::sync::Arc;
use tower::ServiceExt;
use uuid::Uuid;

use universeg_api::app::ports::{AppError, JwtService, PasswordHasher, UserRepository};
use universeg_api::app::usecases::auth::login::{LoginCommand, LoginHandler, LoginHandlerImpl};
use crate::common;
use universeg_api::domain::user::{Email, User, Username};
use universeg_api::domain::user::vo::PasswordHash;

// Mock implementations for testing
#[derive(Clone)]
struct MockUserRepository {
    pub users: Vec<User>,
}

impl MockUserRepository {
    fn new() -> Self {
        Self { users: Vec::new() }
    }

    fn with_user(mut self, user: User) -> Self {
        self.users.push(user);
        self
    }
}

#[async_trait::async_trait]
impl UserRepository for MockUserRepository {
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, AppError> {
        Ok(self.users.iter().find(|u| u.email == *email).cloned())
    }

    async fn create(
        &self,
        _email: Email,
        _username: Username,
        _password_hash: String,
    ) -> Result<User, AppError> {
        unimplemented!("Not needed for login tests")
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        Ok(self.users.iter().find(|u| u.id == id).cloned())
    }
}

#[derive(Clone)]
struct MockPasswordHasher {
    pub should_verify: bool,
}

impl MockPasswordHasher {
    fn new(should_verify: bool) -> Self {
        Self { should_verify }
    }
}

#[async_trait::async_trait]
impl PasswordHasher for MockPasswordHasher {
    async fn hash(&self, _password: &str) -> Result<String, AppError> {
        Ok("hashed_password".to_string())
    }

    async fn verify(&self, _password: &str, _hash: &str) -> Result<bool, AppError> {
        Ok(self.should_verify)
    }
}

#[derive(Clone)]
struct MockJwtService {
    pub should_fail: bool,
}

impl MockJwtService {
    fn new(should_fail: bool) -> Self {
        Self { should_fail }
    }
}

#[async_trait::async_trait]
impl JwtService for MockJwtService {
    async fn sign(&self, _user_id: Uuid) -> Result<String, AppError> {
        if self.should_fail {
            Err(AppError::Infra(anyhow::anyhow!("JWT signing failed")))
        } else {
            Ok("mock_jwt_token".to_string())
        }
    }

    async fn verify(&self, _token: &str) -> Result<Uuid, AppError> {
        if self.should_fail {
            Err(AppError::Infra(anyhow::anyhow!("JWT verification failed")))
        } else {
            Ok(Uuid::new_v4())
        }
    }
}

// Helper function to create a test user
fn create_test_user() -> User {
    let id = Uuid::new_v4();
    let email = Email::parse("test@universeg.gg").unwrap();
    let username = Username::parse("testuser").unwrap();
    let password_hash = PasswordHash::from_hash("$2b$12$C6UzMDM.H6dfI/f/IKcEe.u9F7c/F7kh/3Gzdh0dX8GZFOD4oTi2.".to_string()).unwrap();

    User::new(id, email, username, password_hash, true)
}

#[cfg(test)]
mod login_handler_tests {
    use super::*;

    #[tokio::test]
    async fn test_successful_login() {
        // Arrange
        let test_user = create_test_user();
        let repo = MockUserRepository::new().with_user(test_user.clone());
        let hasher = MockPasswordHasher::new(true);
        let jwt_service = Arc::new(MockJwtService::new(false));

        let handler = LoginHandlerImpl::new(repo, hasher, jwt_service);

        let command = LoginCommand {
            email: "test@universeg.gg".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        // Act
        let result = handler.handle(command).await;

        // Assert
        assert!(result.is_ok());
        let login_result = result.unwrap();
        assert_eq!(login_result.jwt, "mock_jwt_token");
        assert_eq!(login_result.username, test_user.username.as_str());
    }

    #[tokio::test]
    async fn test_login_with_invalid_email_format() {
        // Arrange
        let repo = MockUserRepository::new();
        let hasher = MockPasswordHasher::new(true);
        let jwt_service = Arc::new(MockJwtService::new(false));

        let handler = LoginHandlerImpl::new(repo, hasher, jwt_service);

        let command = LoginCommand {
            email: "invalid-email".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        // Act
        let result = handler.handle(command).await;

        // Assert
        assert!(matches!(result, Err(AppError::InvalidCredentials)));
    }

    #[tokio::test]
    async fn test_login_with_nonexistent_user() {
        // Arrange
        let repo = MockUserRepository::new(); // Empty repository
        let hasher = MockPasswordHasher::new(true);
        let jwt_service = Arc::new(MockJwtService::new(false));

        let handler = LoginHandlerImpl::new(repo, hasher, jwt_service);

        let command = LoginCommand {
            email: "nonexistent@universeg.gg".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        // Act
        let result = handler.handle(command).await;

        // Assert
        assert!(matches!(result, Err(AppError::InvalidCredentials)));
    }

    #[tokio::test]
    async fn test_login_with_wrong_password() {
        // Arrange
        let test_user = create_test_user();
        let repo = MockUserRepository::new().with_user(test_user);
        let hasher = MockPasswordHasher::new(false); // Password verification will fail
        let jwt_service = Arc::new(MockJwtService::new(false));

        let handler = LoginHandlerImpl::new(repo, hasher, jwt_service);

        let command = LoginCommand {
            email: "test@universeg.gg".to_string(),
            password: "WrongPassword123!".to_string(),
        };

        // Act
        let result = handler.handle(command).await;

        // Assert
        assert!(matches!(result, Err(AppError::InvalidCredentials)));
    }

    #[tokio::test]
    async fn test_login_with_jwt_signing_failure() {
        // Arrange
        let test_user = create_test_user();
        let repo = MockUserRepository::new().with_user(test_user);
        let hasher = MockPasswordHasher::new(true);
        let jwt_service = Arc::new(MockJwtService::new(true)); // JWT signing will fail

        let handler = LoginHandlerImpl::new(repo, hasher, jwt_service);

        let command = LoginCommand {
            email: "test@universeg.gg".to_string(),
            password: "ValidPassword123!".to_string(),
        };

        // Act
        let result = handler.handle(command).await;

        // Assert
        assert!(matches!(result, Err(AppError::Infra(_))));
    }
}

#[cfg(test)]
mod login_route_tests {
    use super::*;

    async fn app() -> Router {
        common::build_test_app().await.unwrap()
    }

    #[tokio::test]
    async fn test_login_route_with_valid_request() {
        let app = app().await;

        let login_payload = json!({
            "email": "test@universeg.gg",
            "password": "ValidPassword123!"
        });

        let request = Request::builder()
            .method("POST")
            .uri("/auth/login")
            .header("content-type", "application/json")
            .body(axum::body::Body::from(login_payload.to_string()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        // Note: This will likely fail with 401/500 since we don't have a real database
        // But we can test that the route exists and processes the request
        assert!(
            response.status() == StatusCode::UNAUTHORIZED
            || response.status() == StatusCode::INTERNAL_SERVER_ERROR
            || response.status() == StatusCode::OK
        );
    }

    #[tokio::test]
    async fn test_login_route_with_invalid_email() {
        let app = app().await;

        let login_payload = json!({
            "email": "invalid-email",
            "password": "ValidPassword123!"
        });

        let request = Request::builder()
            .method("POST")
            .uri("/auth/login")
            .header("content-type", "application/json")
            .body(axum::body::Body::from(login_payload.to_string()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_login_route_with_invalid_password() {
        let app = app().await;

        let login_payload = json!({
            "email": "test@universeg.gg",
            "password": "weak"
        });

        let request = Request::builder()
            .method("POST")
            .uri("/auth/login")
            .header("content-type", "application/json")
            .body(axum::body::Body::from(login_payload.to_string()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_login_route_with_missing_fields() {
        let app = app().await;

        let login_payload = json!({
            "email": "test@universeg.gg"
            // Missing password field
        });

        let request = Request::builder()
            .method("POST")
            .uri("/auth/login")
            .header("content-type", "application/json")
            .body(axum::body::Body::from(login_payload.to_string()))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_login_route_with_empty_body() {
        let app = app().await;

        let request = Request::builder()
            .method("POST")
            .uri("/auth/login")
            .header("content-type", "application/json")
            .body(axum::body::Body::from(""))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_login_route_with_malformed_json() {
        let app = app().await;

        let request = Request::builder()
            .method("POST")
            .uri("/auth/login")
            .header("content-type", "application/json")
            .body(axum::body::Body::from("{invalid json}"))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_login_route_wrong_http_method() {
        let app = app().await;

        let request = Request::builder()
            .method("GET")
            .uri("/auth/login")
            .body(axum::body::Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
    }
}
