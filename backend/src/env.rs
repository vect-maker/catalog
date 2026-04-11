use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct AppEnv {
    pub db_url: String,
    pub db_token: String,
    pub client_url: String,
    pub secret_key: String,
    pub port: u16,
}

impl AppEnv {
    pub fn load() -> Self {
        let env = envy::from_env::<AppEnv>().expect("Failed to load environment variables");
        let missing_fields: Vec<&str> = [
            ("DB_URL", &env.db_url),
            ("DB_TOKEN", &env.db_token),
            ("CLIENT_URL", &env.client_url),
            ("SECRET_KEY", &env.secret_key),
        ]
        .iter()
        .filter(|(_, val)| val.trim().is_empty())
        .map(|(name, _)| *name)
        .collect();

        if !missing_fields.is_empty() {
            panic!(
                "\n\n❌ CRITICAL STARTUP ERROR:\n\
                The following environment variables are empty strings:\n\
                {:?}\n\n\
                Please check your Render Dashboard or .env file.\n",
                missing_fields
            );
        }

        env
    }
}
