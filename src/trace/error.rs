// region:    module imports and declarations

// external crates
use tracing::subscriber::SetGlobalDefaultError;

// internal imports

// modules

// self imports and exports

// endregion: module imports and declarations

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to set global default subscriber: {0}")]
    SetGlobalDefaultSubscriberFailed(#[from] SetGlobalDefaultError),
}

pub type Result<T> = anyhow::Result<T, Error>;
