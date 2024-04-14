// region:    module imports and declarations

// external crates
use serde::{de::DeserializeOwned, Serialize};
use serde_valid::Validate;
use sqlx::{mysql::MySqlRow, FromRow};

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

    async fn get_by_id(mm: ModelManager, id: u64) -> Result<E> {
        let db = mm.db();

        let sql = format!("SELECT * FROM {} WHERE id = ?", Self::TABLE);
        let entity: E = sqlx::query_as(&sql)
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound(Self::TABLE, id))?;

        Ok(entity)
    }

    async fn list(mm: ModelManager) -> Result<Vec<E>> {
        let db = mm.db();

        let sql = format!("SELECT * FROM {}", Self::TABLE);
        let entities: Vec<E> = sqlx::query_as(&sql).fetch_all(db).await?;

        Ok(entities)
    }

    async fn create(mm: ModelManager, data: C) -> Result<E>;
    async fn update(mm: ModelManager, id: u64, data: U) -> Result<E>;

    async fn delete(mm: ModelManager, id: u64) -> Result<u64> {
        let db = mm.db();

        let sql = format!("DELETE FROM {} WHERE id = ?", Self::TABLE);
        let result = sqlx::query(&sql).bind(id).execute(db).await?;

        Ok(result.rows_affected())
    }
}

pub trait EntityForCreate: Serialize + DeserializeOwned + Validate {}
pub trait EntityForUpdate: Serialize + DeserializeOwned + Validate {}
