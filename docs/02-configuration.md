# Getting Started

This guide walks you through the prerequisites and initial setup required before integrating SSO.

---

## Prerequisites

Before you can integrate SSO into your application, the following must be set up:

1. ✅ SSO Service is running and accessible
2. ✅ A **Tenant** has been created for your application
3. ✅ You have the **Tenant ID** and **SSO URL**
4. ✅ Your application domain is added to **Allowed Origins**

---

## Step 1: Create a Tenant

Each client application needs a registered tenant in the SSO system. The tenant provides isolation between different applications using the same SSO service.

### Option A: Using Makefile (Recommended)

Run this command in the SSO service directory:

```bash
make create-tenant
```

You will be prompted to enter:
- **Tenant Name**: A human-readable name (e.g., "IoTNet Dashboard")
- **Tenant Description**: Optional description

**Example output:**

```json
{
  "status": true,
  "message": "Tenant created successfully",
  "data": {
    "tenant_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890"
  }
}
```

> ⚠️ **Save the `tenant_id`** - you'll need this for your application configuration.

### Option B: Using cURL

```bash
curl -X POST http://localhost:5500/api/tenants \
  -H "Content-Type: application/json" \
  -H "X-Tenant-Secret-Key: YOUR_TENANT_SECRET_KEY" \
  -d '{
    "name": "My Application",
    "description": "Production application"
  }'
```

**Required Header:**
- `X-Tenant-Secret-Key`: The secret key configured in the SSO service's `.env` file (`TENANT_SECRET_KEY`)

---

## Step 2: Configure SSO Service

Ensure the SSO service's `.env` contains your application's domain in `ALLOWED_ORIGINS`:

```env
# Allow your application to make requests
ALLOWED_ORIGINS=http://localhost:3000,https://your-app.example.com
```

**Restart the SSO service after updating:**

```bash
make dev
# or for Docker
docker-compose restart
```

---

## Step 3: Configure Your Application

Add environment variables to your frontend application:

### Next.js

```env
NEXT_PUBLIC_SSO_URL=https://sso.example.com
NEXT_PUBLIC_TENANT_ID=a1b2c3d4-e5f6-7890-abcd-ef1234567890
```

### Vite (React/Vue)

```env
VITE_SSO_URL=https://sso.example.com
VITE_TENANT_ID=a1b2c3d4-e5f6-7890-abcd-ef1234567890
```

---

## Step 4: Create Auth Routes

Your application needs three routes for SSO:

| Route | Purpose |
|-------|---------|
| `/auth/login` | Redirects to SSO login page |
| `/auth/register` | Redirects to SSO registration page |
| `/auth/callback` | Receives token after authentication |

See [Frontend Implementation](./04-frontend-implementation.md) for code examples.

---

## Step 5: Test the Flow

1. Start your application
2. Navigate to `/auth/login`
3. You should be redirected to the SSO login page
4. After logging in, you should return to `/auth/callback`
5. The callback page should extract the token and redirect to your dashboard

---

## Quick Reference

### Environment Variables (Client App)

| Variable | Example | Description |
|----------|---------|-------------|
| `NEXT_PUBLIC_SSO_URL` | `https://sso.example.com` | SSO service base URL |
| `NEXT_PUBLIC_TENANT_ID` | `uuid-here` | Your tenant identifier |

### Environment Variables (SSO Service)

| Variable | Description |
|----------|-------------|
| `TENANT_SECRET_KEY` | Secret for creating tenants via API |
| `API_KEY` | Required header (`X-API-Key`) for auth endpoints |
| `ALLOWED_ORIGINS` | Comma-separated list of allowed client domains |

### Required Headers by Endpoint

| Endpoint | Required Header |
|----------|-----------------|
| `POST /api/tenants` | `X-Tenant-Secret-Key` |
| `POST /auth/login` | `X-API-Key` |
| `POST /auth/register` | `X-API-Key` |
| `GET /auth/verify` | `Authorization: Bearer {token}` + `X-API-Key` |
| `DELETE /auth/logout` | `Authorization: Bearer {token}` + `X-API-Key` |

---

## Next Steps

→ [Configuration](./02-configuration.md) - Detailed environment setup  
→ [Frontend Implementation](./04-frontend-implementation.md) - Code examples
