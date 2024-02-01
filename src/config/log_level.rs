// region:    module imports and declarations

// external crates
use serde::{Deserialize, Serialize};

// internal imports
use super::{
    constants,
    error::{Error, Result},
};

// modules

// self imports and exports

// endregion: module imports and declarations

#[derive(Serialize, Deserialize, Debug)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    pub fn from_env() -> Result<Self> {
        let from_env = std::env::var(constants::APP_LOG_LEVEL)
            .map_err(|_| Error::EnvVarMissing(constants::APP_LOG_LEVEL))?;

        match from_env.to_lowercase().as_str() {
            "error" => Ok(Self::Error),
            "warn" => Ok(Self::Warn),
            "info" => Ok(Self::Info),
            "debug" => Ok(Self::Debug),
            "trace" => Ok(Self::Trace),
            _ => Err(Error::ParseEnvVarFailed(constants::APP_LOG_LEVEL, from_env)),
        }
    }

    pub const fn to_tracing_level(&self) -> tracing::Level {
        match self {
            Self::Error => tracing::Level::ERROR,
            Self::Warn => tracing::Level::WARN,
            Self::Info => tracing::Level::INFO,
            Self::Debug => tracing::Level::DEBUG,
            Self::Trace => tracing::Level::TRACE,
        }
    }
}
