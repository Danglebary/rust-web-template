// region:    module imports and declarations

// external crates
use std::net::SocketAddr;
use tracing::debug;

// internal imports

// modules
mod config;
mod controller;
mod error;
mod trace;
mod util;

// self imports and exports
pub use config::config;
pub use error::{Error, Result};

// endregion: module imports and declarations

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize application configuration
    let config = config::config();

    // Initialize tracing
    trace::init_tracing()?;

    debug!("Application configuration: {:#?}", config);

    let app = controller::build();

    let addr = SocketAddr::from((config.APP_HOST, config.APP_PORT));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
