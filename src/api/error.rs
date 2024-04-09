// region:    module imports and declarations

// external crates
use axum::{http::StatusCode, response::IntoResponse};

// internal imports

// modules

// self imports and exports

// endregion: module imports and declarations

#[derive(thiserror::Error, Debug)]
pub struct ApiError {
    pub code: StatusCode,
    pub message: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "STATUS {}: {}", self.code, self.message)
    }
}

impl ApiError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.code, self.message).into_response()
    }
}

pub type Result<T> = anyhow::Result<T, ApiError>;