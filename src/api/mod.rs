// region:    module imports and declarations

// external crates
use axum::Router;

// internal imports

// modules
mod controller;
pub mod error;
mod fallback;
mod layer;
pub mod open_api;

// self imports and exports
pub use error::{ApiError, ApiResult, RouterError, RouterResult};

// endregion: module imports and declarations

// This is not actually unused, but rust-analyzer doesn't understand because
// this crate is a dependency of both the main application and the `gen-openapi` binary
// I don't like seeing the warning, so I'm disabling it
// TODO: replace anyhow::Result with custom errors
#[allow(unused)]
pub fn build_router() -> RouterResult<Router> {
    let mut router = Router::new();

    // Add API controller routes
    router = controller::add_routes(router);

    // Add fallback route
    router = fallback::add_fallback(router);

    // Add layers
    router = layer::add_layers(router);

    // Add Swagger UI
    router = open_api::add_swagger_ui(router)?;

    Ok(router)
}
