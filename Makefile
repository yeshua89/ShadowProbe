# Makefile para facilitar comandos Docker
.PHONY: help dev build run test clean shell

help:
	@echo "ShadowProbe - Docker Commands"
	@echo "=============================="
	@echo "make dev      - Run development environment with hot-reload"
	@echo "make build    - Build release binary"
	@echo "make run      - Run production container"
	@echo "make test     - Run all tests"
	@echo "make shell    - Open shell in dev container"
	@echo "make clean    - Remove containers and volumes"
	@echo "make fmt      - Format code"
	@echo "make clippy   - Run clippy linter"

dev:
	docker compose up dev

build:
	docker compose run --rm build

run:
	docker compose run --rm shadowprobe

test:
	docker compose run --rm test

shell:
	docker compose run --rm dev /bin/bash

clean:
	docker compose down -v
	docker system prune -f

fmt:
	docker compose run --rm dev cargo fmt --all

clippy:
	docker compose run --rm dev cargo clippy --all-targets --all-features -- -D warnings

# Atajos para desarrollo rápido
check:
	docker compose run --rm dev cargo check --all

watch:
	docker compose run --rm dev cargo watch -x check -x test -x run
