use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthenticateUserDto {
    pub username: String,
    pub password: String,
}
