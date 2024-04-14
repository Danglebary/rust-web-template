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
    #[error("Failed to create database connection pool: {0}")]
    ConnectionPool(String),
}

pub type Result<T> = AnyResult<T, Error>;
