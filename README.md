# Multi Tenant User Management Plugin

A production-ready, general-purpose user authentication and management service built with Rust and Actix-web. Designed to be easily plugged into any application requiring secure user management, JWT authentication, and role-based access control.

## Key Features

- **Auth**: JWT (Access/Refresh), RBAC, Session Management
- **Security**: Argon2, Rate Limiting, API Key Protection
- **Ops**: Structured Logging, Health Checks, Graceful Shutdown

## Quick Start

### 1. Setup Environment

Ensure you have Rust and PostgreSQL installed. Then configure your environment using the provided example:

```bash
cp .env.example .env
# Edit .env with your specific configuration (DB credentials, secrets, etc.)
```

### 2. Database Migration

Initialize the database schema:

```bash
make migrate-up       # Run migrations
make migrate-fresh    # Reset DB and re-run migrations
```

### 3. Run Server

**Development (Hot Reload):**
```bash
make dev
```

**Production:**
```bash
make build
./target/release/user-auth-plugin
```

The server runs on `http://0.0.0.0:5500`.

### 4. Docker (Recommended)

Run via Docker Compose (using pre-built image from GHCR):

```bash
make start-compose    # Starts the stack
make stop-compose     # Stops the stack
make update           # Update to latest version
```
- Uses `network_mode: "host"` to connect to your local Postgres.
- Mounts `./logs` and `./assets` for persistence.
- Runs as your current user (1000:1000) to avoid permission issues.

## Available Commands

| Command | Description |
|---------|-------------|
| `make help` | Show available commands |
| `make dev` | Run development server with hot reload |
| `make start` | Run development server without hot reload |
| `make install-watch` | Install cargo-watch for hot reload |
| `make build` | Build release binary |
| `make test` | Run all tests |
| `make test-integration` | Run integration tests (whitebox) |
| `make test-e2e` | Run all E2E tests with HTML report |
| `make test-e2e-auth` | Run E2E auth tests only |
| `make test-e2e-users` | Run E2E user tests only |
| `make test-e2e-details` | Run E2E user details tests only |
| `make migrate-up` | Run database migrations |
| `make migrate-down` | Rollback last migration |
| `make migrate-fresh` | Drop all tables and re-run migrations |
| `make db-reset` | Reset database (fresh + seed if available) |
| `make clean` | Clean build artifacts |
| `make kill` | Force kill process on port 5500 |
| `make pull-docker` | Pull latest image |
| `make start-compose` | Start Docker Compose stack |
| `make stop-compose` | Stop Docker Compose stack |
| `make update` | Update running container (Watchtower) |
