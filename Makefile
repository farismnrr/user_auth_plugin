# User Auth Plugin - Makefile for Development Automation

.PHONY: help dev start install-watch build test test-integration test-e2e test-e2e-pre test-e2e-auth test-e2e-tenant test-e2e-user migrate-up migrate-down migrate-fresh db-reset clean kill key start-postgres stop-postgres


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
	@echo "  make test-e2e         - Run all E2E tests (Jest)"
	@echo "  make test-e2e-pre     - Run pre-tests (bootstrapping)"
	@echo "  make test-e2e-auth    - Run auth tests"
	@echo "  make test-e2e-tenant  - Run tenant tests"
	@echo "  make test-e2e-user    - Run user tests"
	@echo "  make start-web        - Build frontend and serve from Rust (static)"
	@echo "  make dev-web          - Run frontend with Vite hot reload (port 5173)"
	@echo "  make dev-all          - Run backend + frontend (static build mode)"
	@echo "  make migrate-up       - Run database migrations"
	@echo "  make migrate-down     - Rollback last migration"
	@echo "  make migrate-fresh    - Drop all tables and re-run migrations"
	@echo "  make db-reset         - Reset database (fresh + seed if available)"
	@echo "  make start-postgres   - Start PostgreSQL container"
	@echo "  make stop-postgres    - Stop PostgreSQL container"
	@echo "  make clean            - Clean build artifacts"
	@echo "  make kill             - Kill process running on PORT (from .env)"
	@echo "  make key              - Generate a random SHA-512 key"
	@echo ""

# Run development server with hot reload (requires cargo-watch)
dev:
	@echo "🚀 Starting development server with hot reload..."
	@echo "💡 Tip: Install cargo-watch with 'make install-watch' if not installed"
	@cargo watch -i "*.sqlite*" -i "*.db*" -i "rocksdb_cache" -x run || (echo "❌ cargo-watch not found. Installing..." && cargo install cargo-watch && cargo watch -i "*.sqlite*" -i "*.db*" -i "rocksdb_cache" -x run)

# Run development server with hot reload
start:
	@echo "🚀 Starting development server (no hot reload)..."
	cargo run

# Run web frontend (static build, served from Rust)
start-web:
	@echo "🔨 Building Web Frontend..."
	@cd web && npm install && npm run build
	@echo "✅ Frontend built and ready to be served from Rust (port 5500)"

# Run web frontend (hot reload with Vite dev server)
dev-web:
	@echo "🚀 Starting Web Frontend (Vite hot reload)..."
	@cd web && npm run dev

# Run both backend and frontend concurrently (static build mode)
dev-all:
	@echo "🚀 Starting User Auth Plugin (Backend + Frontend static)..."
	@make start-web
	@make dev

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

# Build via Docker
docker: build-docker
build-docker:
	@read -p "Enter Docker tag (default: latest): " tag; \
	tag=$${tag:-latest}; \
	echo "🐳 Building Docker image with tag: $$tag..."; \
	docker build -t $(DOCKER_IMAGE_NAME):$$tag -t $(GHCR_REPO):$$tag .; \
	echo "✅ Image tagged as $(DOCKER_IMAGE_NAME):$$tag and $(GHCR_REPO):$$tag"

# Run via Docker (with .env and host network)
start-docker:
	@read -p "Enter Docker tag to run (default: latest): " tag; \
	tag=$${tag:-latest}; \
	echo "🚀 Starting Docker container with tag: $$tag..."; \
	docker run --rm -it --network="host" --env-file .env $(DOCKER_IMAGE_NAME):$$tag

# Push to GHCR (reads env vars) - Multi-arch build
push-local: build-docker
	@read -p "Enter Docker tag to push (default: latest): " tag; \
	tag=$${tag:-latest}; \
	echo "🚀 Pushing to GHCR with multi-arch build (amd64, arm64) - tag: $$tag..."; \
	export $$(grep -v '^#' .env | grep -v '^$$' | xargs); \
	if [ -n "$${CR_PAT}" ] || [ -n "$${GITHUB_TOKEN}" ]; then \
		echo "🔐 Logging in to GHCR..."; \
		echo "$${CR_PAT:-$$GITHUB_TOKEN}" | docker login ghcr.io -u farismnrr --password-stdin; \
	else \
		echo "⚠️  No CR_PAT or GITHUB_TOKEN found. Skipping login (assuming already logged in)..."; \
	fi; \
	docker buildx build --platform linux/amd64,linux/arm64 -t $(GHCR_REPO):$$tag --push .; \
	echo "✅ Image pushed to $(GHCR_REPO):$$tag"

push:
	@echo "🚀 Triggering GitHub Actions workflow for Docker push..."
	@command -v gh >/dev/null 2>&1 || ( \
		if command -v apt-get >/dev/null 2>&1; then \
			echo "⬇️  Installing GitHub CLI via apt..."; \
			SUDO=$$(command -v sudo >/dev/null 2>&1 && echo sudo || echo); \
			$$SUDO apt-get update && $$SUDO apt-get install -y gh || { echo "❌ Failed to install gh"; exit 1; }; \
		else \
			echo "❌ GitHub CLI 'gh' not found and auto-install is not configured for this OS."; \
			echo "   Install from https://cli.github.com/ then rerun 'make push'"; \
			exit 1; \
		fi \
	)
	@REF=$${REF:-main}; \
	echo "📦 Triggering workflow 'build-multitenant-user-management.yml' with ref: $$REF..."; \
	gh workflow run build-multitenant-user-management.yml --ref $$REF
	@echo "✅ Workflow dispatched. Track with 'gh run watch --latest'"

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

# --- PostgreSQL Management ---

# Start PostgreSQL container
start-postgres:
	@echo "🐘 Starting PostgreSQL container..."
	@docker start postgres-sql || docker run --name postgres-sql -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres:alpine

# Stop PostgreSQL container
stop-postgres:
	@echo "🛑 Stopping PostgreSQL container..."
	@docker stop postgres-sql || true

# Run all tests
test: start-postgres
	@echo "🧪 Running all tests..."
	cargo test -- --test-threads=1

# Run integration tests only (whitebox)
test-integration:
	@echo "🧪 Running integration tests (whitebox)..."
	cargo test --test integration_tests -- --test-threads=1

# Jest Test Directory
JEST_DIR = tests/e2e/jest

# Run all E2E tests (Jest)
test-e2e: start-postgres
	@echo "🧪 Running all E2E tests (Jest)..."
	@cd $(JEST_DIR) && npx jest --runInBand

# Run pre-tests (Bootstrapping)
test-e2e-pre:
	@echo "🧪 Running pre-tests..."
	@cd $(JEST_DIR) && npx jest 1_pre_test --runInBand

# Run auth tests
test-e2e-auth:
	@echo "🧪 Running auth tests..."
	@cd $(JEST_DIR) && npx jest 2_auth_test --runInBand

# Run tenant tests
test-e2e-tenant:
	@echo "🧪 Running tenant tests..."
	@cd $(JEST_DIR) && npx jest 3_tenant_test --runInBand

# Run user tests
test-e2e-user:
	@echo "🧪 Running user tests..."
	@cd $(JEST_DIR) && npx jest 4_user_test --runInBand

# Load .env variables and construct DATABASE_URL based on DB_TYPE
define load_env_and_db_url
	export $$(grep -v '^#' .env | grep -v '^$$' | xargs); \
	if [ "$$CORE_DB_TYPE" = "sqlite" ]; then \
		DB_NAME=$${CORE_DB_NAME:-user_auth_plugin.sqlite}; \
		case "$$DB_NAME" in \
			*.sqlite|*.db) ;; \
			*) DB_NAME="$$DB_NAME.sqlite";; \
		esac; \
		export DATABASE_URL="sqlite://$(CURDIR)/$$DB_NAME?mode=rwc"; \
	else \
		export DATABASE_URL="postgresql://$$CORE_DB_USER:$$CORE_DB_PASS@$$CORE_DB_HOST:$$CORE_DB_PORT/$$CORE_DB_NAME"; \
	fi
endef

# Create database if it doesn't exist
define create_db_if_not_exists
	$(load_env_and_db_url); \
	if [ "$$CORE_DB_TYPE" = "sqlite" ]; then \
		echo "📦 Using SQLite database (file-based)"; \
	else \
		docker exec postgres-sql psql -U 5c964c2b206140738bf8e92325746465 -d postgres -tc "SELECT 1 FROM pg_database WHERE datname = '$$CORE_DB_NAME'" | grep -q 1 || \
		docker exec postgres-sql psql -U 5c964c2b206140738bf8e92325746465 -d postgres -c "CREATE DATABASE $$CORE_DB_NAME OWNER $$CORE_DB_USER"; \
	fi
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

# Kill process running on port 5500 (backend) and 5173 (frontend)
kill:
	@echo "🔪 Killing processes on port 5500 (Backend)..."
	@lsof -ti:5500 | xargs -r kill -9 || echo "✅ No process running on port 5500"
	@echo "🔪 Killing processes on port 5173 (Frontend)..."
	@lsof -ti:5173 | xargs -r kill -9 || echo "✅ No process running on port 5173"

# Generate a random SHA-512 hash
key:
	@openssl rand -base64 128 | tr -d '\n' | sha512sum | awk '{print $$1}'

# --- Tenant Management ---

# Create a new tenant interactively
create-tenant:
	@read -p "Enter Tenant Name: " name; \
	read -p "Enter Tenant Description: " description; \
	export $$(grep -v '^#' .env | grep -v '^$$' | xargs); \
	if [ -z "$$name" ]; then \
		echo "❌ Tenant Name is required"; \
		exit 1; \
	fi; \
	echo "🚀 Creating tenant '$$name'..."; \
	curl -s -X POST $$ENDPOINT/api/tenants \
		-H "Content-Type: application/json" \
		-H "X-Tenant-Secret-Key: $$TENANT_SECRET_KEY" \
		-d "{\"name\": \"$$name\", \"description\": \"$$description\"}" | jq .
