// region:    module imports and declarations

// external crates
use axum::{extract::Path, http::StatusCode};

// internal imports

// modules

// self imports and exports
use super::{ApiError, Result};

// endregion: module imports and declarations

#[tracing::instrument]
pub(super) async fn handler() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Hello, world!")
}

#[tracing::instrument]
pub(super) async fn echo(Path(name): Path<String>) -> Result<(StatusCode, String)> {
    match name.as_str() {
        "error" => Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "bad request: name cannot be 'error'",
        )),
        _ => Ok((StatusCode::OK, name)),
    }
}
