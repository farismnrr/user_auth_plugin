# Troubleshooting

This document covers common issues and their solutions when integrating SSO.

---

## CORS Errors

### Problem

```
Access to fetch at 'https://sso.example.com/...' from origin 'https://app.example.com' 
has been blocked by CORS policy
```

### Solution

Ensure your application domain is added to `VITE_ALLOWED_ORIGINS` in the SSO service:

```env
VITE_ALLOWED_ORIGINS=http://localhost:3000,https://app.example.com
```

After updating, restart the SSO service:

```bash
make restart
# or
docker-compose restart
```

---

## Token Not Found in Callback

### Problem

`access_token` is `null` or `undefined` after redirect

### Checklist

1. **Check redirect_uri encoding**
   ```javascript
   // Correct
   const redirectUri = encodeURIComponent('https://app.example.com/callback')
   
   // Wrong - not encoded
   const redirectUri = 'https://app.example.com/callback'
   ```

2. **Verify tenant_id is valid**
   - Confirm the tenant exists in the SSO database
   - Check for typos in the UUID

3. **Check for JavaScript errors**
   - Open browser DevTools (F12)
   - Look for errors in the Console tab

4. **Verify hash fragment parsing**
   ```javascript
   // The token is in the hash (#), not query string (?)
   const hash = window.location.hash.substring(1)  // Correct
   const params = new URLSearchParams(window.location.search)  // Wrong
   ```

---

## Invalid Tenant Error

### Problem

SSO returns "Tenant not found" or similar error

### Solution

1. Verify the tenant ID is correct in your environment variables
2. Check if the tenant is active in the SSO database
3. Contact the SSO administrator to:
   - Create a new tenant
   - Verify the tenant UUID
   - Check if the tenant is not soft-deleted

---

## Token Expired (401 Unauthorized)

### Problem

API requests fail with 401 status after some time

### Solution

Implement token expiry handling:

```javascript
// Check before each request
function isTokenExpired(token) {
    try {
        const payload = JSON.parse(atob(token.split('.')[1]))
        return Date.now() >= payload.exp * 1000
    } catch {
        return true
    }
}

// Redirect to login if expired
function requireAuth() {
    const token = sessionStorage.getItem('access_token')
    
    if (!token || isTokenExpired(token)) {
        sessionStorage.removeItem('access_token')
        window.location.href = '/auth/login'
        return null
    }
    
    return token
}
```

---

## State Mismatch Error

### Problem

User sees state mismatch error after SSO redirect

### Understanding State in This SSO

> **Note**: In this SSO implementation, `state` and `nonce` are generated and validated **by the SSO service itself**, not by your client application. You should NOT generate `state` in your client.

### Causes

This error is rare but can occur when:

1. User has multiple SSO tabs open simultaneously
2. Browser session/storage was cleared during login
3. User's browser blocks sessionStorage

### Solution

Since state is handled by SSO, your client callback should simply extract the token:

```javascript
function handleCallback() {
    const hash = window.location.hash.substring(1)
    const params = new URLSearchParams(hash)
    const accessToken = params.get('access_token')
    
    if (accessToken) {
        sessionStorage.setItem('access_token', accessToken)
        window.location.href = '/dashboard'
    } else {
        // No token received - redirect back to login
        window.location.href = '/auth/login'
    }
}
```

If users consistently see state errors, check SSO service logs for details.

---

## Access Forbidden (Forbidden Page)

### Problem

User is redirected to the `/forbidden` page immediately upon opening the login or registration link.

### Cause

This occurs when the `redirect_uri` provided in the URL does not match any entry in the `VITE_ALLOWED_ORIGINS` whitelist. This is a security feature to prevent token leakage to unauthorized domains.

### Solution

1. **Verify the URL**: Check the `redirect_uri` parameter in your browser's address bar.
2. **Check Configuration**: Ensure the origin (protocol + host, e.g., `http://localhost:3000`) matches exactly.
3. **Update SSO Environment**:
   - Open the SSO service `.env` file.
   - Add your origin to `VITE_ALLOWED_ORIGINS` (comma-separated).
   - **Restart the service** to apply changes.
4. **Encoding**: Ensure the `redirect_uri` is properly URL-encoded when redirecting to SSO.

---

## Redirect Loop

### Problem

Application keeps redirecting between app and SSO

### Causes

1. Token not being stored properly
2. Auth check running on callback page
3. Race condition in state management

### Solution

1. **Exclude callback from auth checks**
   ```javascript
   // In your auth guard
   const publicPaths = ['/auth/login', '/auth/callback', '/auth/register']
   
   if (publicPaths.includes(window.location.pathname)) {
       return // Don't check auth on these pages
   }
   ```

2. **Add logging to debug**
   ```javascript
   function handleCallback() {
       console.log('Callback URL:', window.location.href)
       console.log('Hash:', window.location.hash)
       
       const params = new URLSearchParams(window.location.hash.substring(1))
       console.log('Token received:', !!params.get('access_token'))
       
       // ... rest of handler
   }
   ```

---

## Network Error During Login

### Problem

SSO login page fails to load or returns network error

### Checklist

1. **Verify SSO service is running**
   ```bash
   curl https://sso.example.com/health
   ```

2. **Check SSL certificates** (for HTTPS)
   - Ensure certs are valid and not expired
   - For localhost, use HTTP or proper self-signed certs

3. **Check firewall/security groups**
   - Ensure the SSO port is accessible
   - Check VPN requirements

---

## Docker-Specific Issues

### Problem

Works locally but fails in Docker

### Solutions

1. **Use correct network hostnames**
   ```env
   # Wrong - localhost refers to the container itself
   NEXT_PUBLIC_SSO_URL=http://localhost:5500
   
   # Correct - use service name or external URL
   NEXT_PUBLIC_SSO_URL=http://sso-service:5500  # Internal
   NEXT_PUBLIC_SSO_URL=https://sso.example.com   # External
   ```

2. **Ensure containers are on the same network**
   ```yaml
   # docker-compose.yml
   services:
     frontend:
       networks:
         - app-network
     sso:
       networks:
         - app-network
   
   networks:
     app-network:
   ```

---

## Debug Mode

Enable debug logging in your application:

```javascript
// Debug utility
function debugSSO(message, data = {}) {
    if (process.env.NODE_ENV === 'development') {
        console.log(`[SSO Debug] ${message}`, data)
    }
}

// Usage
debugSSO('Initiating login', { 
    ssoUrl, 
    tenantId, 
    redirectUri 
})

debugSSO('Callback received', { 
    hash: window.location.hash,
    hasToken: !!accessToken 
})
```

---

## Getting Help

If issues persist:

1. Check the SSO service logs:
   ```bash
   docker logs user-management-service
   # or
   tail -f logs/app.log
   ```

2. Verify configuration against working examples:
   - [IoTNet-UI Repository](https://github.com/i-otnet/IoTNet-UI)

3. Ensure all environment variables match between environments

---

## Back to Documentation

← [README](./README.md) - Documentation index
