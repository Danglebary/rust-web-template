// region:    module imports and declarations

// external crates
use axum::Json;
use serde::{de::DeserializeOwned, Serialize};
use serde_valid::Validate;
use sqlb::HasFields;
use sqlx::{postgres::PgRow, FromRow};
use std::future::Future;
use tracing::error;

// internal imports
use super::{Error, ModelManager, Result};

// modules

// self imports and exports

// endregion: module imports and declarations

pub trait EntityService<E, C, U>
where
    E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
    E: HasFields,
    C: EntityForCreate + HasFields + Send,
    U: EntityForUpdate + HasFields + Send,
{
    const TABLE: &'static str;
    const SERVICE_NAME: &'static str;

    fn create(mm: ModelManager, data: C) -> impl Future<Output = Result<E>> + Send {
        async move {
            let db = mm.db();

            let fields = data.not_none_fields();

            match sqlb::insert()
                .table(Self::TABLE)
                .data(fields)
                .returning(E::field_names())
                .fetch_one(db)
                .await
            {
                Ok(entity) => Ok(entity),
                Err(err) => {
                    error!("{}::create: sqlx error: {}", Self::SERVICE_NAME, err);
                    Err(err.into())
                }
            }
        }
    }

    fn get_by_id(mm: ModelManager, id: i64) -> impl Future<Output = Result<E>> + Send {
        async move {
            let db = mm.db();

            match sqlb::select()
                .table(Self::TABLE)
                .columns(E::field_names())
                .and_where_eq("id", id)
                .fetch_optional(db)
                .await?
                .ok_or(Error::EntityNotFound(Self::TABLE, id))
            {
                Ok(entity) => Ok(entity),
                Err(err) => {
                    error!("{}::get_by_id: sqlx error: {}", Self::SERVICE_NAME, err);
                    Err(err.into())
                }
            }
        }
    }

    fn list(mm: ModelManager) -> impl Future<Output = Result<Json<Vec<E>>>> + Send {
        async move {
            let db = mm.db();

            match sqlb::select()
                .table(Self::TABLE)
                .columns(E::field_names())
                .order_by("id")
                .fetch_all(db)
                .await
            {
                Ok(entities) => Ok(Json(entities)),
                Err(err) => {
                    error!("{}::list: sqlx error: {}", Self::SERVICE_NAME, err);
                    Err(err.into())
                }
            }
        }
    }

    fn update(mm: ModelManager, id: i64, data: U) -> impl Future<Output = Result<E>> + Send {
        async move {
            let db = mm.db();

            let fields = data.not_none_fields();

            match sqlb::update()
                .table(Self::TABLE)
                .and_where_eq("id", id)
                .data(fields)
                .returning(E::field_names())
                .fetch_one(db)
                .await
            {
                Ok(entity) => Ok(entity),
                Err(err) => {
                    error!("{}::update: sqlx error: {}", Self::SERVICE_NAME, err);
                    Err(err.into())
                }
            }
        }
    }

    fn delete(mm: ModelManager, id: i64) -> impl Future<Output = Result<u64>> + Send {
        async move {
            let db = mm.db();

            match sqlb::delete()
                .table(Self::TABLE)
                .and_where_eq("id", id)
                .exec(db)
                .await
            {
                Ok(count) => {
                    if count == 0 {
                        Err(Error::EntityNotFound(Self::TABLE, id))
                    } else {
                        Ok(count)
                    }
                }
                Err(err) => {
                    error!("{}::delete: sqlx error: {}", Self::SERVICE_NAME, err);
                    Err(err.into())
                }
            }
        }
    }
}

pub trait EntityForCreate: Serialize + DeserializeOwned + Validate {}
pub trait EntityForUpdate: Serialize + DeserializeOwned + Validate {}
