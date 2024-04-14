// region:    module imports and declarations

// external crates
use axum::http::StatusCode;

// internal imports

// modules

// self imports and exports

// endregion: module imports and declarations

#[tracing::instrument]
pub(super) async fn healthz() -> StatusCode {
    StatusCode::OK
}

#[tracing::instrument]
pub(super) async fn readyz() -> StatusCode {
    // TODO: add readiness checks here
    StatusCode::OK
}
