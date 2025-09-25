use crate::domain::models::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use crate::infrastructure::db::DbPool;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepositoryImpl {
    pub pool: DbPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>(
            "SELECT id, name, nickname, email, hashed_password, created_at, updated_at FROM users",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(users)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, nickname, email, hashed_password, created_at, updated_at FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }

    async fn create(&self, user: User) -> Result<User, sqlx::Error> {
        let created_user = sqlx::query_as::<_, User>(
            "INSERT INTO users (id, name, nickname, email, hashed_password, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING id, name, nickname, email, hashed_password, created_at, updated_at",
        )
        .bind(user.id)
        .bind(&user.name)
        .bind(&user.nickname)
        .bind(&user.email)
        .bind(&user.hashed_password)
        .bind(user.created_at)
        .bind(user.updated_at)
        .fetch_one(&self.pool)
        .await?;
        Ok(created_user)
    }

    async fn update(&self, user: User) -> Result<User, sqlx::Error> {
        let updated_user = sqlx::query_as::<_, User>(
            "UPDATE users SET 
            name            = COALESCE($1, name),
            nickname        = COALESCE($2, nickname),
            email           = COALESCE($3, email),
            hashed_password = COALESCE($4, hashed_password),
            updated_at      = (NOW() AT TIME ZONE 'Asia/Tokyo')
            WHERE id = $5
            RETURNING id, name, nickname, email, hashed_password, created_at, updated_at",
        )
        .bind(&user.name)
        .bind(&user.nickname)
        .bind(&user.email)
        .bind(&user.hashed_password)
        .bind(user.id)
        .fetch_one(&self.pool)
        .await?;
        Ok(updated_user)
    }

    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;
    use sqlx::postgres::PgPoolOptions;

    async fn setup_test_db() -> DbPool {
        dotenv().ok();

        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");

        PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to create pool")
    }

    #[tokio::test]
    #[ignore = "Requires DATABASE_URL to be set"]
    async fn test_create_and_find_by_id() {
        /*
        以下を確認
        - createでデータベースに保存
        - find_by_id で取得
        - delete で削除
         */
        let pool = setup_test_db().await;
        let repo = UserRepositoryImpl::new(pool);

        let name = "yu1hpa".to_string();
        let nickname = "nickname_yu1hpa".to_string();
        let email = "yu1hpa@example.com".to_string();
        let password = "asdfqwer".to_string();
        let user = User::new(
            name.clone(),
            nickname.clone(),
            email.clone(),
            password.clone(),
        );

        let created_user = repo.create(user).await.unwrap();

        let found_user = repo.find_by_id(created_user.id).await.unwrap();

        assert!(found_user.is_some());
        let found_user = found_user.unwrap();
        assert_eq!(found_user.id, created_user.id);
        assert_eq!(found_user.name, name);
        assert_eq!(found_user.nickname, nickname);
        assert_eq!(found_user.email, email);
        assert!(found_user.verify_password(&password));

        repo.delete(created_user.id).await.unwrap();
        let x_user = repo.find_by_id(created_user.id).await.unwrap();
        assert!(x_user.is_none())
    }
}
