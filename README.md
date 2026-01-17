# Multi-Tenant User Management Service

A production-ready, standalone user authentication and management service built with Rust and Actix-web. Designed to be easily plugged into any application requiring secure user management, JWT authentication, and role-based access control.

## Key Features

- **Auth**: JWT (Access & Refresh Tokens), Session Management, RBAC
- **Multi-tenancy**: Tenant-scoped Users, Tenant Isolation
- **Security**: Argon2 Password Hashing, Rate Limiting, API Key Protection
- **Operations**: Structured Logging, Health Checks, Graceful Shutdown, Soft Deletes
- **Performance**: RocksDB for local persistent caching with TTL

## Documentation

📚 **[SSO Integration Guide](docs/README.md)** - Complete guide for integrating SSO into your frontend application

| Document | Description |
|----------|-------------|
| [Overview](docs/01-overview.md) | Architecture and SSO flow |
| [Getting Started](docs/02-configuration.md) | **Tenant setup** and environment config |
| [Redirect Parameters](docs/03-redirect-parameters.md) | URL format and params |
| [Frontend Implementation](docs/04-frontend-implementation.md) | Next.js, React, Vue, JS examples |
| [Token Handling](docs/05-token-handling.md) | Parsing and storing tokens |
| [API Reference](docs/06-api-reference.md) | All endpoints with examples |
| [Troubleshooting](docs/07-troubleshooting.md) | Common issues and solutions |

## Caching

This service implements a local persistent cache using [RocksDB](https://rocksdb.org/). This helps reduce database load and improve response times for frequently accessed data.
- **Persistence**: Cached data is stored locally in the `rocksdb_cache` directory.
- **TTL Support**: Items are cached with an expiration time, ensuring data freshness.
- **Lazy Expiration**: Expired items are removed upon access to maintain efficiency.

## API Structure

This service exposes two API scopes with different authentication mechanisms:

1.  **Public / API Key Protected (`/api` prefix)**
    *   Requires `X-API-Key` header (for Auth/User) or `X-Tenant-Secret-Key` (for Tenant creation).
    *   Endpoints: `/api/auth/login`, `/api/auth/register`, `/api/auth/refresh`.

2.  **Protected / JWT (`/` root scope)**
    *   Requires `Authorization: Bearer <token>` header.
    *   Endpoints: `/auth/logout`, `/auth/verify`, `/auth/change-password`, `/users/*`, `/tenants/*`.

## Environment Variables

| Variable | Description |
|----------|-------------|
| `JWT_SECRET` | Secret key for signing JWT tokens |
| `TENANT_SECRET_KEY` | Secret key for creating tenants via API |
| `API_KEY` | Global API Key for auth endpoints |
    *   Requires `Authorization: Bearer <token>` header.
    *   Endpoints: `/auth/logout`, `/auth/verify`, `/auth/change-password`, `/users/*`, `/tenants/*`.

## How to Run (Local Development)

The following steps describe a local development setup.

### 1. Prerequisites

Ensure you have the following installed:
- **Rust** & **Cargo**
- **Make**
- **Docker** (Required for Database)

### 2. Start Database

Start a PostgreSQL container (required by `Makefile`):

```bash
make start-postgres
```

### 3. Configuration

Set up your environment variables:

```bash
cp .env.example .env
# Edit .env if you need to change secrets or ports (defaults work for dev)
```

### 4. Database Migration

Initialize the database schema:

```bash
make migrate-up
```

### 5. Run Application

**Local Development (Hot Reload):**
```bash
make dev
```
The server will be available at `http://0.0.0.0:5500`.

**Docker (Local / Development):**
```bash
make start-compose
```
This runs the application using Docker Compose.

## Available Commands

The following Makefile commands are provided for development convenience:

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
| `make test-e2e-tenants` | Run E2E tenant tests only |
| `make test-e2e-soft-delete` | Run E2E soft delete tests only |
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
