// region:    module imports and declarations

// external crates
use std::{net::IpAddr, sync::OnceLock};
use tracing::Level;

// internal imports

// modules
mod constants;
pub mod env_var;
pub mod error;
pub mod stage;

// self imports and exports
pub use env_var::*;
pub use error::*;
pub use stage::*;

// endregion: module imports and declarations

/// # Panics
/// Panics if the application configuration fails to load from environment variables.
pub fn config() -> &'static AppConfig {
    static INSTANCE: OnceLock<AppConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        AppConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("Failed to load application configuration: {ex:?}"))
    })
}

#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct AppConfig {
    // Application Identifiers
    pub APP_NAME: String,
    pub APP_VERSION: String,
    pub APP_STAGE: stage::Stage,

    // Application Settings
    pub APP_LOG_LEVEL: Level,
    pub APP_HOST: IpAddr,
    pub APP_PORT: u16,

    // Database Settings
    pub DB_NAME: String,
    pub DB_HOST: String,
    pub DB_PORT: u16,
    pub DB_USER: String,
    pub DB_PASSWORD: String,
    pub DB_POOL_MIN_CONNECTIONS: u32,
    pub DB_POOL_MAX_CONNECTIONS: u32,
    pub DB_IDLE_TIMEOUT_MILLIS: u64,
    pub DB_AQUIRE_TIMEOUT_MILLIS: u64,

    // Observability Settings
    pub OBSERVABILITY_API_KEY_HEADER: String,
    pub OBSERVABILITY_API_KEY: String,
    pub OBSERVABILITY_API_INGEST_URL: String,
}

impl AppConfig {
    pub fn load_from_env() -> Result<Self> {
        // If the SERVICE_APP_ENV var is not set,
        // we are in a local environment and should load the .env file
        if std::env::var("SERVICE_APP_ENV").is_err() {
            dotenvy::dotenv().expect("Failed to load .env file");
        }

        Ok(Self {
            // Application Identifiers
            APP_NAME: EnvVar::from_env::<String>(constants::APP_NAME)?,
            APP_VERSION: EnvVar::from_env::<String>(constants::APP_VERSION)?,
            APP_STAGE: stage::Stage::from_env()?,

            // Application Settings
            APP_LOG_LEVEL: EnvVar::from_env::<Level>(constants::APP_LOG_LEVEL)?,
            APP_HOST: EnvVar::from_env::<IpAddr>(constants::APP_HOST)?,
            APP_PORT: EnvVar::from_env::<u16>(constants::APP_PORT)?,

            // Database Settings
            DB_NAME: EnvVar::from_env::<String>(constants::DB_NAME)?,
            DB_HOST: EnvVar::from_env::<String>(constants::DB_HOST)?,
            DB_PORT: EnvVar::from_env::<u16>(constants::DB_PORT)?,
            DB_USER: EnvVar::from_env::<String>(constants::DB_USER)?,
            DB_PASSWORD: EnvVar::from_env::<String>(constants::DB_PASSWORD)?,
            DB_POOL_MIN_CONNECTIONS: EnvVar::from_env::<u32>(constants::DB_POOL_MIN_CONNECTIONS)?,
            DB_POOL_MAX_CONNECTIONS: EnvVar::from_env::<u32>(constants::DB_POOL_MAX_CONNECTIONS)?,
            DB_IDLE_TIMEOUT_MILLIS: EnvVar::from_env::<u64>(constants::DB_IDLE_TIMEOUT_MILLIS)?,
            DB_AQUIRE_TIMEOUT_MILLIS: EnvVar::from_env::<u64>(constants::DB_AQUIRE_TIMEOUT_MILLIS)?,

            // Observability Settings
            OBSERVABILITY_API_KEY_HEADER: EnvVar::from_env::<String>(
                constants::OBSERVABILITY_API_KEY_HEADER,
            )?,
            OBSERVABILITY_API_KEY: EnvVar::from_env::<String>(constants::OBSERVABILITY_API_KEY)?,
            OBSERVABILITY_API_INGEST_URL: EnvVar::from_env::<String>(
                constants::OBSERVABILITY_API_INGEST_URL,
            )?,
        })
    }
}
