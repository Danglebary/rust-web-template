// region:    module imports and declarations

// external crates
use std::net::SocketAddr;

// internal imports

// modules
mod api;
mod config;
mod error;
mod trace;

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

    // Build the API router
    let router = api::build_router().map_err(|err| Error::Router(err))?;

    // Setup TCP listener and serve the API
    let addr = SocketAddr::from((config.APP_HOST, config.APP_PORT));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}
