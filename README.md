# Template Axum DDD

## 準備

sqlx-cli のインストール

```
cargo install sqlx-cli --no-default-features --features postgres
```

データベース作成

```
sqlx database create
```

テーブルを作成

```
sqlx migrate add create_todos
```

マイグレーション

```
sqlx migrate run
```

マイグレーション取り消し

```
sqlx migrate revert
```

Docker コンテナ内のデータベースを確認

```
docker exec -it app-db psql -U app_user -d app_db
```

```
\l
```

```
\dt
```

```
\d todos
```

## ディレクトリ構成

```
src/
├── domain/                      # ドメイン層（ビジネスルール）
│   ├── models/                  # エンティティ・値オブジェクト
│   │   ├── xxx.rs
│   ├── repositories/            # リポジトリインターフェース
│   │   ├── xxx_repository.rs
├── usecase/                     # ユースケース層（アプリケーションロジック）
│   ├── xxx_usecase.rs           # ユースケース（CRUD 処理）
├── infrastructure/              # インフラ層（データベースなど）
│   ├── db.rs                    # データベース接続設定
│   ├── xxx_repository.rs        # リポジトリの実装（domain/repositories の実装）
├── presentation/                # プレゼンテーション層（Web API）
│   ├── handlers/                # 各エンドポイントのハンドラー
│   │   ├── xxx_handler.rs       # エンドポイント処理
├── main.rs                      # エントリポイント
```

## テスト

```
cargo test
```

```
cargo test -- --ignored
```
