use crate::domain::models::user::User;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait UserRepository {
    async fn find_all(&self) -> Result<Vec<User>, sqlx::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error>;
    async fn create(&self, user: User) -> Result<User, sqlx::Error>;
    async fn update(&self, user: User) -> Result<User, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_repository() {
        let mut mock_repo = MockUserRepository::new();

        mock_repo
            .expect_find_all()
            .times(1)
            .returning(|| Box::pin(async { Ok(vec![]) }));

        let result = mock_repo.find_all().await.unwrap();

        assert_eq!(result.len(), 0);
    }
}
