// region:    module imports and declarations

// external crates
use serde::{Deserialize, Serialize};
use std::{net::IpAddr, sync::OnceLock};

// internal imports
use crate::util::git;

// modules
mod constants;
mod env_var;
pub mod error;
pub mod log_level;
pub mod stage;

// self imports and exports
use env_var::EnvVar;
pub use stage::Stage;

#[allow(unused_imports)]
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

#[allow(non_snake_case, clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    // Application Identifiers
    pub APP_NAME: String,
    pub APP_VERSION: String,
    pub APP_SHORT_SHA: String,
    pub APP_STAGE: stage::Stage,

    // Application Secrets (e.g. API keys)

    // Database Configuration
    pub DB_URL: String,
    pub DB_NAME: String,
    pub DB_USER: String,
    pub DB_PASSWORD: String,
    pub DB_POOL_SIZE: u16,

    // Application Settings
    pub APP_LOG_LEVEL: log_level::LogLevel,
    pub APP_HOST: IpAddr,
    pub APP_PORT: u16,
}

impl AppConfig {
    pub fn load_from_env() -> Result<Self> {
        Ok(Self {
            // Application Identifiers
            APP_NAME: EnvVar::from_env::<String>(constants::APP_NAME)?,
            APP_VERSION: EnvVar::from_env::<String>(constants::APP_VERSION)?,
            APP_SHORT_SHA: git::short_sha()?,
            APP_STAGE: stage::Stage::from_env()?,
            // Application Secrets (e.g. API keys)
            DB_URL: EnvVar::from_env::<String>(constants::DB_URL)?,
            DB_NAME: EnvVar::from_env::<String>(constants::DB_NAME)?,
            DB_USER: EnvVar::from_env::<String>(constants::DB_USER)?,
            DB_PASSWORD: EnvVar::from_env::<String>(constants::DB_PASSWORD)?,
            DB_POOL_SIZE: EnvVar::from_env::<u16>(constants::DB_POOL_SIZE)?,
            // Application Settings
            APP_LOG_LEVEL: log_level::LogLevel::from_env()?,
            APP_HOST: EnvVar::from_env::<IpAddr>(constants::APP_HOST)?,
            APP_PORT: EnvVar::from_env::<u16>(constants::APP_PORT)?,
        })
    }
}
