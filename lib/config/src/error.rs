// region:    module imports and declarations

// external crates
use anyhow::Result as AnyResult;
use thiserror::Error as ThisError;

// internal imports

// modules

// self imports and exports

// endregion: module imports and declarations

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("config variable {0} is missing in environment")]
    EnvVarMissing(&'static str),
    #[error("failed to parse environment variable {0} with value '{1}', expected one of {2:?}")]
    ParseEnvVarEnumFailed(&'static str, String, [&'static str; 3]),
    #[error("failed to parse environment variable {0} with value '{1}'")]
    ParseEnvVarFailed(&'static str, String),
}

pub type Result<T> = AnyResult<T, Error>;
