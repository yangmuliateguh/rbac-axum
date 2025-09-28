use crate::models::UserRole;
use axum::{
    extract::{Extension, Request},
    http::StatusCode,
    middleware::Next,
    response::Response
};
use http::HeaderMap;

pub async fn rbac_middleware(
    headers: HeaderMap,
    Extension(required_role): Extension<UserRole>,
    request: Request,
    next: Next
) -> Result<Response, StatusCode> {
    let auth_header = headers
        .get(http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let token = auth_header
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED);

    let claims = crate::auth::jwt::validate_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if claims.role == UserRole::Admin || claims.role == required_role {
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}