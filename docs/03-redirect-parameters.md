# Redirect Parameters

This document details the URL parameters used in SSO redirects.

---

## Login Redirect URL

When initiating SSO login, your client application redirects users to:

```
{SSO_URL}/login?tenant_id={TENANT_ID}&redirect_uri={CALLBACK_URL}
```

> **Note**: You only need to provide `tenant_id` and `redirect_uri`. The SSO login page automatically generates and appends `state`, `nonce`, `response_type`, and `scope` parameters for security.

---

## Parameter Reference

### Client-Provided (Required)

| Parameter | Required | Description |
|-----------|----------|-------------|
| `tenant_id` | ✅ Yes | UUID identifying your application |
| `redirect_uri` | ✅ Yes | URL-encoded callback URL in your app |

---

## Security: Redirect URI Validation

The SSO service maintains a whitelist of **Allowed Origins** (configured via `VITE_ALLOWED_ORIGINS`). 

1. **Strict Validation**: Upon reaching the login or registration page, the SSO router extracts the `redirect_uri`.
2. **Origin Match**: It validates that the protocol and host of the `redirect_uri` match one of the allowed origins.
3. **Forbidden Redirect**: If the URI is invalid or its origin is not in the whitelist, the user is automatically redirected to the `/forbidden` page and the login flow is blocked.

> [!IMPORTANT]
> Ensure your application's domain (e.g., `http://localhost:3000` or `https://app.example.com`) is correctly added to the SSO service configuration.

---

## Session Persistence

To ensure the SSO flow remains consistent across page refreshes or internal navigation, the service stores the following in `sessionStorage`:
- `sso_redirect_uri`: The target callback URL.
- `sso_tenant_id`: The application identifier.

These are cleared once the final redirect back to your client application is performed.

### SSO-Generated (Automatic)

These parameters are added by the SSO login page, **not by your client**:

| Parameter | Description |
|-----------|-------------|
| `response_type` | Value `code` (OAuth2 compatibility) |
| `scope` | Value `openid` (OIDC compatibility) |
| `state` | Random string for CSRF protection |
| `nonce` | Random string for replay attack protection |

> ⚠️ **Important**: Do NOT generate `state` and `nonce` in your client application. The SSO service handles this automatically via the `useSSO` composable when the login page loads.

---

## Building the Redirect URL

### Simple Approach (Recommended)

Your client only needs to send `tenant_id` and `redirect_uri`:

```javascript
function redirectToSSO(ssoUrl, tenantId, callbackPath) {
    const redirectUri = encodeURIComponent(
        `${window.location.origin}${callbackPath}`
    )
    
    // SSO will add state, nonce, response_type, and scope automatically
    window.location.href = `${ssoUrl}/login?tenant_id=${tenantId}&redirect_uri=${redirectUri}`
}

// Usage
redirectToSSO(
    'https://sso.example.com',
    'your-tenant-uuid',
    '/auth/callback'
)
```

### What Happens Behind the Scenes

1. Your app redirects to: `https://sso.example.com/login?tenant_id=abc&redirect_uri=https://app.example.com/callback`
2. SSO login page loads and the `useSSO` composable generates `state` and `nonce`
3. SSO updates the URL to include all parameters:
   ```
   https://sso.example.com/login?tenant_id=abc&redirect_uri=https://app.example.com/callback&response_type=code&scope=openid&state=xyz789&nonce=qwe456
   ```
4. After login, SSO redirects back with the token and state

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

For user registration, redirect to `/register` with only `tenant_id` and `redirect_uri`:

```
{SSO_URL}/register?tenant_id={TENANT_ID}&redirect_uri={CALLBACK_URL}
```

The SSO registration page will add `state` and `nonce` automatically.

After successful registration, users are redirected to the login page to sign in.

---

## Security: State Handling

> **Note**: State validation is handled internally by the SSO service. Your client application does not need to generate or validate `state`.

### How it works:

1. **SSO generates state**: When the login page loads, the `useSSO` composable generates `state` and `nonce`
2. **Stored in Pinia**: The values are stored in the SSO frontend's state management
3. **Sent with login**: When user logs in, `state` is included in the redirect to your callback
4. **SSO validates**: The SSO backend validates the `state` before issuing the token

### Callback Handling (Simple)

Your callback only needs to extract the token:

```javascript
function handleCallback() {
    const hash = window.location.hash.substring(1)
    const params = new URLSearchParams(hash)
    
    const accessToken = params.get('access_token')
    
    if (accessToken) {
        sessionStorage.setItem('access_token', accessToken)
        window.location.href = '/dashboard'
    } else {
        window.location.href = '/login'
    }
}
```

---

## Seamless Flow (Auto-Redirect)

If a user is **already authenticated** in the SSO service (e.g., they have a valid session cookie) and visits the login or registration page with SSO parameters:

1. **Verification**: The system detects the existing session.
2. **Auto-Redirect**: The SSO service immediately redirects the user back to the `redirect_uri` with a fresh `access_token` and the current `state`.
3. **User Experience**: The user experiences a seamless transition without seeing the login form again.

> [!NOTE]
> This behavior is handled automatically by the router guard in `router/index.js`.

---

## Next Steps

→ [Frontend Implementation](./04-frontend-implementation.md) - Framework-specific code examples
