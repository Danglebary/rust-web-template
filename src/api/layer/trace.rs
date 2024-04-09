// region:    module imports and declarations

// external crates
use axum::{body::Body, extract::MatchedPath, http::Request, Router};
use tower_http::trace::TraceLayer;
use tracing::info_span;
use uuid::Uuid;

// internal imports

// modules

// self imports and exports

// endregion: module imports and declarations

/// Create a trace layer for the server that creates a custom span for each incoming request
pub(super) fn add_trace_layer(router: Router) -> Router {
    let trace_layer = TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
        let request_id = Uuid::new_v4().to_string();

        let matched_path = request
            .extensions()
            .get::<MatchedPath>()
            .map(MatchedPath::as_str);

        info_span!("http_request", %request_id, matched_path, method = %request.method(), headers = ?request.headers())
    });

    router.layer(trace_layer)
}
