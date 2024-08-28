// region:    module imports and declarations

// external crates

use serde_valid::Validate;

// internal imports
use super::{Todo, TodoForCreate, TodoForUpdate};
use crate::{entity::EntityService, ModelManager, Result};

// modules

// self imports and exports

// endregion: module imports and declarations

pub struct TodoService;

impl EntityService<Todo, TodoForCreate, TodoForUpdate> for TodoService {
    const TABLE: &'static str = "todo";

    async fn create(mm: ModelManager, data: TodoForCreate) -> Result<Todo> {
        // validate the data
        data.validate()?;

        let db = mm.db();

        // TODO: We should be able to use the `INSERT INTO ... RETURNING *` syntax here,
        // but there is a bug in MariaDB that is prTodoing sqlx from returning the inserted row.
        // tl;dr: MariaDB's COM_STMT_PREPARE response is responding with a zero-length column count,
        // even though the column names are present in the response. This is causing sqlx to not
        // be able to parse the response.
        // slqx issue: https://github.com/launchbadge/sqlx/issues/1530
        // MariaDB issue: https://jira.mariadb.org/browse/MDEV-27013
        // Note: The MariaDB issue was opened in 2021, and it's still open as of 04/2024
        // So, for now, we'll just do two queries to get the inserted row.

        // create the todo
        let (id,) = sqlx::query_as::<_, (i64,)>("INSERT INTO todo (title) VALUES (?) RETURNING id")
            .bind(data.title)
            .fetch_one(db)
            .await?;

        // get the todo
        let todo = sqlx::query_as::<_, Todo>("SELECT * FROM todo WHERE id = ?")
            .bind(id)
            .fetch_one(db)
            .await?;

        Ok(todo)
    }

    async fn update(mm: ModelManager, id: i64, data: TodoForUpdate) -> Result<Todo> {
        // validate the data
        data.validate()?;

        let db = mm.db();

        // setup transaction
        let mut tx = db.begin().await?;

        let _ = sqlx::query("SELECT * FROM todo WHERE id = ?")
            .bind(id)
            .fetch_one(&mut *tx)
            .await?;

        // update the todo
        let _ = sqlx::query("UPDATE todo SET title = ?, completed = ? WHERE id = ?")
            .bind(data.title)
            .bind(data.completed)
            .bind(id)
            .execute(&mut *tx)
            .await?;

        // get the todo
        let todo = sqlx::query_as::<_, Todo>("SELECT * FROM todo WHERE id = ?")
            .bind(id)
            .fetch_one(&mut *tx)
            .await?;

        // commit the transaction
        tx.commit().await?;

        Ok(todo)
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use test_case::test_case;

    use super::*;

    #[test_case(
        TodoForCreate { title: "".into() },
        None,
        Some("field 'title' must be at least 1 character long");
        "title too short"
    )]
    #[test_case(
        TodoForCreate { title: "a".repeat(256) },
        None,
        Some("field 'title' must be at most 255 characters long");
        "title too long"
    )]
    #[test_case(
        TodoForCreate { title: "test_create".into() },
        Some("test_create"),
        None;
        "success"
    )]
    #[serial]
    #[tokio::test(flavor = "multi_thread")]
    async fn create_tests(
        for_create: TodoForCreate,
        expected_title: Option<&'static str>,
        expected_error: Option<&'static str>,
    ) {
        let mm = ModelManager::new_test().await.unwrap();
        let result = TodoService::create(mm.clone(), for_create).await;

        println!("{:?}", result);

        match result {
            Ok(todo) => {
                if let Some(title) = expected_title {
                    assert_eq!(title, todo.title);
                } else {
                    panic!("expected an error, but got a todo: {:?}", todo);
                }
            }
            Err(err) => {
                if let Some(error) = expected_error {
                    assert!(err.to_string().contains(error));
                } else {
                    panic!("expected a todo, but got an error: {:?}", err);
                }
            }
        }
    }
}
