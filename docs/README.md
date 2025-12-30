# SSO Integration Documentation

Welcome to the SSO Integration documentation for the Multi-Tenant User Management Service. This guide provides everything you need to integrate Single Sign-On (SSO) authentication into your frontend application.

---

## Quick Navigation

| Document | Description |
|----------|-------------|
| [01 - Overview](./01-overview.md) | Architecture and how SSO works |
| [02 - Getting Started](./02-configuration.md) | **Tenant setup**, environment variables |
| [03 - Redirect Parameters](./03-redirect-parameters.md) | URL format and query parameters |
| [04 - Frontend Implementation](./04-frontend-implementation.md) | Next.js, React, Vue, vanilla JS examples |
| [05 - Token Handling](./05-token-handling.md) | Parsing, storing, and using tokens |
| [06 - API Reference](./06-api-reference.md) | All endpoints with examples |
| [07 - Troubleshooting](./07-troubleshooting.md) | Common issues and solutions |

---

## Before You Start

> ⚠️ **Important**: You must create a tenant before integrating SSO.

1. **Create a Tenant** using `make create-tenant` (see [Getting Started](./02-configuration.md))
2. **Save the Tenant ID** returned from the command
3. **Add your domain** to `ALLOWED_ORIGINS` in the SSO service

---

## Quick Start

### 1. Create Tenant (SSO Service)

```bash
make create-tenant
# Enter: Tenant Name, Description
# Save the returned tenant_id
```

### 2. Configure Your App

```env
NEXT_PUBLIC_SSO_URL=https://sso.example.com
NEXT_PUBLIC_TENANT_ID=<tenant_id from step 1>
```

### 3. Add Auth Routes

```
/auth/login     → Redirect to SSO login
/auth/register  → Redirect to SSO register  
/auth/callback  → Handle token after auth
```

### 4. Redirect to SSO

```javascript
const ssoUrl = 'https://sso.example.com'
const tenantId = 'your-tenant-id'
const redirectUri = encodeURIComponent(`${window.location.origin}/auth/callback`)

window.location.href = `${ssoUrl}/login?tenant_id=${tenantId}&redirect_uri=${redirectUri}`
```

### 5. Handle Callback

```javascript
// In /auth/callback page
const hash = window.location.hash.substring(1)
const params = new URLSearchParams(hash)
const token = params.get('access_token')

if (token) {
    sessionStorage.setItem('access_token', token)
    window.location.href = '/dashboard'
}
```

---

## Reference Implementation

For a complete working example, see the [IoTNet-UI](https://github.com/i-otnet/IoTNet-UI) repository which implements SSO with Next.js.

---

## Key Endpoints

| Endpoint | Method | Auth | Description |
|----------|--------|------|-------------|
| `/login` | GET | - | SSO login page (redirect) |
| `/register` | GET | - | SSO register page (redirect) |
| `/api/tenants` | POST | `X-Tenant-Secret-Key` | Create tenant |
| `/auth/login` | POST | `X-API-Key` | Login via API |
| `/auth/register` | POST | `X-API-Key` | Register via API |
| `/auth/logout` | DELETE | `Bearer token` | Logout |
| `/auth/verify` | GET | `Bearer token` | Get user info |

---

*Last updated: December 2024*
