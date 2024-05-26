ENV ?= local

ifdef SERVER_ENV
	ENV := ${SERVER_ENV}
endif

include .env.${ENV}

.PHONY: build
build:
	@echo "Building containers..."
	@docker compose --env-file .env.${ENV} build

.PHONY: build-run
build-run:
	@echo "Building and Starting containers..."
	@docker compose --env-file .env.${ENV} up -d --build

.PHONY: run
run:
	@echo "Starting containers..."
	@docker compose --env-file .env.${ENV} up -d

.PHONY: stop
stop:
	@echo "Stopping containers..."
	@docker compose --env-file .env.${ENV} down

.PHONY: enter-psql
enter-psql:
	@echo "Entering psql container..."
	@docker exec -it totp-based-2fa-server-postgres bash

.PHONY: gen-migration
gen-migration:
	@echo "Generating a migration file..."
	@sqlx migrate add -r ${NAME}

.PHONY: migrate
migrate:
	@echo "Migrating database..."
	@echo "DATABASE_URL: ${DATABASE_URL}"
	@sqlx migrate run --database-url ${DATABASE_URL}
