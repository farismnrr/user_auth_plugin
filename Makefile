# User Auth Plugin - Makefile for Development Automation

.PHONY: help dev start install-watch build test test-integration test-e2e test-e2e-auth test-e2e-users test-e2e-details migrate-up migrate-down migrate-fresh db-reset clean kill

# Default target
help:
	@echo "User Auth Plugin - Available Commands:"
	@echo ""
	@echo "  make dev              - Run development server with hot reload"
	@echo "  make start            - Run development server without hot reload"
	@echo "  make install-watch    - Install cargo-watch for hot reload"
	@echo "  make build            - Build release binary"
	@echo "  make test             - Run all tests"
	@echo "  make test-integration - Run integration tests (whitebox)"
	@echo "  make test-e2e         - Run E2E tests (blackbox)"
	@echo "  make test-e2e-auth    - Run e2e auth tests only"
	@echo "  make test-e2e-users   - Run e2e user tests only"
	@echo "  make test-e2e-details - Run e2e user details tests only"
	@echo "  make migrate-up       - Run database migrations"
	@echo "  make migrate-down     - Rollback last migration"
	@echo "  make migrate-fresh    - Drop all tables and re-run migrations"
	@echo "  make db-reset         - Reset database (fresh + seed if available)"
	@echo "  make clean            - Clean build artifacts"
	@echo "  make kill             - Kill process running on PORT (from .env)"
	@echo ""

# Run development server with hot reload (requires cargo-watch)
dev:
	@echo "🚀 Starting development server with hot reload..."
	@echo "💡 Tip: Install cargo-watch with 'make install-watch' if not installed"
	@cargo watch -x run || (echo "❌ cargo-watch not found. Installing..." && cargo install cargo-watch && cargo watch -x run)

# Run development server without hot reload
start:
	@echo "🚀 Starting development server (no hot reload)..."
	cargo run

# Install cargo-watch for hot reload
install-watch:
	@echo "📦 Installing cargo-watch..."
	cargo install cargo-watch
	@echo "✅ cargo-watch installed successfully"

# Build release binary
build:
	@echo "🔨 Building release binary..."
	cargo build --release

# Run all tests
test:
	@echo "🧪 Running all tests..."
	cargo test -- --test-threads=1

# Run integration tests only (whitebox)
test-integration:
	@echo "🧪 Running integration tests (whitebox)..."
	cargo test --test integration_tests -- --test-threads=1

# K6 command for E2E tests
K6_CMD = docker run --rm -i --user "$(shell id -u):$(shell id -g)" --network="host" -v $(PWD):/scripts -w /scripts grafana/k6 run

# Run E2E tests only (blackbox)
test-e2e:
	@echo "🧪 Running all k6 E2E tests with HTML report..."
	@$(K6_CMD) tests/e2e/k6/test-e2e.js
	@echo "✅ All k6 tests completed. Report generated at coverage/test-e2e.html"

# E2E auth tests only
test-e2e-auth:
	@echo "🧪 Running e2e auth tests..."
	@$(K6_CMD) tests/e2e/k6/auth/register.js
	@$(K6_CMD) tests/e2e/k6/auth/login.js
	@$(K6_CMD) tests/e2e/k6/auth/logout.js
	@$(K6_CMD) tests/e2e/k6/auth/refresh.js
	@$(K6_CMD) tests/e2e/k6/auth/verify.js
	@echo "✅ Auth tests completed"

# E2E user tests only
test-e2e-users:
	@echo "🧪 Running e2e user tests..."
	@$(K6_CMD) tests/e2e/k6/users/get.js
	@$(K6_CMD) tests/e2e/k6/users/get_all.js
	@$(K6_CMD) tests/e2e/k6/users/update.js
	@$(K6_CMD) tests/e2e/k6/users/delete.js
	@echo "✅ User tests completed"

# E2E user details tests only
test-e2e-details:
	@echo "🧪 Running e2e user details tests..."
	@$(K6_CMD) tests/e2e/k6/user_details/update.js
	@$(K6_CMD) tests/e2e/k6/user_details/upload.js
	@echo "✅ User details tests completed"

# Load .env variables and construct DATABASE_URL from CORE_DB_* variables
define load_env_and_db_url
	export $$(grep -v '^#' .env | grep -v '^$$' | xargs); \
	export DATABASE_URL="postgresql://$$CORE_DB_USER:$$CORE_DB_PASS@$$CORE_DB_HOST:$$CORE_DB_PORT/$$CORE_DB_NAME"
endef

# Create database if it doesn't exist
define create_db_if_not_exists
	$(load_env_and_db_url); \
	docker exec postgres-sql psql -U $$CORE_DB_USER -tc "SELECT 1 FROM pg_database WHERE datname = '$$CORE_DB_NAME'" | grep -q 1 || \
	docker exec postgres-sql psql -U $$CORE_DB_USER -c "CREATE DATABASE $$CORE_DB_NAME"
endef

# Run database migrations (up)
migrate-up:
	@echo "⬆️  Running database migrations..."
	@echo "📦 Ensuring database exists..."
	@$(create_db_if_not_exists)
	@$(load_env_and_db_url); cd migration && cargo run -- up
	@echo "✅ Migrations completed"

# Rollback last migration
migrate-down:
	@echo "⬇️  Rolling back last migration..."
	@$(load_env_and_db_url); cd migration && cargo run -- down
	@echo "✅ Rollback completed"

# Fresh migration (drop all and re-run)
migrate-fresh:
	@echo "🔄 Running fresh migrations..."
	@echo "📦 Ensuring database exists..."
	@$(create_db_if_not_exists)
	@$(load_env_and_db_url); cd migration && cargo run -- fresh
	@echo "✅ Fresh migrations completed"

# Reset database (fresh migrations)
db-reset: migrate-fresh
	@echo "🗑️  Database reset completed"

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	@echo "✅ Clean completed"

# Kill process running on port 5500 (server's default port)
kill:
	@echo "🔪 Killing processes on port 5500..."
	@lsof -ti:5500 | xargs -r kill -9 || echo "✅ No process running on port 5500"



