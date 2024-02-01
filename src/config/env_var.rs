// region:    module imports and declarations

// external crates

// internal imports
use super::error::{Error, Result};

// modules

// self imports and exports

// endregion: module imports and declarations

pub struct EnvVar;

impl EnvVar {
    pub fn from_env<T>(key: &'static str) -> Result<T>
    where
        T: std::str::FromStr,
    {
        let from_env = std::env::var(key).map_err(|_| Error::EnvVarMissing(key))?;

        from_env
            .parse::<T>()
            .map_err(|_| Error::ParseEnvVarFailed(key, from_env))
    }
}
