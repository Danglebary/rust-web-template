#[derive(thiserror::Error, Debug)]
#[error("command {command} {args:?} failed: {cause}")]
pub struct Error {
    pub command: &'static str,
    pub args: Vec<&'static str>,
    pub cause: String,
}

pub type Result<T> = anyhow::Result<T, Error>;
