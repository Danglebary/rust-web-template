// region:    module imports and declarations

// external crates
use tracing::info;

// internal imports

// modules
mod config;
mod error;
mod trace;
mod util;

// self imports and exports
pub use config::config;
pub use error::{Error, Result};

// endregion: module imports and declarations

fn main() -> Result<()> {
    // Initialize application configuration
    let config = config::config();

    // Initialize tracing
    trace::init_tracing()?;

    info!("Hello, World!");
    info!("Application configuration: {:#?}", config);

    Ok(())
}
