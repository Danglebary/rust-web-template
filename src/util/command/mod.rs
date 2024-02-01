// region:    module imports and declarations

// external crates
use std::process::{Command, Output};

// internal imports

// modules
pub mod error;

// self imports and exports
pub use error::{Error, Result};

// endregion: module imports and declarations

pub fn run(command: &'static str, args: &[&'static str]) -> Result<Output> {
    Command::new(command)
        .args(args)
        .output()
        .map_err(|err| Error {
            command,
            args: args.to_vec(),
            cause: err.to_string(),
        })
}
