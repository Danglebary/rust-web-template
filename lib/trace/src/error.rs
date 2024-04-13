// region:    module imports and declarations

// external crates
use anyhow::Result as AnyResult;
use opentelemetry::trace::TraceError;
use thiserror::Error as ThisError;
use tracing::subscriber::SetGlobalDefaultError;

// internal imports

// modules

// self imports and exports

// endregion: module imports and declarations

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("failed to set global default subscriber: {0}")]
    SetGlobalDefaultSubscriberFailed(#[from] SetGlobalDefaultError),
    #[error("failed to initialize otlp tracing pipeline: {0}")]
    OtlpTracingPipelineInitFailed(#[from] TraceError),
}

pub type Result<T> = AnyResult<T, Error>;
