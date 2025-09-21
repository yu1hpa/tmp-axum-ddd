use crate::domain::models::todo::Todo;
use crate::domain::repositories::todo_repository::TodoRepository;
use crate::infrastructure::db::DbPool;
use async_trait::async_trait;
use uuid::Uuid;

#[derive(Clone)]
pub struct TodoRepositoryImpl {
    pub pool: DbPool,
}

impl TodoRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<Todo>, sqlx::Error> {
        let todos = sqlx::query_as::<_, Todo>(
            "SELECT id, title, description, completed, created_at, updated_at FROM todos",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(todos)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Todo>, sqlx::Error> {
        let todo = sqlx::query_as::<_, Todo>(
            "SELECT id, title, description, completed, created_at, updated_at FROM todos WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(todo)
    }

    async fn create(&self, todo: Todo) -> Result<Todo, sqlx::Error> {
        let created_todo = sqlx::query_as::<_, Todo>(
            "INSERT INTO todos (id, title, description, completed, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING id, title, description, completed, created_at, updated_at",
        )
        .bind(todo.id)
        .bind(&todo.title)
        .bind(&todo.description)
        .bind(todo.completed)
        .bind(todo.created_at)
        .bind(todo.updated_at)
        .fetch_one(&self.pool)
        .await?;
        Ok(created_todo)
    }

    async fn update(&self, todo: Todo) -> Result<Todo, sqlx::Error> {
        let updated_todo = sqlx::query_as::<_, Todo>(
            "UPDATE todos SET title = $1, description = $2, completed = $3, updated_at = (NOW() AT TIME ZONE 'Asia/Tokyo')
             WHERE id = $4
             RETURNING id, title, description, completed, created_at, updated_at"
        )
        .bind(&todo.title)
        .bind(&todo.description)
        .bind(todo.completed)
        .bind(todo.id)
        .fetch_one(&self.pool)
        .await?;
        Ok(updated_todo)
    }

    async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM todos WHERE id = $1")
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
        let pool = setup_test_db().await;
        let repo = TodoRepositoryImpl::new(pool);

        // 新しいTodoを作成
        let title = "テストタスク".to_string();
        let description = "テスト説明".to_string();
        let todo = Todo::new(title.clone(), description.clone());

        // 作成したTodoをデータベースに保存
        let created_todo = repo.create(todo).await.unwrap();

        // IDで検索
        let found_todo = repo.find_by_id(created_todo.id).await.unwrap();

        // 検証
        assert!(found_todo.is_some());
        let found_todo = found_todo.unwrap();
        assert_eq!(found_todo.id, created_todo.id);
        assert_eq!(found_todo.title, title);
        assert_eq!(found_todo.description, Some(description));
        assert!(!found_todo.completed);

        // 後処理：作成したTodoを削除
        repo.delete(created_todo.id).await.unwrap();
    }
}
