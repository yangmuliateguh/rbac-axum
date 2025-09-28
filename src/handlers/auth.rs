use crate::models::UserRole;
use crate::auth::jwt::create_token;
use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn login_handler(
    Json(payload): Json<LoginRequest>
) -> Result<Json<LoginResponse>, StatusCode> {
    let (valid, role) = match payload.username.as_str() {
        "admin" => (payload.password == "admin123", UserRole::Admin),
        "user" => (payload.password == "user123", UserRole::User),
        _ => (false, UserRole::User),
    };

    if !valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = create_token(payload.username, role)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(LoginResponse { token }))
}