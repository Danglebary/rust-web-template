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
    #[error("Failed to open openapi spec file: {0}")]
    OpenApiFileError(#[from] std::io::Error),
    #[error("Failed to parse openapi spec file: {0}")]
    OpenApiParseError(#[from] serde_json::Error),
}

pub type Result<T> = AnyResult<T, Error>;
