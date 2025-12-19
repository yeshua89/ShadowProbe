# Makefile para facilitar comandos Docker
.PHONY: help dev build run test clean shell compile check run-cli

help:
	@echo "╔══════════════════════════════════════════════════════════════╗"
	@echo "║           ShadowProbe - Comandos Rápidos                     ║"
	@echo "╚══════════════════════════════════════════════════════════════╝"
	@echo ""
	@echo "⚡ COMANDOS MÁS USADOS:"
	@echo "  make compile  - Compilar (rápido, sin shell)"
	@echo "  make test     - Ejecutar tests"
	@echo "  make run-cli  - Ver ayuda del CLI"
	@echo ""
	@echo "🛠️  DESARROLLO:"
	@echo "  make fmt      - Formatear código"
	@echo "  make clippy   - Linter"
	@echo "  make check    - Verificar código"
	@echo ""
	@echo "🐚 SHELL INTERACTIVA (para trabajo prolongado):"
	@echo "  make dev      - Abrir bash en el contenedor"
	@echo "                  Útil para: debuggear, experimentar, múltiples comandos"
	@echo "                  Dentro usa: cargo build, cargo test, cargo run"
	@echo "                  Salir: exit o Ctrl+D"
	@echo ""
	@echo "🚀 PRODUCCIÓN:"
	@echo "  make build    - Compilar release optimizado"
	@echo "  make run      - Ejecutar contenedor de producción"
	@echo ""
	@echo "🧹 UTILIDADES:"
	@echo "  make clean    - Limpiar contenedores"
	@echo ""
	@echo "💡 TIPS:"
	@echo "  - Para tareas rápidas: use 'make compile', 'make test'"
	@echo "  - Para trabajar largo rato: use 'make dev' y quédate en la shell"
	@echo "  - Dentro de 'make dev' NO uses comandos 'make', usa 'cargo'"

dev:
	@echo "═══════════════════════════════════════════════════"
	@echo "🐚 Abriendo shell interactiva..."
	@echo ""
	@echo "Comandos útiles dentro:"
	@echo "  cargo build              - Compilar"
	@echo "  cargo test               - Tests"
	@echo "  cargo run -- --help      - Ejecutar CLI"
	@echo "  cargo watch -x test      - Auto-test en cambios"
	@echo "  exit                     - Salir"
	@echo "═══════════════════════════════════════════════════"
	@echo ""
	@docker compose run --rm dev /bin/bash

compile:
	@echo "Building debug binary..."
	docker compose run --rm dev cargo build --bin shadowprobe

build:
	@echo "Building release binary..."
	docker compose run --rm dev cargo build --release --bin shadowprobe

run:
	docker compose run --rm shadowprobe

test:
	docker compose run --rm test

shell:
	@echo "Opening interactive shell in dev container..."
	@echo "Type 'exit' or press Ctrl+D to return to your terminal"
	docker compose run --rm dev /bin/bash

clean:
	@echo "Cleaning up containers and volumes..."
	docker compose down -v
	docker system prune -f

fmt:
	@echo "Formatting code..."
	docker compose run --rm dev cargo fmt --all

clippy:
	@echo "Running clippy linter..."
	docker compose run --rm dev cargo clippy --all-targets --all-features -- -D warnings

check:
	@echo "Checking code..."
	docker compose run --rm dev cargo check --all

watch:
	@echo "Starting cargo watch..."
	docker compose run --rm dev cargo watch -x check -x test -x run

# Shortcuts for common tasks
quick-test:
	@echo "Running quick tests..."
	docker compose run --rm dev cargo test --lib

doc:
	@echo "Generating documentation..."
	docker compose run --rm dev cargo doc --no-deps --open

# Comando conveniente para ejecutar el CLI después de compilar
run-cli:
	@echo "Compilando y ejecutando CLI..."
	@docker compose run --rm dev sh -c "cargo build --bin shadowprobe 2>/dev/null && ./target/debug/shadowprobe --help"

# Ejecutar un escaneo de prueba
scan-example:
	@echo "Ejemplo de escaneo (requiere compilar primero con 'make compile'):"
	@echo "docker compose run --rm dev ./target/debug/shadowprobe scan --url https://example.com"

# Alias para comandos comunes
quick: compile test
	@echo "✅ Compilación y tests completados!"

ci: fmt clippy test
	@echo "✅ CI checks pasados!"
