# Token Handling

This document covers parsing, storing, and using access tokens from the SSO service.

---

## Parsing the Token

The access token is returned in the URL hash fragment after successful authentication:

```
https://your-app.com/callback#access_token=eyJhbG...&state=xyz
```

> [!IMPORTANT]
> The `refresh_token` is NOT present in the URL hash. It is handled exclusively via a `HttpOnly`, `SameSite=None` cookie set on the SSO service domain.

### Extraction Code

```javascript
function extractTokenFromHash() {
    // Get hash without the '#' character
    const hash = window.location.hash.substring(1)
    
    // Parse as URL parameters
    const params = new URLSearchParams(hash)
    
    return {
        accessToken: params.get('access_token'),
        state: params.get('state'),
    }
}
```

### Security: Clear URL Hash

Always remove the hash from the URL after extracting the token:

```javascript
window.history.replaceState(null, '', window.location.pathname)
```

This prevents the token from being:
- Visible in the browser address bar
- Saved in browser history
- Leaked via the Referer header

---

## Token Storage

### Recommended: In-Memory State

The most secure approach is storing tokens in application memory (state). This prevents the token from being accessed by malicious scripts (XSS) that might try to read from storage.

```typescript
// Example: Zustand Store (React)
const useAuthStore = create((set) => ({
    accessToken: null,
    setAccessToken: (token) => set({ accessToken: token }),
    clearAuth: () => set({ accessToken: null }),
}))

// Example: Pinia Store (Vue)
const useAuthStore = defineStore('auth', () => {
    const accessToken = ref(null)
    const setAccessToken = (token) => { accessToken.value = token }
    const clearAuth = () => { accessToken.value = null }
    return { accessToken, setAccessToken, clearAuth }
})
```

> [!TIP]
> Since the token is lost on page refresh, you should implement a **silent refresh** logic that calls the `/auth/refresh` endpoint when the application initializes.

---

## Using the Token

### API Requests with Fetch

```javascript
async function fetchProtectedData() {
    const { accessToken } = useAuthStore.getState() // Get from store
    
    const response = await fetch('https://api.example.com/users/me', {
        method: 'GET',
        headers: {
            'Authorization': `Bearer ${accessToken}`,
            'Content-Type': 'application/json',
        },
    })
    
    if (response.status === 401) {
        // Token expired or invalid
        handleTokenExpiry()
        return
    }
    
    return response.json()
}
```

### API Requests with Axios

```javascript
import axios from 'axios'

const api = axios.create({
    baseURL: 'https://api.example.com',
})

// Request interceptor - add token to all requests
api.interceptors.request.use((config) => {
    const { accessToken } = useAuthStore.getState()
    if (accessToken) {
        config.headers.Authorization = `Bearer ${accessToken}`
    }
    return config
})

// Response interceptor - handle token expiry
api.interceptors.response.use(
    (response) => response,
    (error) => {
        if (error.response?.status === 401) {
            handleTokenExpiry()
        }
        return Promise.reject(error)
    }
)

export default api
```

---

## Token Expiry Handling

### Detect Expiry

JWT tokens contain an expiration timestamp. You can decode and check it:

```javascript
function isTokenExpired(token) {
    try {
        const payload = JSON.parse(atob(token.split('.')[1]))
        const expiry = payload.exp * 1000 // Convert to milliseconds
        return Date.now() >= expiry
    } catch {
        return true // Treat invalid tokens as expired
    }
}
```

### Handle Expiry

```javascript
function handleTokenExpiry() {
    // Clear state
    const { clearAuth } = useAuthStore.getState()
    clearAuth()
    
    // Redirect to login
    window.location.href = '/login'
}
```

---

## Complete Token Flow Example

```javascript
// 1. User clicks login - only tenant_id and redirect_uri needed
function initiateLogin() {
    const redirectUri = encodeURIComponent(`${window.location.origin}/callback`)
    window.location.href = `${SSO_URL}/login?tenant_id=${TENANT_ID}&redirect_uri=${redirectUri}`
}

// 2. Handle callback - SSO handles state validation internally
function handleCallback() {
    const hash = window.location.hash.substring(1)
    const params = new URLSearchParams(hash)
    const accessToken = params.get('access_token')
    
    if (!accessToken) {
        throw new Error('No token received')
    }
    
    // Store token in memory state
    const { setAccessToken } = useAuthStore.getState()
    setAccessToken(accessToken)
    
    // Clear URL
    window.history.replaceState(null, '', window.location.pathname)
    
    // Redirect to app
    window.location.href = '/dashboard'
}

// 3. Use token for API calls
async function getUser() {
    const { accessToken } = useAuthStore.getState()
    const response = await fetch('/api/users/me', {
        headers: { 'Authorization': `Bearer ${accessToken}` }
    })
    return response.json()
}

// 4. Logout
async function logout() {
    const { accessToken, clearAuth } = useAuthStore.getState()
    
    await fetch('/auth/logout', {
        method: 'DELETE',
        headers: { 
            'Authorization': `Bearer ${accessToken}`,
            'X-API-Key': API_KEY
        }
    })
    
    clearAuth()
    window.location.href = '/login'
}
```

---

## Next Steps

→ [API Reference](./06-api-reference.md) - Available endpoints and authentication methods
