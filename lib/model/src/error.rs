// region:    module imports and declarations

// external crates
use anyhow::Result as AnyResult;
use serde_valid::validation::Errors as ValidationErrors;
use sqlx::Error as DbError;
use thiserror::Error as ThisError;

// internal imports
use lib_store::Error as StoreError;

// modules

// self imports and exports

// endregion: module imports and declarations

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Database store error: {0}")]
    DbStore(#[from] StoreError),

    #[error("Database error: {0}")]
    Db(#[from] DbError),

    #[error("validation error: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("entity {0} with id {1} not found")]
    EntityNotFound(&'static str, u64),
}

pub type Result<T> = AnyResult<T, Error>;
