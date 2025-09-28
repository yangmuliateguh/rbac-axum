use crate::models::Claims;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use std::env;

static JWT_SECRET: Lazy<Vec<u8>> = Lazy::new(|| {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET harus disetel di environment");
    secret.into_bytes()
});

pub fn create_token(
    username: String, role: crate::models::UserRole
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() + 3600;

    let claims = Claims { sub: username, role, exp: expiration as usize };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET))
}

pub fn validate_token(
    token: &str
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &validation
    )?;
    Ok(token_data.claims)
}