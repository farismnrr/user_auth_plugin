# Overview

This document explains the SSO architecture and authentication flow used by the Multi-Tenant User Management Service.

---

## What is SSO?

Single Sign-On (SSO) allows users to authenticate once with a centralized service and gain access to multiple applications without re-entering credentials. This service acts as an **Identity Provider (IdP)** for your applications.

---

## Architecture

```
┌─────────────────────┐                            ┌──────────────────────┐
│                     │  1. Redirect to SSO        │                      │
│   Client App        │ ─────────────────────────▶ │   SSO Service        │
│   (Your Frontend)   │                            │   (Auth Portal)      │
│                     │ ◀───────────────────────── │                      │
└─────────────────────┘  4. Redirect with token    └──────────────────────┘
         │                                                   │
         │                                                   │
         │ 5. API calls with                    2. User enters credentials
         │    Bearer token                      3. Authentication successful
         ▼                                                   ▼
┌─────────────────────┐                            ┌──────────────────────┐
│                     │                            │                      │
│   Your Backend API  │ ◀────────────────────────  │   Database           │
│                     │     JWT Validation         │                      │
└─────────────────────┘                            └──────────────────────┘
```

---

## Authentication Flow

### Step 1: Initiate Login

Your application redirects the user to the SSO login page with required parameters:

```
https://sso.example.com/login?tenant_id=xxx&redirect_uri=xxx
```

> **Note**: The SSO login page automatically adds `state`, `nonce`, `response_type`, and `scope` parameters for security.

### Step 2: User Authentication

The SSO portal displays a login form where the user enters their credentials. The SSO service validates the credentials against the database.

### Step 3: Token Generation

Upon successful authentication, the SSO service generates a JWT access token containing:

- User ID
- Tenant ID
- User role
- Token expiration time

### Multi-Tenant Access Model

The service enforces strict role-based access rules:

1. **Global SSO (Role: `user`)**:
   - A generic `user` account can access multiple tenants.
   - Credentials (email/password) are shared across tenants (Single Sign-On).
   - When a user logs into a new tenant, they are automatically linked to that tenant.

2. **Tenant-Scoped Access (Role: `admin`, `supplier`, etc.)**:
   - Roles other than `user` are strictly scoped to a single tenant.
   - An `admin` in Tenant A cannot access Tenant B with the same account.
   - Separate accounts (different emails/usernames) are required for admin access to different tenants.
   - Attempts to register/link a non-user role to an existing account will result in a **409 Conflict**.

### Step 4: Redirect Back

The SSO service redirects the user back to your application with the access token in the URL hash fragment:

```
https://your-app.com/auth/callback#access_token=eyJhbG...&state=xxx
```

> **Security Note**: The token is placed in the hash fragment (`#`) rather than query parameters (`?`) to prevent it from being logged in server access logs.

### Step 5: Use Token

Your application extracts the token and uses it for authenticated API requests:

```
Authorization: Bearer eyJhbGciOiJIUzI1NiIs...
```

---

## Key Features

| Feature | Description |
|---------|-------------|
| **Multi-Tenant** | One SSO service supports multiple applications |
| **JWT Tokens** | Industry-standard token format with built-in expiration |
| **Secure Redirect** | Tokens transmitted via hash fragment for security |
| **State Parameter** | CSRF protection using random state values |
| **Nonce Parameter** | Replay attack protection |

---

## Supported Operations

| Operation | SSO Endpoint | Description |
|-----------|--------------|-------------|
| Login | `/login` | User authentication |
| Register | `/register` | New user registration |
| Logout | `/auth/logout` | Token invalidation |
| Token Refresh | `/api/auth/refresh` | Get new access token |

---

## Next Steps

→ [Configuration](./02-configuration.md) - Set up environment variables
