use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct AppEnv {
    pub db_url: String,
    pub db_token: String,
    pub client_url: String,
    pub secret_key: String,
}

pub fn get_env() -> AppEnv {
    envy::from_env::<AppEnv>().expect("FATAL: Missing required ENV variables")
}
