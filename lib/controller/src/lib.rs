// region:    module imports and declarations

// external crates
use axum::{routing::get, Router};

// internal imports
use lib_model::ModelManager;

// modules
pub mod error;
mod health;

// Any controller that utilizes `utoipa` must be public
pub mod todo;

// self imports and exports
pub use error::*;

// endregion: module imports and declarations

pub async fn add_routes(router: Router) -> Router {
    let mm = ModelManager::new()
        .await
        .expect("Failed to create ModelManager");

    // Add routes not related to API controllers
    let router = router
        .route("/healthz", get(health::healthz))
        .route("/readyz", get(health::readyz));

    // Add API controllers
    let mut api_v1 = Router::new();

    let todo_routes = todo::get_routes();

    // Merge all API controllers
    api_v1 = api_v1.nest("/todo", todo_routes);

    // Nest API controllers under /v1
    router.nest("/v1", api_v1.with_state(mm))
}
