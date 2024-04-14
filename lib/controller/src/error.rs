// region:    module imports and declarations

// external crates
use anyhow::{Error as AnyError, Result as AnyResult};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

// internal imports

// modules

// self imports and exports

// endregion: module imports and declarations

#[derive(Debug)]
pub struct Error {
    pub status: StatusCode,
    pub message: String,
}

impl Error {
    pub fn new(status: StatusCode, message: String) -> Self {
        Self { status, message }
    }

    pub fn from_internal_error(error: AnyError) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }

    pub fn internal() -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal Server Error".to_string(),
        }
    }

    pub fn not_found(entity_name: String) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: format!("{} not found", entity_name),
        }
    }

    pub fn bad_request(message: String) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message,
        }
    }

    pub fn unauthorized() -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            message: "Unauthorized".to_string(),
        }
    }

    pub fn forbidden() -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            message: "Forbidden".to_string(),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (self.status, self.message).into_response()
    }
}

pub type Result<T> = AnyResult<T, Error>;
