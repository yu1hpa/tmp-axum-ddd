use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub nickname: String,
    pub email: String,
    pub hashed_password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(name: String, nickname: String, email: String, password: String) -> Self {
        // 日本時間のオフセット（UTC+9時間）
        let jst = FixedOffset::east_opt(9 * 3600).unwrap();
        // 現在の日本時間を取得し、UTCに変換
        let now_jst = jst.from_utc_datetime(&Utc::now().naive_utc());
        let now_utc = now_jst.with_timezone(&Utc);
        Self {
            id: Uuid::now_v7(),
            name,
            nickname,
            email,
            hashed_password: hash(&password),
            created_at: now_utc,
            updated_at: now_utc,
        }
    }

    pub fn verify_password(&self, pw: &str) -> bool {
        let parsed_hash = PasswordHash::new(&self.hashed_password).unwrap();
        Argon2::default()
            .verify_password(pw.as_bytes(), &parsed_hash)
            .is_ok()
    }
}

pub fn hash(str: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(str.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_user() {
        /*
        - ユーザー名、ニックネームが正しく設定されていること
        - メールアドレスが正しく設定されていること
        - パスワードがハッシュ化されたものであること
        */

        let name = "yu1hpa";
        let nickname = "nickname_yu1hpa";
        let email = "yu1hpa@example.com";
        let password = "asdfqwer";

        let user = User::new(
            name.to_string(),
            nickname.to_string(),
            email.to_string(),
            password.to_string(),
        );

        assert_eq!(user.name, name);
        assert_eq!(user.nickname, nickname);

        assert_eq!(user.email, email);
        assert!(user.verify_password(password));
    }
}
