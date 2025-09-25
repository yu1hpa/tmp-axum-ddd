use std::sync::Arc;

use crate::domain::models::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserUsecase {
    repository: Arc<dyn UserRepository + Send + Sync>,
}

impl UserUsecase {
    pub fn new(repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { repository }
    }
}

#[async_trait]
pub trait UserService {
    async fn get_all(&self) -> Result<Vec<User>, sqlx::Error>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error>;
    async fn create(
        &self,
        name: String,
        nickname: String,
        email: String,
        password: String,
    ) -> Result<User, sqlx::Error>;
    async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        nickname: Option<String>,
        email: Option<String>,
        password: Option<String>,
    ) -> Result<User, sqlx::Error>;
    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error>;
}

#[async_trait]
impl UserService for UserUsecase {
    async fn get_all(&self) -> Result<Vec<User>, sqlx::Error> {
        self.repository.find_all().await
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        self.repository.find_by_id(id).await
    }

    async fn create(
        &self,
        name: String,
        nickname: String,
        email: String,
        password: String,
    ) -> Result<User, sqlx::Error> {
        // todo!("Email Validation");
        // todo!("Password Validation");
        let new_user = User::new(name, nickname, email, password);
        self.repository.create(new_user).await
    }

    async fn update(
        &self,
        id: Uuid,
        name: Option<String>,
        nickname: Option<String>,
        email: Option<String>,
        hashed_password: Option<String>,
    ) -> Result<User, sqlx::Error> {
        // todo!("Length Validation");
        // todo!("Email Validation");
        // todo!("Password Validation");
        let mut existing_user = match self.repository.find_by_id(id).await? {
            Some(user) => user,
            None => return Err(sqlx::Error::RowNotFound),
        };

        if let Some(n) = name {
            existing_user.name = n;
        }

        if let Some(nn) = nickname {
            existing_user.nickname = nn;
        }

        if let Some(e) = email {
            existing_user.email = e;
        }

        if let Some(p) = hashed_password {
            existing_user.hashed_password = p;
        }

        self.repository.update(existing_user).await
    }

    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        self.repository.delete(id).await
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::repositories::user_repository::MockUserRepository;
    use mockall::predicate::eq;

    use super::*;
    fn create_test_user(name: &str, nickname: &str, email: &str, password: &str) -> User {
        User::new(
            name.to_string(),
            nickname.to_string(),
            email.to_string(),
            password.to_string(),
        )
    }

    #[tokio::test]
    async fn test_get_all_users() {
        let mut mock_repo = MockUserRepository::new();

        let users = Arc::new(vec![
            create_test_user(
                "yu1hpa",
                "nickname_yu1hpa",
                "yu1hpa@example.com",
                "asdfqwer",
            ),
            create_test_user(
                "yu2hpa",
                "nickname_yu2hpa",
                "yu2hpa@example.com",
                "qwerasdf",
            ),
        ]);

        mock_repo.expect_find_all().times(1).returning(move || {
            let users_clone = Arc::clone(&users);
            Box::pin(async move { Ok((*users_clone).clone()) })
        });

        let usecase = UserUsecase::new(Arc::new(mock_repo));

        let result = usecase.get_all().await.unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "yu1hpa");
        assert_eq!(result[1].email, "yu2hpa@example.com");
    }

    #[tokio::test]
    async fn test_update_user() {
        /*
        - 名前だけ yu1hpa -> yu2hpa に変更
         */
        let mut mock_repo = MockUserRepository::new();

        let existing_user = create_test_user(
            "yu1hpa",
            "nickname_yu1hpa",
            "yu1hpa@example.com",
            "asdfqwer",
        );
        let user_id = existing_user.id;

        mock_repo
            .expect_find_by_id()
            .with(eq(existing_user.id))
            .returning(move |_| {
                let user = existing_user.clone();
                Box::pin(async move { Ok(Some(user)) })
            });

        mock_repo
            .expect_update()
            .withf(|user: &User| user.name == "yu2hpa")
            .returning(move |user| {
                let updated_user = user.clone();
                Box::pin(async move { Ok(updated_user) })
            });

        let usecase = UserUsecase::new(Arc::new(mock_repo));

        // 名前だけ更新
        let result = usecase
            .update(user_id, Some("yu2hpa".into()), None, None, None)
            .await
            .unwrap();

        assert_eq!(result.name, "yu2hpa".to_string());
        assert_eq!(result.nickname, "nickname_yu1hpa".to_string());
        assert_eq!(result.email, "yu1hpa@example.com".to_string());
    }
}
