use std::sync::Arc;

use crate::domain::models::todo::Todo;
use crate::domain::repositories::todo_repository::TodoRepository;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone)]
pub struct TodoUsecase {
    repository: Arc<dyn TodoRepository + Send + Sync>,
}

impl TodoUsecase {
    pub fn new(repository: Arc<dyn TodoRepository + Send + Sync>) -> Self {
        Self { repository }
    }
}

#[async_trait]
pub trait TodoService {
    async fn get_all(&self) -> Result<Vec<Todo>, sqlx::Error>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Todo>, sqlx::Error>;
    async fn create(&self, title: String, description: String) -> Result<Todo, sqlx::Error>;
    async fn update(
        &self,
        id: Uuid,
        title: String,
        description: String,
        completed: bool,
    ) -> Result<Todo, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
}

#[async_trait]
impl TodoService for TodoUsecase {
    async fn get_all(&self) -> Result<Vec<Todo>, sqlx::Error> {
        self.repository.find_all().await
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Todo>, sqlx::Error> {
        self.repository.find_by_id(id).await
    }

    async fn create(&self, title: String, description: String) -> Result<Todo, sqlx::Error> {
        let new_todo = Todo::new(title, description);
        self.repository.create(new_todo).await
    }

    async fn update(
        &self,
        id: Uuid,
        title: String,
        description: String,
        completed: bool,
    ) -> Result<Todo, sqlx::Error> {
        let existing_todo = self.repository.find_by_id(id).await?;
        if let Some(mut todo) = existing_todo {
            todo.title = title;
            todo.description = Some(description);
            todo.completed = completed;
            return self.repository.update(todo).await;
        }
        Err(sqlx::Error::RowNotFound)
    }

    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        self.repository.delete(id).await
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::repositories::todo_repository::MockTodoRepository;

    use super::*;
    use chrono::{FixedOffset, TimeZone, Utc};
    fn create_test_todo(title: &str, description: Option<&str>) -> Todo {
        let jst = FixedOffset::east_opt(9 * 3600).unwrap();
        let now_jst = jst.from_utc_datetime(&Utc::now().naive_utc());
        let now_utc = now_jst.with_timezone(&Utc);

        Todo {
            id: Uuid::now_v7(),
            title: title.to_string(),
            description: description.map(|s| s.to_string()),
            completed: false,
            created_at: now_utc,
            updated_at: now_utc,
        }
    }
    #[tokio::test]
    async fn test_get_all_todos() {
        let mut mock_repo = MockTodoRepository::new();

        let todos = Arc::new(vec![
            create_test_todo("タスク1", Some("説明1")),
            create_test_todo("タスク2", Some("説明2")),
        ]);

        mock_repo.expect_find_all().times(1).returning(move || {
            let todos_clone = Arc::clone(&todos);
            Box::pin(async move { Ok((*todos_clone).clone()) })
        });

        let usecase = TodoUsecase::new(Arc::new(mock_repo));

        let result = usecase.get_all().await.unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].title, "タスク1");
        assert_eq!(result[1].title, "タスク2");
    }
}
