// region:    module imports and declarations

// external crates
use axum::Router;
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;

// internal imports
use lib_config::config;

// modules

// self imports and exports

// endregion: module imports and declarations

// TODO: let's revist how we add layers

pub(crate) fn add_layers(router: Router) -> Router {
    let router = lib_trace::add_trace_layer(router);
    add_timeout_layer(router)
}

fn add_timeout_layer(router: Router) -> Router {
    router.layer(TimeoutLayer::new(Duration::from_secs(
        config().APP_API_TIMEOUT_SECS.into(),
    )))
}
