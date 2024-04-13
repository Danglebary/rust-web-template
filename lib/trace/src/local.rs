// region:    module imports and declarations

// external crates
use tracing::Level;

// internal imports
use crate::Result;

// modules

// self imports and exports

// endregion: module imports and declarations

pub(crate) fn init_tracing_local(log_level: Level) -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .pretty()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_max_level(log_level)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
