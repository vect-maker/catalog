use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct AppConfig {
    pub db_url: String,
    pub db_token: String,
    pub client_url: String,
    pub secret_key: String,

    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct AdminConfig {
    #[serde(default = "default_admin_user")]
    pub admin_user: String,

    #[serde(default = "default_admin_password")]
    pub admin_password: String,

    #[serde(default = "default_admin_id")]
    pub admin_id: String,
}

// Default value providers
fn default_admin_user() -> String {
    "admin".to_string()
}
fn default_admin_password() -> String {
    "1234".to_string()
}
fn default_admin_id() -> String {
    "019d83c7-3fe1-791c-851e-15697f8c1b9d".to_string()
}
fn default_port() -> u16 {
    8080
}

pub fn load_configs() -> (AppConfig, AdminConfig) {
    let app =
        envy::from_env::<AppConfig>().expect("❌ CRITICAL: Failed to parse App configuration");

    let admin =
        envy::from_env::<AdminConfig>().expect("❌ CRITICAL: Failed to parse Admin configuration");

    validate_not_empty(&[
        ("DB_URL", &app.db_url),
        ("DB_TOKEN", &app.db_token),
        ("CLIENT_URL", &app.client_url),
        ("SECRET_KEY", &app.secret_key),
    ]);

    (app, admin)
}

fn validate_not_empty(fields: &[(&str, &String)]) {
    let missing: Vec<&str> = fields
        .iter()
        .filter(|(_, val)| val.trim().is_empty())
        .map(|(name, _)| *name)
        .collect();

    if !missing.is_empty() {
        panic!(
            "\n\n❌ CRITICAL STARTUP ERROR:\n\
            The following environment variables are empty strings:\n\
            {:?}\n\n\
            Please check your .env file or container configuration.\n",
            missing
        );
    }
}
