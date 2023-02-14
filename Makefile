.PHONY: post

post:
	@curl -X POST -H 'Content-Type: application/json' -d '{"username": "maru"}' http://localhost:8000/create

migrate:
	@cd infra/db && \
	sqlx migrate run --database-url postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DATABASE}
