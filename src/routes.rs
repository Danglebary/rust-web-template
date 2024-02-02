// region:    module imports and declarations

// external crates
use axum::{
    http::StatusCode,
    routing::{get, MethodRouter},
    Router,
};

// internal imports

// modules

// self imports and exports

// endregion: module imports and declarations

pub fn build() -> Router {
    Router::new().fallback_service(fallback())
}

fn fallback() -> MethodRouter {
    get(|| async {
        (
            StatusCode::NOT_FOUND,
            "The requested resource was not found.",
        )
    })
}
