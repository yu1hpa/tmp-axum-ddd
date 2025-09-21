use crate::domain::models::todo::Todo;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait TodoRepository {
    async fn find_all(&self) -> Result<Vec<Todo>, sqlx::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Todo>, sqlx::Error>;
    async fn create(&self, todo: Todo) -> Result<Todo, sqlx::Error>;
    async fn update(&self, todo: Todo) -> Result<Todo, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_repository() {
        let mut mock_repo = MockTodoRepository::new();

        mock_repo
            .expect_find_all()
            .times(1)
            .returning(|| Box::pin(async { Ok(vec![]) }));

        let result = mock_repo.find_all().await.unwrap();

        assert_eq!(result.len(), 0);
    }
}
