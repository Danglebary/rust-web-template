// region:    module imports and declarations

// external crates
use axum::{extract::State, response::IntoResponse, Json};

// internal imports
use lib_model::{service::TodoService, EntityService, ModelManager, TodoForCreate};

// modules

// self imports and exports

// endregion: module imports and declarations

#[tracing::instrument]
#[utoipa::path(
    post,
    path = "/todo",
    responses(
        (status = 200, body = Todo),
        (status = 400, body = String)
    )
)]
pub async fn create_todo(
    State(mm): State<ModelManager>,
    Json(data): Json<TodoForCreate>,
) -> impl IntoResponse {
    let todo = TodoService::create(mm, data).await;

    match todo {
        Ok(todo) => todo.into_response(),
        Err(e) => e.to_string().into_response(),
    }
}
