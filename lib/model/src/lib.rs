// region:    module imports and declarations

// external crates

// internal imports
use lib_store::Db;

// modules
pub mod entity;
mod error;

// All entity models must be public
// This is for the OpenApi generation
pub mod todo;

// self imports and exports
pub use entity::*;
pub use error::*;
pub use todo::*;

// endregion: module imports and declarations

#[derive(Debug, Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        let db = lib_store::new_pool().await?;

        Ok(Self { db })
    }

    // Only the model layer needs to access the database directly.
    pub(crate) fn db(&self) -> &Db {
        &self.db
    }
}

#[cfg(test)]
impl ModelManager {
    /// Create a new ModelManager for testing.
    ///
    /// This will create a new database connection pool and run the migrations
    /// against the local docker instance of MariaDB
    pub(crate) fn new_test() -> impl std::future::Future<Output = Result<Self>> {
        async move {
            let db = lib_store::test::new_test_pool().await;
            Ok(Self { db })
        }
    }
}
