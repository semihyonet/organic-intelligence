use config::{Config, File};
use dotenvy::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Settings {
    app_name: String,
    debug: bool,
    pub database_url: String,
    api_key: String,
}

pub async fn get_configuration() -> Result<Settings, config::ConfigError> {
    assert_ne!(dotenv().ok(), None); // Load .env file

    let env = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

    let settings = Config::builder()
        .add_source(File::with_name("config/development").required(false))
        .add_source(File::with_name(&format!("config/{}", env)).required(false))
        .add_source(config::Environment::default()) // Override with env vars
        .build()
        .expect("Failed to load configuration");

    Ok(settings.try_deserialize().expect("Failed to deserialize settings"))
}


