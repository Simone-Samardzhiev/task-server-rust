include .env

MIGRATIONS_PATH=./migrations

.PHONY: migrate-create
migrate-create:
	@migrate create -seq -ext sql -dir $(MIGRATIONS_PATH) $(NAME)

.PHONY: migrate-up
migrate-up:
	@migrate -path $(MIGRATIONS_PATH) -database $(DATABASE_URL) up

.PHONY: migrate-down
migrate-down:
	@migrate -path  $(MIGRATIONS_PATH) -database $(DATABASE_URL) down

.PHONY: start
start:
	@go run ./cmd/api/main.go