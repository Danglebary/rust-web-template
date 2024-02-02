#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to load application configuration: {0:?}")]
    Config(#[from] crate::config::Error),
    #[error("failed to initialize tracing: {0:?}")]
    Trace(#[from] crate::trace::Error),
    #[error("failed to bind to address: {0:?}")]
    Bind(#[from] std::io::Error),
}

pub type Result<T> = anyhow::Result<T, Error>;
