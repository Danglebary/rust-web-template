// region:    module imports and declarations

// external crates
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

// internal imports
use super::{Error, Result};
use lib_model::{
    service::TodoService, EntityService, Error as ModelError, ModelManager, Todo, TodoForCreate,
    TodoForUpdate,
};

// modules

// self imports and exports

// endregion: module imports and declarations

#[tracing::instrument]
#[utoipa::path(
    get,
    path = "/v1/todo/{id}",
    responses(
        (status = 200, body = Todo),
        (status = 404, body = String),
        (status = 500, body = String)
    )
)]
pub async fn get_by_id(State(mm): State<ModelManager>, Path(id): Path<u64>) -> Result<Todo> {
    let todo = TodoService::get_by_id(mm, id).await;

    match todo {
        Ok(todo) => Ok(todo),
        Err(ModelError::EntityNotFound(_, _)) => Err(Error::not_found("todo".to_string())),
        _ => Err(Error::internal()),
    }
}

#[tracing::instrument]
#[utoipa::path(
    get,
    path = "/v1/todo",
    responses(
        (status = 200, body = Vec<Todo>),
        (status = 500, body = String)
    )
)]
pub async fn list(State(mm): State<ModelManager>) -> Result<Json<Vec<Todo>>> {
    let todos = TodoService::list(mm).await;

    match todos {
        Ok(todos) => Ok(todos),
        _ => Err(Error::internal()),
    }
}

#[tracing::instrument]
#[utoipa::path(
    post,
    path = "/v1/todo",
    responses(
        (status = 200, body = Todo),
        (status = 400, body = String),
        (status = 500, body = String)
    )
)]
pub async fn create(
    State(mm): State<ModelManager>,
    Json(data): Json<TodoForCreate>,
) -> Result<Todo> {
    let todo = TodoService::create(mm, data).await;

    match todo {
        Ok(todo) => Ok(todo),
        Err(ModelError::Validation(err)) => Err(Error::bad_request(err.to_string())),
        _ => Err(Error::internal()),
    }
}

#[tracing::instrument]
#[utoipa::path(
    patch,
    path = "/v1/todo/{id}",
    responses(
        (status = 200, body = Todo),
        (status = 400, body = String),
        (status = 404, body = String),
        (status = 500, body = String)
    )
)]
pub async fn update(
    State(mm): State<ModelManager>,
    Path(id): Path<u64>,
    Json(data): Json<TodoForUpdate>,
) -> Result<Todo> {
    let todo = TodoService::update(mm, id, data).await;

    match todo {
        Ok(todo) => Ok(todo),
        Err(ModelError::EntityNotFound(name, _)) => Err(Error::not_found(name.into())),
        Err(ModelError::Validation(err)) => Err(Error::bad_request(err.to_string())),
        _ => Err(Error::internal()),
    }
}

#[tracing::instrument]
#[utoipa::path(
    delete,
    path = "/v1/todo/{id}",
    responses(
        (status = 200, body = String),
        (status = 404, body = String),
        (status = 500, body = String)
    )
)]
pub async fn delete(State(mm): State<ModelManager>, Path(id): Path<u64>) -> Result<String> {
    let todo = TodoService::delete(mm, id).await;

    match todo {
        Ok(_) => Ok("".to_string()),
        Err(ModelError::EntityNotFound(_, _)) => Err(Error::not_found("todo".to_string())),
        _ => Err(Error::internal()),
    }
}

pub(crate) fn get_routes() -> Router<ModelManager> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/:id", get(get_by_id).patch(update).delete(delete))
}
