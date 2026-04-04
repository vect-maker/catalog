use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AuthenticateUserDto {
    pub name: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthTokenResponseDto {
    pub token: String,
}
