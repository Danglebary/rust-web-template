// region:    module imports and declarations

// external crates
use axum::{extract::Path, http::StatusCode};

// internal imports

// modules

// self imports and exports
use super::{ApiError, ApiResult};

// endregion: module imports and declarations

/// Simple hello world endpoint
#[tracing::instrument]
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, body = &'static str)
    )
)]
pub(super) async fn handler() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Hello, world!")
}

/// Simple echo endpoint that returns the name passed in the path
#[tracing::instrument]
#[utoipa::path(
    get,
    path = "/echo/{name}",
    responses(
        (status = 200, body = String, description = "Echoed name"),
        (status = 400, body = String, description = "Bad request: name cannot be 'error'")
    ),
    params(
        ("name" = String, Path, description = "Name to echo back")
    ),
)]
pub(super) async fn echo(Path(name): Path<String>) -> ApiResult<(StatusCode, String)> {
    match name.as_str() {
        "error" => Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "bad request: name cannot be 'error'",
        )),
        _ => Ok((StatusCode::OK, format!("Hello, {}!", name))),
    }
}
