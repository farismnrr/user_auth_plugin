# User Auth Plugin - Makefile for Development Automation

.PHONY: help dev start install-watch build test test-integration test-e2e test-e2e-auth test-e2e-users test-e2e-details test-e2e-tenants test-e2e-soft-delete migrate-up migrate-down migrate-fresh db-reset clean kill

# Default target
help:
	@echo "User Auth Plugin - Available Commands:"
	@echo ""
	@echo "  make dev              - Run development server with hot reload"
	@echo "  make start            - Run development server without hot reload"
	@echo "  make install-watch    - Install cargo-watch for hot reload"
	@echo "  make build            - Build release binary"
	@echo "  make build-docker     - Build Docker image"
	@echo "  make start-docker     - Run Docker image (with .env and host network)"
	@echo "  make push             - Push Docker image to GHCR"
	@echo "  make pull-docker      - Pull latest image for Docker Compose"
	@echo "  make start-compose    - Start Docker Compose stack (pulls first)"
	@echo "  make stop-compose     - Stop Docker Compose stack"
	@echo "  make update    - Update running container using Watchtower"
	@echo "  make test             - Run all tests"
	@echo "  make test-integration - Run integration tests (whitebox)"
	@echo "  make test-e2e         - Run E2E tests (blackbox)"
	@echo "  make test-e2e-auth    - Run e2e auth tests only"
	@echo "  make test-e2e-users   - Run e2e user tests only"
	@echo "  make test-e2e-details - Run e2e user details tests only"
	@echo "  make test-e2e-tenants - Run e2e tenant tests only"
	@echo "  make test-e2e-soft-delete - Run e2e soft delete tests only"
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

# --- Docker Configuration ---
DOCKER_IMAGE_NAME = user_auth_plugin
GHCR_REPO = ghcr.io/farismnrr/user_auth_plugin
DOCKER_TAG = latest

# Build via Docker
build-docker:
	@echo "🐳 Building Docker image..."
	docker build -t $(DOCKER_IMAGE_NAME) .

# Run via Docker (with .env and host network)
start-docker:
	@echo "🚀 Starting Docker container..."
	docker run --rm -it --network="host" --env-file .env $(DOCKER_IMAGE_NAME)

# Push to GHCR (reads env vars)
push:
	@echo "🏷️  Tagging image..."
	docker tag $(DOCKER_IMAGE_NAME) $(GHCR_REPO):$(DOCKER_TAG)
	@echo "🚀 Pushing to GHCR..."
	@echo "🔐 Logging in to GHCR..."
	@export $$(grep -v '^#' .env | grep -v '^$$' | xargs) && \
	(echo "$${CR_PAT:-$$GITHUB_TOKEN}" | docker login ghcr.io -u farismnrr --password-stdin)
	docker push $(GHCR_REPO):$(DOCKER_TAG)
	@echo "✅ Image pushed to $(GHCR_REPO):$(DOCKER_TAG)"

# --- Docker Compose Configuration ---

# Pull latest image for Docker Compose
pull-docker:
	@echo "📥 Pulling latest Docker image..."
	docker compose pull

# Start Docker Compose (pulls first)
start-compose: pull-docker
	@echo "🚀 Starting Docker Compose stack..."
	docker compose up -d

# Stop Docker Compose
stop-compose:
	@echo "🛑 Stopping Docker Compose stack..."
	docker compose down

# Update running container using Watchtower
update:
	@echo "🔄 Checking for updates with Watchtower..."
	docker run --rm \
		-v /var/run/docker.sock:/var/run/docker.sock \
		--env DOCKER_API_VERSION=1.45 \
		containrrr/watchtower \
		--run-once \
		user_auth_plugin

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

# E2E tenant tests only
test-e2e-tenants:
	@echo "🧪 Running e2e tenant tests..."
	@$(K6_CMD) tests/e2e/k6/tenants/create.js
	@$(K6_CMD) tests/e2e/k6/tenants/get.js
	@$(K6_CMD) tests/e2e/k6/tenants/update.js
	@$(K6_CMD) tests/e2e/k6/tenants/delete.js
	@echo "✅ Tenant tests completed"

# E2E soft delete tests only
test-e2e-soft-delete:
	@echo "🧪 Running e2e soft delete tests..."
	@$(K6_CMD) tests/e2e/k6/users/soft_delete.js
	@$(K6_CMD) tests/e2e/k6/user_details/soft_delete.js
	@$(K6_CMD) tests/e2e/k6/tenants/soft_delete.js
	@echo "✅ Soft delete tests completed"

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

