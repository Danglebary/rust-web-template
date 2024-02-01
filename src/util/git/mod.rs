// region:    module imports and declarations

// external crates

// internal imports
use crate::util::command;

// modules
pub mod error;

// self imports and exports
pub use error::{Error, Result};

// endregion: module imports and declarations

pub fn short_sha() -> Result<String> {
    let output = command::run("git", &["rev-parse", "--short", "HEAD"])?;

    if output.stdout.is_empty() {
        Err(Error::ShortShaEmptyStdOut)?;
    }

    let mut short_sha = String::from_utf8(output.stdout).map_err(|_| Error::ShortShaParseFailed)?;

    // Before we check length, we need to remove the trailing newline.
    // It is safe to unwrap here because we already checked that the string is not empty.
    #[allow(clippy::unwrap_used)]
    short_sha.pop().unwrap();

    if short_sha.len() != 7 {
        Err(Error::ShortShaInvalid(short_sha.clone(), "expected 7 characters"))?;
    }

    Ok(short_sha)
}
