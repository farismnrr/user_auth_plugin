# API Reference

This document lists all available API endpoints for authentication and user management.

---

## Authentication Methods

| Method | Header | Used For |
|--------|--------|----------|
| **Tenant Secret** | `X-Tenant-Secret-Key: {key}` | Tenant creation only |
| **API Key** | `X-API-Key: {key}` | Auth endpoints (`/auth/*`) |
| **JWT Token** | `Authorization: Bearer {token}` | Protected endpoints |

---

## Route Structure

The API is organized into two main scopes:

```
/api/*          → API Key or Tenant Secret protected
  /api/tenants  → Tenant management (Tenant Secret)
  /api/users    → User management (JWT)

/auth/*         → Authentication endpoints (API Key)
  /auth/login   → Login
  /auth/register → Registration
  /auth/logout  → Logout (JWT required)
  /auth/verify  → Token verification (JWT required)
```

---

## SSO Web Endpoints

These endpoints serve the SSO web interface (for browser redirects):

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/login` | SSO login page |
| GET | `/register` | SSO registration page |

### Query Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `tenant_id` | Yes | Tenant UUID |
| `redirect_uri` | Yes | Callback URL (URL-encoded) |
| `state` | No | CSRF protection token |
| `nonce` | No | Replay attack protection |

---

## Tenant Endpoints

### Create Tenant (Bootstrapping)

```http
POST /api/tenants
X-Tenant-Secret-Key: your-tenant-secret-key
Content-Type: application/json

{
    "name": "My Application",
    "description": "Optional description"
}
```

**Response (201 Created):**

```json
{
    "status": true,
    "message": "Tenant created successfully",
    "data": {
        "tenant_id": "uuid-here"
    }
}
```

**Duplicate tenant returns existing ID (200 OK - Idempotent):**

```json
{
    "status": true,
    "message": "Tenant already exists",
    "data": {
        "tenant_id": "existing-uuid"
    }
}
```

### List Tenants

```http
GET /api/tenants
Authorization: Bearer {token}
```

### Get Tenant by ID

```http
GET /api/tenants/{tenant_id}
Authorization: Bearer {token}
```

### Update Tenant

```http
PUT /api/tenants/{tenant_id}
Authorization: Bearer {token}
Content-Type: application/json

{
    "name": "Updated Name",
    "description": "Updated description"
}
```

### Delete Tenant

```http
DELETE /api/tenants/{tenant_id}
Authorization: Bearer {token}
```

---

## Authentication Endpoints

All auth endpoints require `X-API-Key` header.

### Register

```http
POST /auth/register
X-API-Key: your-api-key
Content-Type: application/json

{
    "username": "johndoe",
    "email": "john@example.com",
    "password": "StrongPassword123!",
    "role": "user"
}
```

**Response (201 Created):**

```json
{
    "status": true,
    "message": "User registered successfully",
    "data": {
        "user_id": "uuid-here"
    }
}
```

### Login

```http
POST /auth/login
X-API-Key: your-api-key
Content-Type: application/json

{
    "email_or_username": "john@example.com",
    "password": "StrongPassword123!"
}
```

**Response (200 OK):**

```json
{
    "status": true,
    "message": "Login successful",
    "data": {
        "access_token": "eyJhbGciOiJIUzI1NiIs..."
    }
}
```

**Note:** A refresh token is also set as an HTTP-only cookie.

### Refresh Token

```http
GET /auth/refresh
X-API-Key: your-api-key
```

The refresh token is read from cookies automatically.

**Response (200 OK):**

```json
{
    "status": true,
    "data": {
        "access_token": "eyJhbGciOiJIUzI1NiIs..."
    }
}
```

### Verify Token

```http
GET /auth/verify
X-API-Key: your-api-key
Authorization: Bearer {access_token}
```

**Response (200 OK):**

```json
{
    "status": true,
    "data": {
        "user": {
            "id": "uuid-here",
            "username": "johndoe",
            "email": "john@example.com",
            "role": "user",
            "tenant_id": "tenant-uuid"
        }
    }
}
```

### Logout

```http
DELETE /auth/logout
X-API-Key: your-api-key
Authorization: Bearer {access_token}
```

**Response (200 OK):**

```json
{
    "status": true,
    "message": "Logged out successfully"
}
```

### SSO Logout (Browser)

For browser-based logout with cookie clearing:

```http
GET /auth/sso/logout
```

This endpoint clears the refresh token cookie and can redirect users.

### Reset Password

```http
PUT /auth/reset
X-API-Key: your-api-key
Authorization: Bearer {access_token}
Content-Type: application/json

{
    "current_password": "OldPassword123!",
    "new_password": "NewPassword456!"
}
```

**Response (200 OK):**

```json
{
    "status": true,
    "message": "Password changed successfully"
}
```

---

## User Endpoints

All user endpoints require JWT authentication.

### Get Current User

```http
GET /api/users/me
Authorization: Bearer {access_token}
```

### List Users

```http
GET /api/users
Authorization: Bearer {access_token}
```

### Get User by ID

```http
GET /api/users/{user_id}
Authorization: Bearer {access_token}
```

### Update User

```http
PUT /api/users/{user_id}
Authorization: Bearer {access_token}
Content-Type: application/json

{
    "username": "newusername",
    "email": "newemail@example.com"
}
```

### Delete User (Soft Delete)

```http
DELETE /api/users/{user_id}
Authorization: Bearer {access_token}
```

---

## Error Responses

All endpoints return errors in this format:

```json
{
    "status": false,
    "message": "Description of the error"
}
```

### With Validation Details

```json
{
    "status": false,
    "message": "Validation Error",
    "details": [
        {
            "field": "email",
            "message": "Invalid email format"
        }
    ]
}
```

### Common HTTP Status Codes

| Code | Meaning |
|------|---------|
| `200` | Success |
| `201` | Created |
| `400` | Bad Request (malformed JSON) |
| `401` | Unauthorized (missing/invalid credentials) |
| `403` | Forbidden (account banned or insufficient permissions) |
| `404` | Not Found |
| `409` | Conflict (duplicate email/username) |
| `415` | Unsupported Media Type (missing Content-Type) |
| `422` | Validation Error |
| `429` | Too Many Requests (rate limited) |
| `500` | Internal Server Error |

---

## Rate Limiting

The API implements rate limiting to prevent abuse:

| Setting | Default Value |
|---------|---------------|
| Max Requests | 5 per window |
| Window Duration | 15 minutes |
| Block Duration | 30 minutes |

When rate limited:

```http
HTTP/1.1 429 Too Many Requests
Retry-After: 1800

{
    "status": false,
    "message": "Too Many Requests"
}
```

---

## Next Steps

→ [Troubleshooting](./07-troubleshooting.md) - Common issues and solutions
