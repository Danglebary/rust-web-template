// region:    module imports and declarations

// external crates
use axum::{extract::Path, http::StatusCode, Json};
use serde_json::{json, Value};

// internal imports
use crate::{Error, Result};

// modules

// self imports and exports

// endregion: module imports and declarations

#[tracing::instrument]
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, body = &'static str)
    )
)]
pub async fn index() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Hello, world!")
}

#[tracing::instrument]
#[utoipa::path(
    get,
    path = "/echo/{name}",
    responses(
        (status = 200, body = String),
        (status = 400, body = String)
    )
)]
pub async fn echo(Path(name): Path<String>) -> Result<Json<Value>> {
    match name.as_str() {
        "error" => return Err(Error::bad_request("name cannot be 'error'".to_string())),
        _ => Ok(json!({
            "message": format!("Hello, {}!", name)
        })
        .into()),
    }
}
