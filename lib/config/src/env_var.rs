// region:    module imports and declarations

// external crates
use std::{env, str::FromStr};

// internal imports
use crate::{Error, Result};

// modules

// self imports and exports

// endregion: module imports and declarations

pub struct EnvVar;

impl EnvVar {
    pub fn from_env<T>(key: &'static str) -> Result<T>
    where
        T: FromStr,
    {
        let from_env = env::var(key).map_err(|_| Error::EnvVarMissing(key))?;

        from_env
            .parse::<T>()
            .map_err(|_| Error::ParseEnvVarFailed(key, from_env))
    }
}
