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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Stage {
    Local,
    Staging,
    Production,
}

impl Stage {
    pub fn from_env() -> Result<Self> {
        let from_env = std::env::var(constants::APP_STAGE)
            .map_err(|_| Error::EnvVarMissing(constants::APP_STAGE))?;

        match from_env.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "staging" => Ok(Self::Staging),
            "production" => Ok(Self::Production),
            _ => Err(Error::ParseEnvVarEnumFailed(
                constants::APP_STAGE,
                from_env,
                ["local", "staging", "production"],
            )),
        }
    }
}

impl std::fmt::Display for Stage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Local => write!(f, "local"),
            Self::Staging => write!(f, "staging"),
            Self::Production => write!(f, "production"),
        }
    }
}
