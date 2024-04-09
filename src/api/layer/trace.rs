// region:    module imports and declarations

// external crates
use axum::{body::Body, extract::MatchedPath, http::Request};
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::TraceLayer,
};
use tracing::info_span;

// internal imports

// modules

// self imports and exports

// endregion: module imports and declarations

fn make_custom_span(request: &Request<Body>) -> tracing::Span {
    // Log the matched route's path (with placeholders not yet replaced)
    // Use `request.uri()` to log the full path
    let matched_path = request
        .extensions()
        .get::<MatchedPath>()
        .map(MatchedPath::as_str);

    info_span!("http_request", method = ?request.method(), matched_path, headers = ?request.headers())
}

pub(super) fn trace_layer() -> TraceLayer<
    SharedClassifier<ServerErrorsAsFailures>,
    for<'a> fn(&'a Request<Body>) -> tracing::Span,
> {
    TraceLayer::new_for_http().make_span_with(make_custom_span)
}
