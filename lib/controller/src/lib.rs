// region:    module imports and declarations

// external crates
use axum::{
    routing::{get, post},
    Router,
};

// internal imports

// modules
pub mod error;
mod health;

// Any controller that utilizes `utoipa` must be public
pub mod hello;
pub mod todo;

// self imports and exports
pub use error::*;
use lib_model::ModelManager;

// endregion: module imports and declarations

pub async fn add_routes(router: Router) -> Router {
    let mm = ModelManager::new()
        .await
        .expect("Failed to create ModelManager");

    // Add routes not related to API controllers
    let router = router
        .route("/healthz", get(health::healthz))
        .route("/readyz", get(health::readyz));

    // Create API controller router
    let mut api_v1 = Router::new()
        .route("/", get(hello::index))
        .route("/echo/:name", get(hello::echo));

    api_v1 = api_v1.route("/todo", post(todo::create_todo).with_state(mm.clone()));

    // Nest API controllers under /v1
    router.nest("/v1", api_v1)
}
