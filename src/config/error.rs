#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("config variable {0} is missing in environment")]
    EnvVarMissing(&'static str),
    #[error("failed to parse environment variable {0} with value '{1}', expected one of {2:?}")]
    ParseEnvVarEnumFailed(&'static str, String, [&'static str; 3]),
    #[error("failed to parse environment variable {0} with value '{1}'")]
    ParseEnvVarFailed(&'static str, String),
    #[error("git command failed: {0}")]
    GitCommandFailed(#[from] crate::util::git::Error),
}

pub type Result<T> = anyhow::Result<T, Error>;
