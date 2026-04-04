use crate::AppError;

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use axum::body::Bytes;
use image::ImageFormat;
use std::io::Cursor;
use tokio::task;

use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error,
};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: u64,
    pub exp: usize,
}

pub fn create_jwt(user_id: u64, secret_key: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
        + (24 * 3600);

    let claims = Claims {
        sub: user_id,
        exp: expiration as usize,
    };
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
}

pub fn verify_jwt(token: &str, secret_key: &str) -> Result<Claims, Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = 60;

    let decoding_key = DecodingKey::from_secret(secret_key.as_bytes());

    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;

    Ok(token_data.claims)
}

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}

pub fn verify_password(password: &str, stored_hash: &str) -> Result<(), AppError> {
    let parsed_hash = PasswordHash::new(stored_hash).map_err(|_| AppError::InternalError)?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::InvalidCredentials)?;

    Ok(())
}

pub async fn convert_to_webp(raw_bytes: Bytes) -> Result<Vec<u8>, AppError> {
    let webp_bytes = task::spawn_blocking(move || {
        let img = image::load_from_memory(&raw_bytes).map_err(|_| AppError::InvalidImageFormat)?;

        let mut buffer = Cursor::new(Vec::new());

        img.write_to(&mut buffer, ImageFormat::WebP)
            .map_err(|_| AppError::EncodingFailed)?;

        Ok::<Vec<u8>, AppError>(buffer.into_inner())
    })
    .await
    .map_err(|_| AppError::ThreadPanic)??;
    Ok(webp_bytes)
}
