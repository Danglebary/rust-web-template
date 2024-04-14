// region:    module imports and declarations

// external crates
use axum::Json;
use serde::{de::DeserializeOwned, Serialize};
use serde_valid::Validate;
use sqlx::{mysql::MySqlRow, FromRow};
use std::future::Future;

// internal imports
use super::{Error, ModelManager, Result};

// modules

// self imports and exports

// endregion: module imports and declarations

pub trait EntityService<E, C, U>
where
    E: for<'r> FromRow<'r, MySqlRow> + Unpin + Send,
    C: EntityForCreate,
    U: EntityForUpdate,
{
    const TABLE: &'static str;

    fn get_by_id(mm: ModelManager, id: u64) -> impl Future<Output = Result<E>> + Send {
        async move {
            let db = mm.db();

            let sql = format!("SELECT * FROM {} WHERE id = ?", Self::TABLE);
            let entity: E = sqlx::query_as(&sql)
                .bind(id)
                .fetch_optional(db)
                .await?
                .ok_or(Error::EntityNotFound(Self::TABLE, id))?;
            Ok(entity)
        }
    }

    fn list(mm: ModelManager) -> impl Future<Output = Result<Json<Vec<E>>>> + Send {
        async move {
            let db = mm.db();

            let sql = format!("SELECT * FROM {}", Self::TABLE);
            let entities: Vec<E> = sqlx::query_as(&sql).fetch_all(db).await?;

            Ok(Json(entities))
        }
    }

    fn create(mm: ModelManager, data: C) -> impl Future<Output = Result<E>> + Send;
    fn update(mm: ModelManager, id: u64, data: U) -> impl Future<Output = Result<E>> + Send;

    fn delete(mm: ModelManager, id: u64) -> impl Future<Output = Result<u64>> + Send {
        async move {
            let db = mm.db();

            let sql = format!("DELETE FROM {} WHERE id = ?", Self::TABLE);
            let result = sqlx::query(&sql).bind(id).execute(db).await?;

            Ok(result.rows_affected())
        }
    }
}

pub trait EntityForCreate: Serialize + DeserializeOwned + Validate {}
pub trait EntityForUpdate: Serialize + DeserializeOwned + Validate {}
