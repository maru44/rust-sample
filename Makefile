.PHONY: post

post:
	@curl -X POST -H 'Content-Type: application/json' -d '{"name": "maru", "email": "aa@example.com"}' http://localhost:8000/users

db/migrate:
	@cd ${ARC} && \
	make migrate
