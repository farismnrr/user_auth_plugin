# Frontend Implementation

This document provides code examples for integrating SSO into various frontend frameworks.

---

## Table of Contents

- [Next.js](#nextjs)
- [React (Vite)](#react-vite)
- [Vue.js](#vuejs)
- [Vanilla JavaScript](#vanilla-javascript)

---

## Next.js

### Project Structure

```
src/
├── app/
│   ├── api/
│   │   └── config/
│   │       └── route.ts      # Server-side config endpoint
│   ├── login/
│   │   └── page.tsx      # Login redirect page
│   ├── register/
│   │   └── page.tsx      # Register redirect page
│   └── callback/
│       └── page.tsx      # Token handling page
│   └── dashboard/
│       └── page.tsx          # Protected page
└── store/
    └── auth.ts               # Auth state management
```

### 1. Config API Route

Create `src/app/api/config/route.ts`:

```typescript
import { NextResponse } from 'next/server'

export const dynamic = 'force-dynamic'

export async function GET() {
    return NextResponse.json({
        ssoUrl: process.env.NEXT_PUBLIC_SSO_URL || 'http://localhost:5500',
        tenantId: process.env.NEXT_PUBLIC_TENANT_ID || '',
    })
}
```

### 2. User Proxy Route
Create `src/app/api/auth/user/route.ts` to forward user requests to the backend:

```typescript
import { type NextRequest, NextResponse } from "next/server";

export async function GET(req: NextRequest) {
  try {
    const authHeader = req.headers.get("Authorization");
    if (!authHeader) {
      return NextResponse.json({ message: "No authorization header found" }, { status: 401 });
    }

    const ssoUrl = process.env.SSO_URL || "http://localhost:5500";
    // Call /auth/verify endpoint which validates token and returns user data
    const userEndpoint = `${ssoUrl}/auth/verify`;

    const backendResponse = await fetch(userEndpoint, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
        Authorization: authHeader,
        "X-API-Key": process.env.NEXT_PUBLIC_API_KEY || "", // Required for tenant validation
      },
    });

    if (!backendResponse.ok) {
        return NextResponse.json({ message: "Failed to fetch user" }, { status: backendResponse.status });
    }

    const data = await backendResponse.json();
    return NextResponse.json(data, { status: 200 });
  } catch (error) {
    return NextResponse.json({ message: "Internal server error" }, { status: 500 });
  }
}
```

```

### 3. Logout Proxy Route
Create `src/app/api/auth/logout/route.ts` to handle logout securely:

```typescript
import { type NextRequest, NextResponse } from "next/server";

export async function POST(req: NextRequest) {
  try {
    const authHeader = req.headers.get("Authorization");
    // Call backend/SSO logout endpoint
    const ssoUrl = process.env.SSO_URL || "http://localhost:5500";
    const response = await fetch(`${ssoUrl}/auth/logout`, {
      method: "POST",
      headers: { 
          Authorization: authHeader || "",
          "X-API-Key": process.env.NEXT_PUBLIC_API_KEY || ""
      },
    });

    return NextResponse.json({ success: true }, { status: 200 });
  } catch (error) {
    return NextResponse.json({ message: "Logout failed" }, { status: 500 });
  }
}
```

### 4. Login Page

Create `src/app/auth/login/page.tsx`:

```tsx
'use client'

import { useEffect, useState } from 'react'

export default function LoginPage() {
    const [status, setStatus] = useState('Initializing...')

    useEffect(() => {
        const initAuth = async () => {
            try {
                setStatus('Loading configuration...')
                const res = await fetch('/api/config')
                const config = await res.json()

                const ssoUrl = config.ssoUrl
                const tenantId = config.tenantId
                const redirectUri = encodeURIComponent(
                    `${window.location.origin}/callback`
                )

                // SSO will generate state/nonce automatically
                setStatus('Redirecting to login...')
                window.location.href = `${ssoUrl}/login?tenant_id=${tenantId}&redirect_uri=${redirectUri}`
            } catch (error) {
                console.error('Failed to load config:', error)
                setStatus('Error loading configuration')
            }
        }

        initAuth()
    }, [])

    return (
        <div className="flex min-h-screen items-center justify-center">
            <p className="text-gray-600">{status}</p>
        </div>
    )
}
```

### 3. Callback Page

Create `src/app/auth/callback/page.tsx`:

```tsx
'use client'

import { useEffect, Suspense } from 'react'
import { useRouter } from 'next/navigation'

function CallbackContent() {
    const router = useRouter()

    useEffect(() => {
        const processCallback = async () => {
            // Extract token from hash fragment
            const hash = window.location.hash.substring(1)
            const params = new URLSearchParams(hash)
            const accessToken = params.get('access_token')

            if (accessToken) {
                // Store token in memory (Zustand state)
                const { setAccessToken } = useAuthStore.getState()
                setAccessToken(accessToken)

                // Clear hash from URL for security
                window.history.replaceState(null, '', window.location.pathname)

                // Navigate to protected area
                router.replace('/dashboard')
            } else {
                router.replace('/login')
            }
        }

        processCallback()
    }, [router])

    return (
        <div className="flex min-h-screen items-center justify-center">
            <p className="text-gray-600">Authenticating...</p>
        </div>
    )
}

export default function CallbackPage() {
    return (
        <Suspense fallback={<div>Loading...</div>}>
            <CallbackContent />
        </Suspense>
    )
}
```

### 4. Auth Store (Zustand)

Create `src/store/auth.ts`:

```typescript
import { create } from 'zustand'

interface AuthState {
    accessToken: string | null
    setAccessToken: (token: string) => void
    clearAuth: () => void
    isAuthenticated: () => boolean
}

export const useAuthStore = create<AuthState>((set, get) => ({
    accessToken: null,
    setAccessToken: (token) => set({ accessToken: token }),
    clearAuth: () => set({ accessToken: null }),
    isAuthenticated: () => !!get().accessToken,

    // Refresh Token Logic (Cross-Origin Cookie)
    refresh: async () => {
        try {
            const SSO_URL = process.env.NEXT_PUBLIC_SSO_URL || "http://localhost:5500";
            const API_KEY = process.env.NEXT_PUBLIC_API_KEY || "";

            // Call Service directly to send the HttpOnly + SameSite=None cookie
            const response = await fetch(`${SSO_URL}/auth/refresh`, {
                method: "GET",
                headers: {
                    "Accept": "application/json",
                    "X-API-Key": API_KEY 
                },
                credentials: "include", // MIMIC ROW: Critical for sending cookies
            });

            if (response.ok) {
                const data = await response.json();
                const token = data.data?.access_token || data.access_token;
                if (token) {
                    set({ accessToken: token });
                }
            } else {
                get().clearAuth();
            }
        } catch (error) {
            console.error("Refresh failed", error);
            get().clearAuth();
        }
    },
    
    // Fetch User via Proxy
    fetchUser: async () => {
        const { accessToken } = get();
        if (!accessToken) return;

        try {
            const res = await fetch("/api/auth/user", {
                headers: { Authorization: `Bearer ${accessToken}` },
            });
            if (res.ok) {
                const userData = await res.json();
                console.log("User loaded:", userData);
            }
        } catch (error) {
            console.error("Fetch user failed:", error);
        }
    }
}))
```

---

## React (Vite)

### 1. Configuration

Create `src/config.ts`:

```typescript
export const config = {
    ssoUrl: import.meta.env.VITE_SSO_URL || 'http://localhost:5500',
    tenantId: import.meta.env.VITE_TENANT_ID || '',
}
```

### 2. SSO Hook

Create `src/hooks/useSSO.ts`:

```typescript
import { config } from '../config'

export function useSSO() {
    // SSO generates state/nonce automatically - client only needs tenant_id and redirect_uri
    const redirectToLogin = () => {
        const redirectUri = encodeURIComponent(
            `${window.location.origin}/callback`
        )
        window.location.href = `${config.ssoUrl}/login?tenant_id=${config.tenantId}&redirect_uri=${redirectUri}`
    }

    const redirectToRegister = () => {
        const redirectUri = encodeURIComponent(
            `${window.location.origin}/callback`
        )
        window.location.href = `${config.ssoUrl}/register?tenant_id=${config.tenantId}&redirect_uri=${redirectUri}`
    }

    return { redirectToLogin, redirectToRegister }
}
```

### 3. Callback Component

Create `src/pages/Callback.tsx`:

```tsx
import { useEffect } from 'react'
import { useNavigate } from 'react-router-dom'

export function CallbackPage() {
    const navigate = useNavigate()

    useEffect(() => {
        const hash = window.location.hash.substring(1)
        const params = new URLSearchParams(hash)
        const accessToken = params.get('access_token')

        if (accessToken) {
            // Store in memory (e.g. via a hook or state management)
            setAccessToken(accessToken)
            window.history.replaceState(null, '', window.location.pathname)
            navigate('/dashboard', { replace: true })
        } else {
            navigate('/login', { replace: true })
        }
    }, [navigate])

    return <div>Authenticating...</div>
}
```

---

## Vue.js

### 1. SSO Composable

Create `src/composables/useSSO.ts`:

```typescript
export function useSSO() {
    const ssoUrl = import.meta.env.VITE_SSO_URL || 'http://localhost:5500'
    const tenantId = import.meta.env.VITE_TENANT_ID || ''

    // SSO generates state/nonce automatically - client only needs tenant_id and redirect_uri
    const redirectToLogin = () => {
        const redirectUri = encodeURIComponent(
            `${window.location.origin}/callback`
        )
        window.location.href = `${ssoUrl}/login?tenant_id=${tenantId}&redirect_uri=${redirectUri}`
    }

    const redirectToRegister = () => {
        const redirectUri = encodeURIComponent(
            `${window.location.origin}/callback`
        )
        window.location.href = `${ssoUrl}/register?tenant_id=${tenantId}&redirect_uri=${redirectUri}`
    }

    return { redirectToLogin, redirectToRegister }
}
```

### 2. Auth Store (Pinia)

Create `src/stores/auth.ts`:

```typescript
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export const useAuthStore = defineStore('auth', () => {
    const accessToken = ref<string | null>(null)

    const isAuthenticated = computed(() => !!accessToken.value)

    const setAccessToken = (token: string) => {
        accessToken.value = token
    }

    const clearAuth = () => {
        accessToken.value = null
    }

    return { 
        accessToken, 
        isAuthenticated, 
        setAccessToken, 
        clearAuth 
    }
})
```

### 3. Callback View

Create `src/views/Callback.vue`:

```vue
<script setup lang="ts">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const authStore = useAuthStore()

onMounted(() => {
    const hash = window.location.hash.substring(1)
    const params = new URLSearchParams(hash)
    const accessToken = params.get('access_token')

    if (accessToken) {
        authStore.setAccessToken(accessToken)
        window.history.replaceState(null, '', window.location.pathname)
        router.replace('/dashboard')
    } else {
        router.replace('/login')
    }
})
</script>

<template>
    <div class="auth-loading">
        <p>Authenticating...</p>
    </div>
</template>
```

---

## Vanilla JavaScript

### Login Page

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Login</title>
</head>
<body>
    <button id="loginBtn">Sign In</button>
    <button id="registerBtn">Create Account</button>

    <script>
        const SSO_URL = 'https://sso.example.com'
        const TENANT_ID = 'your-tenant-uuid'

        // SSO generates state/nonce automatically
        function buildAuthUrl(endpoint) {
            const redirectUri = encodeURIComponent(
                `${window.location.origin}/callback.html`
            )
            return `${SSO_URL}/${endpoint}?tenant_id=${TENANT_ID}&redirect_uri=${redirectUri}`
        }

        document.getElementById('loginBtn').onclick = () => {
            window.location.href = buildAuthUrl('login')
        }

        document.getElementById('registerBtn').onclick = () => {
            window.location.href = buildAuthUrl('register')
        }
    </script>
</body>
</html>
```

### Callback Page

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Authenticating...</title>
</head>
<body>
    <p id="status">Processing authentication...</p>

    <script>
        const hash = window.location.hash.substring(1)
        const params = new URLSearchParams(hash)
        const accessToken = params.get('access_token')

        if (accessToken) {
            // Note: In vanilla JS, a global variable or custom state object 
            // should be used for in-memory storage.
            window.accessToken = accessToken 
            window.history.replaceState(null, '', window.location.pathname)
            window.location.href = '/dashboard.html'
        } else {
            document.getElementById('status').textContent = 
                'Authentication failed. Redirecting...'
            setTimeout(() => {
                window.location.href = '/login.html'
            }, 2000)
        }
    </script>
</body>
        }
    </script>
</body>
</html>
```

---

## Role-Based Access Control (RBAC)

Implementing RBAC in your frontend provides a better user experience by hiding restricted areas from unauthorized users. **Note:** Always rely on backend validation for data security.

### Decoding the Token

The access token is a standard JWT. You can decode it to access the user's role.

**Recommended Library:** `jwt-decode`

```bash
npm install jwt-decode
```

### Implementation Example (Next.js/React)

```typescript
import { jwtDecode } from "jwt-decode";

interface JwtPayload {
  sub: string;
  role: "admin" | "user";
  tenant_id: string;
  exp: number;
}

export const getUserRole = (token: string): string | null => {
  try {
    const decoded = jwtDecode<JwtPayload>(token);
    return decoded.role;
  } catch (error) {
    return null;
  }
};

// Protected Route Component
export function AdminGuard({ children }: { children: React.ReactNode }) {
  const { accessToken } = useAuthStore();
  const role = accessToken ? getUserRole(accessToken) : null;

  if (role !== 'admin') {
    return <div>Access Denied: Admins Only</div>;
  }

  return <>{children}</>;
}
```

## Multi-Tenant Registration

### Scenario: Existing User Linking to New Tenant

When a user tries to register with an email that already exists, they can link their existing account to the new tenant by providing their password.

1. **Frontend receives 409 Conflict** (if password entered is wrong or account already exists but needs verification).
2. **Frontend should display**: "This email is already registered. Please enter your password to link this account to [Tenant Name]."
3. **User enters correct password**.
4. **Backend verifies password and links account**.

### Error Handling for Registration

| Error Code | Message | Action |
|------------|---------|--------|
| 409 | "Invalid credentials for account linking" | User provided wrong password while trying to link existing account to a new tenant/role. |
| 409 | "Already registered in this tenant with role: {role}" | User is already a member of this tenant. Redirect to login. |
| 409 | "Username already exists" | Username taken by another account. |
| 403 | "Invalid or missing invitation code" | Request a valid invitation code (required for admin roles). |
| 400 | "Missing required fields" | Ensure all fields (username, email, password, role) are sent. |

## Next Steps

→ [Token Handling](./05-token-handling.md) - Managing and using access tokens
