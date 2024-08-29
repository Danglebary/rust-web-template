// region:    module imports and declarations

// external crates
use axum::{
    body::Body,
    extract::MatchedPath,
    http::{Request, Response},
    Router,
};
use std::time::Duration;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{error, info, warn, warn_span, Span};
use uuid::Uuid;

// internal imports

// modules

// self imports and exports

// endregion: module imports and declarations

// TODO: let's revisit this implementation
// This is likely more complex than necessary

pub fn add_trace_layer(router: Router) -> Router {
    let layer = TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
        let request_id = Uuid::new_v4().to_string();

        let matched_path = request
            .extensions()
            .get::<MatchedPath>()
            .map(MatchedPath::as_str);

        warn_span!("http_request", %request_id, matched_path, method = %request.method(), headers = ?request.headers())
    }).on_request(|req: &Request<Body>, span: &Span| {
        info!(parent: span, message = "request started", method = %req.method(), uri = %req.uri(), headers = ?req.headers());
    })
    .on_response(|res: &Response<_>, latency: Duration, span: &Span| {
        let status = res.status().as_u16();

        match status {
            200..=299 => info!(parent: span, message = "request successful", status = status, ?latency, headers = ?res.headers()),
            400..=499 => warn!(parent: span, message = "request client error", status = status, ?latency, headers = ?res.headers()),
            _ => {},
        }
    })
    .on_failure(|error: ServerErrorsFailureClass, latency: Duration, span: &Span| {
        error!(parent: span, message = "request failed", %error, ?latency);
    });

    router.layer(layer)
}
