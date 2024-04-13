// region:    module imports and declarations

// external crates
use axum::{routing::get, Router};

// internal imports

// modules
pub mod error;
mod health;

// Any controller that utilizes `utoipa` must be public
pub mod hello;

// self imports and exports
pub use error::*;

// endregion: module imports and declarations

pub fn add_routes(router: Router) -> Router {
    // Add routes not related to API controllers
    let router = router
        .route("/healthz", get(health::healthz))
        .route("/readyz", get(health::readyz));

    // Create API controller router
    let api_v1 = Router::new()
        .route("/", get(hello::index))
        .route("/echo/:name", get(hello::echo));

    // Nest API controllers under /v1
    router.nest("/v1", api_v1)
}
