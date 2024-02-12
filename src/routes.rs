// region:    module imports and declarations

// external crates
use axum::{
    extract::MatchedPath,
    http::{Request, StatusCode},
    routing::{get, MethodRouter},
    Router,
};
use tower_http::trace::TraceLayer;
use tracing::info_span;

// internal imports

// modules

// self imports and exports

// endregion: module imports and declarations

#[tracing::instrument]
async fn hello_world() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Hello, world!")
}

pub fn build() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .fallback_service(fallback())
        .layer(TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
            // Log the matched route's path (with placeholders not yet replaced)
            // Use `request.uri()` to log the full path
            let matched_path = request
                .extensions()
                .get::<MatchedPath>()
                .map(MatchedPath::as_str);

            info_span!("http_request", method = ?request.method(), matched_path, headers = ?request.headers())
        }))
}

fn fallback() -> MethodRouter {
    get(|| async {
        (
            StatusCode::NOT_FOUND,
            "The requested resource was not found.",
        )
    })
}
