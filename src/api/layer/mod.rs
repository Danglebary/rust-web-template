// region:    module imports and declarations

// external crates
use axum::Router;

// internal imports

// modules
mod trace;

// self imports and exports

// endregion: module imports and declarations

pub(super) fn add_layers(router: Router) -> Router {
    router.layer(trace::trace_layer())
}
