# ============================================================================
# Multi-stage Dockerfile for User Auth Plugin
# Builds both Rust backend and Vue frontend in a single container
# ============================================================================

# ============================================================================
# Stage 1: Build Rust Backend
# ============================================================================
FROM rust:slim-bookworm AS rust-builder

# Install build dependencies for RocksDB
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libclang-dev \
    clang \
    cmake \
    make \
    binutils \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy Cargo files first for dependency caching
COPY Cargo.toml Cargo.lock ./
COPY src/domains/user/migration/Cargo.toml src/domains/user/migration/
COPY src/domains/tenant/migration/Cargo.toml src/domains/tenant/migration/

# Create dummy source files for dependency caching
RUN mkdir -p src && echo "fn main() {}" > src/main.rs && \
    echo "pub fn lib() {}" > src/lib.rs && \
    mkdir -p src/domains/user/migration/src && echo "fn main() {}" > src/domains/user/migration/src/main.rs && \
    echo "pub fn lib() {}" > src/domains/user/migration/src/lib.rs && \
    mkdir -p src/domains/tenant/migration/src && echo "fn main() {}" > src/domains/tenant/migration/src/main.rs && \
    echo "pub fn lib() {}" > src/domains/tenant/migration/src/lib.rs

# Build dependencies (cached layer)
RUN cargo build --release 2>/dev/null || true

# Remove dummy files and copy real source code
COPY src src/
COPY src/domains/user/migration src/domains/user/migration/
COPY src/domains/tenant/migration src/domains/tenant/migration/

# Build the actual application
RUN cargo build --release --workspace && \
    strip --strip-debug target/release/user-auth-plugin && \
    strip --strip-debug target/release/migration && \
    ls -la target/release

# ============================================================================
# Stage 2: Build Vue Frontend
# ============================================================================
FROM node:22-slim AS frontend-builder

WORKDIR /app/web

# Copy package files for dependency caching
COPY web/package.json web/package-lock.json ./

# Install dependencies
RUN npm ci

# Copy frontend source code
COPY web/ ./

# Build frontend for production
# Run lint before build - fail if lint error
RUN npm run lint

RUN npm run build

# ============================================================================
# Stage 2.5: E2E Testing (Quality Gate)
# Dies if tests fail
# ============================================================================
FROM node:22-bookworm-slim AS e2e-tester

# Install runtime deps for backend (backend compiled in debian-slim-bookworm context)
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    curl \
    procps \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy backend binary from builder
COPY --from=rust-builder /app/target/release/user-auth-plugin ./
COPY --from=rust-builder /app/target/release/user_migration ./
COPY --from=rust-builder /app/target/release/tenant_migration ./migration
# Copy test files
COPY tests/e2e ./tests/e2e

# Install test dependencies
WORKDIR /app/tests/e2e/jest
RUN npm ci

# Setup Test Environment
ENV PORT=5500
ENV HOST=0.0.0.0
ENV CORE_DB_TYPE=sqlite
ENV CORE_DB_NAME=e2e_test.sqlite
ENV JWT_SECRET=test_secret_key_very_long_and_secure_enough_for_testing
ENV JWT_ACCESS_EXPIRY=3600
ENV JWT_REFRESH_EXPIRY=86400
ENV RUST_LOG=info

WORKDIR /app

# Run E2E Tests
# 1. Run migrations
# 2. Start Server (background)
# 3. Wait for Healthcheck
# 4. Run Tests
# 5. Fail build if any step fails
RUN ./user_migration up && ./tenant_migration up && \
    (./user-auth-plugin & echo $! > server_pid) && \
    echo "Waiting for server to start..." && \
    sleep 2 && \
    (curl --retry 10 --retry-delay 2 --retry-connrefused http://localhost:5500/health || (cat logs/* && exit 1)) && \
    echo "Server is up, running tests..." && \
    cd tests/e2e/jest && \
    npm test && \
    echo "Tests Passed!" && \
    kill $(cat /app/server_pid)

# ============================================================================
# Stage 3: Runtime Image
# ============================================================================
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    dumb-init \
    && rm -rf /var/lib/apt/lists/*

# Create app user for security
RUN groupadd -r appgroup && useradd -r -g appgroup -u 1000 appuser

WORKDIR /app

# Copy backend binary
COPY --from=rust-builder /app/target/release/user-auth-plugin ./user-auth-plugin
COPY --from=rust-builder /app/target/release/migration ./migration

# Hardening: Make binaries immutable
RUN chmod 0555 ./user-auth-plugin ./migration

# Copy frontend build output
COPY --from=frontend-builder /app/web/dist ./web/dist

# Copy entrypoint script
COPY docker-entrypoint.sh ./

# Create necessary directories
RUN mkdir -p /app/logs /app/assets /app/rocksdb_cache && \
    chown -R appuser:appgroup /app

# Make entrypoint executable
RUN chmod +x /app/docker-entrypoint.sh

# Switch to non-root user
USER appuser

# Expose backend port
EXPOSE 5500

# Healthcheck
HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:5500/health || exit 1

# Set entrypoint
ENTRYPOINT ["/usr/bin/dumb-init", "--", "/app/docker-entrypoint.sh"]
