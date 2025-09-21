# 環境変数をロード
include .env
export $(shell sed 's/=.*//' .env)

# コンテナ起動
up:
	docker compose up -d

# コンテナ停止
down:
	docker compose down

# PostgreSQL に接続
db-shell:
	docker exec -it app-db psql -U $(POSTGRES_USER) -d $(POSTGRES_DB)

# PostgreSQL のログを確認
db-logs:
	docker logs -f app-db

# PostgreSQL のデータをバックアップ
db-dump:
	docker exec app-db pg_dump -U $(POSTGRES_USER) -d $(POSTGRES_DB) > backup.sql

# バックアップからデータを復元
db-restore:
	docker exec -i app-db psql -U $(POSTGRES_USER) -d $(POSTGRES_DB) < backup.sql

help:
	@echo "Available commands:"
	@echo "  up           - Start containers"
	@echo "  down         - Stop containers"
	@echo "  db-shell     - Enter PostgreSQL shell"
	@echo "  db-logs      - Show PostgreSQL logs"
	@echo "  db-dump      - Backup database"
	@echo "  db-restore   - Restore database"
