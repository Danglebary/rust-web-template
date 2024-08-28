// region:    module imports and declarations

// external crates

// internal imports
use super::Db;
use crate::config;

// modules

// self imports and exports

// endregion: module imports and declarations

pub async fn new_test_pool() -> Db {
    // setup the database for testing
    root_setup().await;

    // create the test database pool
    let pool = test_pool().await;

    // Run db migrations
    let m = sqlx::migrate::Migrator::new(std::path::Path::new("../../migrations"))
        .await
        .unwrap();
    m.run(&pool).await.unwrap();

    pool
}

/// Setup the database for testing.
///
/// This will:
/// - Create the test database and user
/// - Run the migrations against the test database
async fn root_setup() {
    let root_pool = root_pool().await;

    let init_str = std::fs::read_to_string("../../migrations/dev_only/init.sql").unwrap();
    let statements = init_str
        .lines()
        .filter(|line| !line.contains("--") && !line.is_empty())
        .map(|line| line.trim().replace(';', ""))
        .collect::<Vec<String>>();

    for statement in statements {
        sqlx::query(&statement).execute(&root_pool).await.unwrap();
    }
}

async fn root_pool() -> Db {
    let config = config();

    let url = format!(
        "postgres://root:super_secret_password@{}:{}/",
        config.DB_HOST, config.DB_PORT
    );

    new_pool(url).await
}

async fn test_pool() -> Db {
    let config = config();

    let url = format!(
        "postgres://test_user:test_password@{}:{}/test_db",
        config.DB_HOST, config.DB_PORT
    );

    new_pool(url).await
}

async fn new_pool(url: String) -> Db {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_millis(500))
        .connect(&url)
        .await
        .unwrap_or_else(|_| panic!("Failed to create pool with url: {url}"))
}
