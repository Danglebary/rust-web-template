// region:    module imports and declarations

// external crates
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::time::Duration;

// internal imports
use lib_config::config;

// modules
mod error;
pub mod test;

// self imports and exports
pub use error::*;

// endregion: module imports and declarations

pub type Db = Pool<MySql>;

pub async fn new_pool() -> Result<Db> {
    let db_url = format!(
        "mariadb://{}:{}@{}:{}/{}",
        config().DB_USER,
        config().DB_PASSWORD,
        config().DB_HOST,
        config().DB_PORT,
        config().DB_NAME
    );

    MySqlPoolOptions::new()
        .max_connections(config().DB_POOL_MAX_CONNECTIONS)
        .min_connections(config().DB_POOL_MIN_CONNECTIONS)
        .acquire_timeout(Duration::from_millis(config().DB_AQUIRE_TIMEOUT_MILLIS))
        .idle_timeout(Duration::from_millis(config().DB_IDLE_TIMEOUT_MILLIS))
        .connect(&db_url)
        .await
        .map_err(|err| Error::ConnectionPool(err.to_string()))
}
