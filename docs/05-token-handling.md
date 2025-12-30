# Token Handling

This document covers parsing, storing, and using access tokens from the SSO service.

---

## Parsing the Token

The access token is returned in the URL hash fragment after successful authentication:

```
https://your-app.com/auth/callback#access_token=eyJhbG...&state=xyz
```

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

The most secure approach is storing tokens in application memory:

```typescript
// Zustand (React)
const useAuthStore = create((set) => ({
    accessToken: null,
    setAccessToken: (token) => set({ accessToken: token }),
}))

// Pinia (Vue)
const useAuthStore = defineStore('auth', () => {
    const accessToken = ref(null)
    return { accessToken }
})
```

**Pros:** Cannot be accessed by XSS attacks  
**Cons:** Token lost on page refresh

### Alternative: Session Storage

For persistence across page navigation:

```javascript
// Store
sessionStorage.setItem('access_token', token)

// Retrieve
const token = sessionStorage.getItem('access_token')

// Clear
sessionStorage.removeItem('access_token')
```

**Pros:** Survives page refresh, cleared when browser tab closes  
**Cons:** Vulnerable to XSS if not careful

### Not Recommended: Local Storage

```javascript
// Avoid for sensitive tokens
localStorage.setItem('access_token', token)
```

**Cons:** Persists indefinitely, more exposed to XSS attacks

---

## Using the Token

### API Requests with Fetch

```javascript
async function fetchProtectedData() {
    const token = sessionStorage.getItem('access_token')
    
    const response = await fetch('https://api.example.com/users/me', {
        method: 'GET',
        headers: {
            'Authorization': `Bearer ${token}`,
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
    const token = sessionStorage.getItem('access_token')
    if (token) {
        config.headers.Authorization = `Bearer ${token}`
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
    // Clear stored token
    sessionStorage.removeItem('access_token')
    
    // Redirect to login
    window.location.href = '/auth/login'
}
```

### Proactive Refresh (Optional)

For better UX, check token expiry before it expires:

```javascript
function scheduleTokenCheck() {
    const token = sessionStorage.getItem('access_token')
    if (!token) return
    
    const payload = JSON.parse(atob(token.split('.')[1]))
    const expiresAt = payload.exp * 1000
    const checkAt = expiresAt - (5 * 60 * 1000) // 5 minutes before expiry
    
    const delay = Math.max(0, checkAt - Date.now())
    
    setTimeout(() => {
        // Either refresh token or prompt user to re-login
        console.log('Token expiring soon, please re-authenticate')
    }, delay)
}
```

---

## Complete Token Flow Example

```javascript
// 1. User clicks login
function initiateLogin() {
    window.location.href = buildSSOUrl('/login')
}

// 2. Handle callback
function handleCallback() {
    const { accessToken, state } = extractTokenFromHash()
    
    if (!accessToken) {
        throw new Error('No token received')
    }
    
    // Validate state (CSRF protection)
    const savedState = sessionStorage.getItem('sso_state')
    if (state !== savedState) {
        throw new Error('State mismatch')
    }
    
    // Store token
    sessionStorage.setItem('access_token', accessToken)
    sessionStorage.removeItem('sso_state')
    
    // Clear URL
    window.history.replaceState(null, '', window.location.pathname)
    
    // Redirect to app
    window.location.href = '/dashboard'
}

// 3. Use token for API calls
async function getUser() {
    const token = sessionStorage.getItem('access_token')
    const response = await fetch('/api/users/me', {
        headers: { 'Authorization': `Bearer ${token}` }
    })
    return response.json()
}

// 4. Logout
async function logout() {
    const token = sessionStorage.getItem('access_token')
    
    await fetch('/auth/logout', {
        method: 'DELETE',
        headers: { 'Authorization': `Bearer ${token}` }
    })
    
    sessionStorage.removeItem('access_token')
    window.location.href = '/auth/login'
}
```

---

## Next Steps

→ [API Reference](./06-api-reference.md) - Available endpoints and authentication methods
