// region:    module imports and declarations

// external crates
use tracing::{Level, Subscriber};

// internal imports
use crate::{config, config::Stage};

// modules
pub mod error;

// self imports and exports
pub use error::{Error, Result};

// endregion: module imports and declarations

fn init_tracing_local(log_level: Level) -> impl Subscriber + Send + Sync + 'static {
    tracing_subscriber::fmt()
        .pretty()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_max_level(log_level)
        .finish()
}

pub fn init_tracing() -> Result<()> {
    let subscriber = match config().APP_STAGE {
        Stage::Production => todo!("init_tracing_production()"),
        Stage::Staging => todo!("init_tracing_staging()"),
        Stage::Local => init_tracing_local(config().APP_LOG_LEVEL.to_tracing_level()),
    };

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
