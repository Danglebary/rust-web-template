// region:    module imports and declarations

// external crates
use axum::http::StatusCode;

// internal imports

// modules

// self imports and exports

// endregion: module imports and declarations

#[tracing::instrument]
pub(super) async fn handler() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Hello, world!")
}
