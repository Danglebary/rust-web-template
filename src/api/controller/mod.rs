// region:    module imports and declarations

// external crates
use axum::{routing::get, Router};

// internal imports

// modules
mod health;
// all API controllers must be public for use with openapi
pub mod hello_world;

// self imports and exports
pub use super::{ApiError, ApiResult};

// endregion: module imports and declarations

pub(super) fn add_routes(router: Router) -> Router {
    // Add routes not related to API controllers
    let router = router
        .route("/healthz", get(health::healthz))
        .route("/readyz", get(health::readyz));

    // Create API controller router
    let api_v1 = Router::new()
        .route("/", get(hello_world::handler))
        .route("/echo/:name", get(hello_world::echo));

    // Nest API controllers under /v1
    router.nest("/v1", api_v1)
}
