// region:    module imports and declarations

// external crates
use anyhow::Result as AnyResult;
use std::io::Error as IoError;
use thiserror::Error as ThisError;

// internal imports
use lib_api::Error as ApiError;
use lib_config::Error as ConfigError;
use lib_trace::Error as TraceError;

// modules

// self imports and exports

// endregion: module imports and declarations

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("failed to build API router: {0:?}")]
    Api(#[from] ApiError),
    #[error("failed to load application configuration: {0:?}")]
    Config(#[from] ConfigError),
    #[error("failed to bind listener to address: {0:?}")]
    ListenerBind(#[from] IoError),
    #[error("failed to initialize tracing: {0:?}")]
    Trace(#[from] TraceError),
}

pub type Result<T> = AnyResult<T, Error>;
