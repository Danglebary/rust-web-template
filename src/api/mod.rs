// region:    module imports and declarations

// external crates
use axum::Router;

// internal imports

// modules
mod controller;
mod fallback;
mod layer;

// self imports and exports

// endregion: module imports and declarations

pub fn build_router() -> Router {
    let mut router = Router::new();

    // Add API controller routes
    router = controller::add_routes(router);

    // Add fallback route
    router = fallback::add_fallback(router);

    // Add layers
    router = layer::add_layers(router);

    router
}
