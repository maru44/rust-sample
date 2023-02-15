db/migrate:
	@cd infra/db && \
	sqlx migrate run --database-url postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DATABASE}
