// region:    module imports and declarations

// external crates

// internal imports
use lib_config::{config, Stage};

// modules
mod axum;
pub mod error;
mod external;
mod local;

// self imports and exports
pub use axum::add_trace_layer;
pub use error::*;
use external::init_tracing_external;
use local::init_tracing_local;

// endregion: module imports and declarations

/// # Panics
/// Panics if using otlp tracing, and the application configuration for the observability API key is not a valid ASCII string.
pub fn init_tracing() -> Result<()> {
    let level = config().APP_LOG_LEVEL;

    if config().APP_STAGE == Stage::Local {
        Ok(init_tracing_local(level)?)
    } else {
        Ok(init_tracing_external(level)?)
    }
}
