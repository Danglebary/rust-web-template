// region:    module imports and declarations

// external crates

// internal imports
use crate::util::command;

// modules

// self imports and exports

// endregion: module imports and declarations

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("git command failed: {0}")]
    CommandFailed(#[from] command::Error),
    #[error("short sha command returned empty stdout")]
    ShortShaEmptyStdOut,
    #[error("short sha command output could not be parsed")]
    ShortShaParseFailed,
    #[error("short sha command return invalid string '{0}'. {1}")]
    ShortShaInvalid(String, &'static str),
}

pub type Result<T> = anyhow::Result<T, Error>;
