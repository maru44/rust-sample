.PHONY: post

post:
	@curl -X POST -H 'Content-Type: application/json' -d '{"username": "maru"}' http://localhost:8000/create
