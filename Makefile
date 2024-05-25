ENV ?= local

ifdef SERVER_ENV
	ENV := ${SERVER_ENV}
endif

.PHONY: build
build:
	docker compose --env-file .env.${ENV} build

.PHONY: build-run
build-run:
	docker compose --env-file .env.${ENV} up -d --build

.PHONY: run
run:
	docker compose --env-file .env.${ENV} up -d

.PHONY: stop
stop:
	docker compose --env-file .env.${ENV} down

.PHONY: enter-psql
enter-psql:
	docker exec -it totp-based-2fa-server-postgres bash