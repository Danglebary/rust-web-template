// region:    module imports and declarations

// external crates

use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use sqlb::Fields;
use utoipa::ToSchema;

// internal imports
use crate::entity::EntityForCreate;

// modules

// self imports and exports

// endregion: module imports and declarations

#[derive(Debug, Serialize, Deserialize, Validate, Clone, ToSchema, Fields)]
pub struct TodoForCreate {
    #[validate(
        min_length = 1,
        message = "field 'title' must be at least 1 character long"
    )]
    #[validate(
        max_length = 255,
        message = "field 'title' must be at most 255 characters long"
    )]
    pub title: String,
}

impl EntityForCreate for TodoForCreate {}
