# Redirect Parameters

This document details the URL parameters used in SSO redirects.

---

## Login Redirect URL

When initiating SSO login, redirect users to:

```
{SSO_URL}/login?tenant_id={TENANT_ID}&redirect_uri={CALLBACK_URL}&response_type=code&scope=openid&state={STATE}&nonce={NONCE}
```

---

## Parameter Reference

| Parameter | Required | Description |
|-----------|----------|-------------|
| `tenant_id` | ✅ Yes | UUID identifying your application |
| `redirect_uri` | ✅ Yes | URL-encoded callback URL in your app |
| `response_type` | ❌ No | Value `code` (OAuth2 compatibility) |
| `scope` | ❌ No | Value `openid` (OIDC compatibility) |
| `state` | ❌ No | Random string for CSRF protection |
| `nonce` | ❌ No | Random string for replay attack protection |

---

## Building the Redirect URL

### JavaScript Example

```javascript
function buildSSOLoginUrl(ssoUrl, tenantId, callbackPath) {
    const redirectUri = encodeURIComponent(
        `${window.location.origin}${callbackPath}`
    )
    
    // Generate security parameters
    const state = crypto.randomUUID()
    const nonce = crypto.randomUUID()
    
    // Store state for later validation (optional)
    sessionStorage.setItem('sso_state', state)
    
    return `${ssoUrl}/login?` + 
           `tenant_id=${tenantId}&` +
           `redirect_uri=${redirectUri}&` +
           `response_type=code&` +
           `scope=openid&` +
           `state=${state}&` +
           `nonce=${nonce}`
}

// Usage
const loginUrl = buildSSOLoginUrl(
    'https://sso.example.com',
    'your-tenant-uuid',
    '/auth/callback'
)

window.location.href = loginUrl
```

### Example Full URL

**Before encoding:**
```
https://sso.example.com/login?tenant_id=abc-123&redirect_uri=https://app.example.com/auth/callback&state=xyz789&nonce=qwe456
```

**With encoded redirect_uri:**
```
https://sso.example.com/login?tenant_id=abc-123&redirect_uri=https%3A%2F%2Fapp.example.com%2Fauth%2Fcallback&state=xyz789&nonce=qwe456
```

---

## Callback Response

After successful authentication, the SSO service redirects to:

```
{redirect_uri}#access_token={JWT_TOKEN}&state={STATE}
```

### Response Parameters

| Parameter | Description |
|-----------|-------------|
| `access_token` | JWT token for API authentication |
| `state` | Echo of the state parameter sent in the request |

### Example Callback URL

```
https://app.example.com/auth/callback#access_token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...&state=xyz789
```

---

## Registration Flow

For user registration, use the `/register` endpoint with the same parameters:

```
{SSO_URL}/register?tenant_id={TENANT_ID}&redirect_uri={CALLBACK_URL}&state={STATE}&nonce={NONCE}
```

After successful registration, users are redirected to the login page to sign in.

---

## Security: State Validation

To prevent CSRF attacks, validate the returned `state` parameter:

```javascript
function validateCallback() {
    const hash = window.location.hash.substring(1)
    const params = new URLSearchParams(hash)
    
    const returnedState = params.get('state')
    const savedState = sessionStorage.getItem('sso_state')
    
    if (returnedState !== savedState) {
        throw new Error('State mismatch - possible CSRF attack')
    }
    
    // Clear saved state
    sessionStorage.removeItem('sso_state')
    
    return params.get('access_token')
}
```

---

## Next Steps

→ [Frontend Implementation](./04-frontend-implementation.md) - Framework-specific code examples
