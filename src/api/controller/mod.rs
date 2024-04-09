// region:    module imports and declarations

// external crates
use axum::{routing::get, Router};

// internal imports

// modules
mod health;
mod hello_world;

// self imports and exports

// endregion: module imports and declarations

pub(super) fn add_routes(router: Router) -> Router {
    router
        .route("/healthz", get(health::healthz))
        .route("/readyz", get(health::readyz))
        .route("/", get(hello_world::handler))
}
