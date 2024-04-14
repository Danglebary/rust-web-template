// region:    module imports and declarations

// external crate
use std::net::SocketAddr;

// internal imports
use lib_config::config;

// modules
mod error;

// self imports and exports
use error::*;

// endregion: module imports and declarations

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize application configuration
    let config = config();

    // Initialize tracing
    lib_trace::init_tracing()?;

    // Build the API router
    let router = lib_api::build_router()?;

    // Setup TCP listener and serve the API
    let addr = SocketAddr::from((config.APP_HOST, config.APP_PORT));
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    axum::serve(listener, router).await?;

    Ok(())
}
