// region:    module imports and declarations

// external crates
use axum::{http::StatusCode, Router};

// internal imports

// modules

// self imports and exports

// endregion: module imports and declarations

pub(super) fn add_fallback(router: Router) -> Router {
    router.fallback(|| async {
        (
            StatusCode::NOT_FOUND,
            "The requested resource was not found.",
        )
    })
}
