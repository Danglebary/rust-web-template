// region:    module imports and declarations

// external crates
use serde::{Deserialize, Serialize};
use std::{net::IpAddr, sync::OnceLock};

// internal imports

// modules
mod constants;
mod env_var;
pub mod error;
pub mod log_level;
pub mod stage;

// self imports and exports
use env_var::EnvVar;
pub use stage::Stage;

pub use error::{Error, Result};

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
#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    // Application Identifiers
    pub APP_NAME: String,
    pub APP_VERSION: String,
    pub APP_STAGE: stage::Stage,

    // Application Settings
    pub APP_LOG_LEVEL: log_level::LogLevel,
    pub APP_HOST: IpAddr,
    pub APP_PORT: u16,

    // Observability Settings
    pub OBSERVABILITY_API_KEY_HEADER: String,
    pub OBSERVABILITY_API_KEY: String,
    pub OBSERVABILITY_API_INGEST_URL: String,
}

impl AppConfig {
    pub fn load_from_env() -> Result<Self> {
        Ok(Self {
            // Application Identifiers
            APP_NAME: EnvVar::from_env::<String>(constants::APP_NAME)?,
            APP_VERSION: EnvVar::from_env::<String>(constants::APP_VERSION)?,
            APP_STAGE: stage::Stage::from_env()?,
            // Application Settings
            APP_LOG_LEVEL: log_level::LogLevel::from_env()?,
            APP_HOST: EnvVar::from_env::<IpAddr>(constants::APP_HOST)?,
            APP_PORT: EnvVar::from_env::<u16>(constants::APP_PORT)?,
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
