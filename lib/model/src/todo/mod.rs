// region:    module imports and declarations

// external crates
use axum::response::{IntoResponse, Json, Response};
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use serde::Serialize;
use sqlb::Fields;
use sqlx::FromRow;
use utoipa::ToSchema;

// internal imports

// modules
pub mod for_create;
pub mod for_update;
pub mod service;

// self imports and exports
pub use for_create::TodoForCreate;
pub use for_update::TodoForUpdate;

// endregion: module imports and declarations

#[derive(Debug, Serialize, Clone, Fields, FromRow, ToSchema)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,

    #[serde(with = "ts_milliseconds")]
    pub created_at: DateTime<Utc>,

    #[serde(with = "ts_milliseconds")]
    pub updated_at: DateTime<Utc>,
}

impl IntoResponse for Todo {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
