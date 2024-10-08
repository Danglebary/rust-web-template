// region:    module imports and declarations

// external crates
use axum::Router;

// internal imports

// modules
pub mod error;
mod fallback;
mod layers;
mod swagger;

// self imports and exports
pub use error::*;
use layers::add_layers;

// endregion: module imports and declarations

pub async fn build_router() -> Result<Router> {
    let mut router = Router::new();

    // Add API controller routes
    router = lib_controller::add_routes(router).await;

    // Add fallback route
    router = fallback::add_fallback(router);

    // Add layers
    router = add_layers(router);

    // Add Swagger UI
    router = swagger::add_swagger_ui(router)?;

    Ok(router)
}
