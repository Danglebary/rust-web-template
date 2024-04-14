// region:    module imports and declarations

// external crates
use axum::Router;

// internal imports

// modules

// self imports and exports

// endregion: module imports and declarations

pub(crate) fn add_layers(router: Router) -> Router {
    lib_trace::add_trace_layer(router)
}
