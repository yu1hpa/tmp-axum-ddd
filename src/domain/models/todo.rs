use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Todo {
    pub fn new(title: String, description: String) -> Self {
        // 日本時間のオフセット（UTC+9時間）
        let jst = FixedOffset::east_opt(9 * 3600).unwrap();
        // 現在の日本時間を取得し、UTCに変換
        let now_jst = jst.from_utc_datetime(&Utc::now().naive_utc());
        let now_utc = now_jst.with_timezone(&Utc);

        Self {
            id: Uuid::now_v7(),
            title,
            description: Some(description),
            completed: false,
            created_at: now_utc,
            updated_at: now_utc,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_todo() {
        /*
        - タイトルと説明が正しく設定されていること
        - 初期状態ではcompletedがfalseであること
        - 作成日時と更新日時が同じであること
        - 生成されるUUIDがバージョン7（時間ベース）であること
        - 作成時刻が現在時刻に近いこと
        */
        let title = "テストタイトル";
        let description = "テスト説明";

        let todo = Todo::new(title.to_string(), description.to_string());

        assert_eq!(todo.title, title);
        assert_eq!(todo.description, Some(description.to_string()));

        assert!(!todo.completed);

        assert_eq!(todo.created_at, todo.updated_at);

        assert_eq!(todo.id.get_version_num(), 7);

        let now = Utc::now();
        let diff = now.signed_duration_since(todo.created_at);
        assert!(diff.num_seconds().abs() < 1);
    }
}
