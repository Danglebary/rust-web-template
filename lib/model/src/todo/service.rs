// region:    module imports and declarations

// external crates

// internal imports
use super::{Todo, TodoForCreate, TodoForUpdate};
use crate::entity::EntityService;

// modules

// self imports and exports

// endregion: module imports and declarations

pub struct TodoService;

impl EntityService<Todo, TodoForCreate, TodoForUpdate> for TodoService {
    const TABLE: &'static str = "todo";
    const SERVICE_NAME: &'static str = "TodoService";
}

#[cfg(test)]
mod tests {
    use crate::ModelManager;
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
