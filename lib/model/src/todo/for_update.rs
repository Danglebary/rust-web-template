// region:    module imports and declarations

// external crates
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use utoipa::ToSchema;

// internal imports
use crate::entity::EntityForUpdate;

// modules

// self imports and exports

// endregion: module imports and declarations

#[derive(Debug, Serialize, Deserialize, Validate, Clone, ToSchema)]
pub struct TodoForUpdate {
    #[validate(
        min_length = 1,
        message = "The title must be at least 1 character long."
    )]
    #[validate(
        max_length = 255,
        message = "The title must be at most 255 characters long."
    )]
    pub title: Option<String>,

    pub completed: Option<bool>,
}

impl EntityForUpdate for TodoForUpdate {}
