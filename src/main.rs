// region:    module imports and declarations

// external crate
use std::net::SocketAddr;
use tokio::signal;
use tracing::{debug, info};

// internal imports

// modules
mod error;

// self imports and exports
use error::*;

// endregion: module imports and declarations

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize application configuration
    let config = lib_config::config();

    // Initialize tracing
    lib_trace::init_tracing()?;
    debug!("Tracing initialized");

    // Build the API router
    debug!("Building API router");
    let router = lib_api::build_router().await?;
    debug!("API router built");

    // Setup TCP listener and serve the API
    debug!("Setting up TCP listener");
    let addr = SocketAddr::from((config.APP_HOST, config.APP_PORT));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    debug!("TCP listener setup");

    info!("Server listening on: {}", config.APP_PORT);
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
