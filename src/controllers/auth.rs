// src/controllers/auth.rs
use axum::{Json, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
struct AuthedUser {
    message: String,
}

pub(crate) async fn login() -> impl IntoResponse {
    Json(AuthedUser {
        message: "Login here".to_string(),
    })
}

pub(crate) async fn register() -> impl IntoResponse {
    Json(AuthedUser {
        message: "Register".to_string(),
    })
}
