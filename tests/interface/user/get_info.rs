use axum::Router;
use http::{Request, StatusCode, header};
use tower::ServiceExt;
use uuid::Uuid;
use std::env;
use dotenvy::dotenv;
use universeg_api::infrastructure::security::jwt::Hs256Jwt;
use universeg_api::app::ports::JwtService;
use crate::common;

async fn app_with_env() -> Router {
    common::build_test_app().await.unwrap()
}

#[tokio::test]
async fn get_info_requires_auth() {
    let app = app_with_env().await;

    let res = app
        .clone()
        .oneshot(
            Request::get("/user/info")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn get_info_with_invalid_token() {
    let app = app_with_env().await;

    let req = Request::get("/user/info")
        .header(header::AUTHORIZATION, "Bearer nope")
        .body(axum::body::Body::empty())
        .unwrap();

    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn get_info_with_valid_token() {
    dotenv().ok();
    let uid_str = env::var("UID").expect("UID no definido");
    let uid = Uuid::parse_str(&uid_str).expect("UID inválido (debe ser uuid v4)");
    let app = app_with_env().await;

    let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());

    let jwt = Hs256Jwt::new(&jwt_secret, 1);

    let token = jwt.sign(uid).await.unwrap();

    let req = Request::get("/user/info")
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .body(axum::body::Body::empty())
        .unwrap();

    let res = app.clone().oneshot(req).await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
}
