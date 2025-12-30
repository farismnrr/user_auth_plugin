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
│   ├── auth/
│   │   ├── login/
│   │   │   └── page.tsx      # Login redirect page
│   │   ├── register/
│   │   │   └── page.tsx      # Register redirect page
│   │   └── callback/
│   │       └── page.tsx      # Token handling page
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

### 2. Login Page

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
                    `${window.location.origin}/auth/callback`
                )

                // Generate security parameters
                const state = crypto.randomUUID()
                const nonce = crypto.randomUUID()

                setStatus('Redirecting to login...')
                window.location.href = 
                    `${ssoUrl}/login?` +
                    `tenant_id=${tenantId}&` +
                    `redirect_uri=${redirectUri}&` +
                    `response_type=code&` +
                    `scope=openid&` +
                    `state=${state}&` +
                    `nonce=${nonce}`
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
                // Store token securely
                sessionStorage.setItem('access_token', accessToken)

                // Clear hash from URL for security
                window.history.replaceState(null, '', window.location.pathname)

                // Navigate to protected area
                router.replace('/dashboard')
            } else {
                router.replace('/auth/login')
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
    const redirectToLogin = () => {
        const redirectUri = encodeURIComponent(
            `${window.location.origin}/auth/callback`
        )
        const state = crypto.randomUUID()
        const nonce = crypto.randomUUID()

        window.location.href = 
            `${config.ssoUrl}/login?` +
            `tenant_id=${config.tenantId}&` +
            `redirect_uri=${redirectUri}&` +
            `state=${state}&` +
            `nonce=${nonce}`
    }

    const redirectToRegister = () => {
        const redirectUri = encodeURIComponent(
            `${window.location.origin}/auth/callback`
        )
        const state = crypto.randomUUID()
        const nonce = crypto.randomUUID()

        window.location.href = 
            `${config.ssoUrl}/register?` +
            `tenant_id=${config.tenantId}&` +
            `redirect_uri=${redirectUri}&` +
            `state=${state}&` +
            `nonce=${nonce}`
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
            sessionStorage.setItem('access_token', accessToken)
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

    const redirectToLogin = () => {
        const redirectUri = encodeURIComponent(
            `${window.location.origin}/auth/callback`
        )
        const state = crypto.randomUUID()
        const nonce = crypto.randomUUID()

        window.location.href = 
            `${ssoUrl}/login?` +
            `tenant_id=${tenantId}&` +
            `redirect_uri=${redirectUri}&` +
            `state=${state}&` +
            `nonce=${nonce}`
    }

    const redirectToRegister = () => {
        const redirectUri = encodeURIComponent(
            `${window.location.origin}/auth/callback`
        )
        const state = crypto.randomUUID()
        const nonce = crypto.randomUUID()

        window.location.href = 
            `${ssoUrl}/register?` +
            `tenant_id=${tenantId}&` +
            `redirect_uri=${redirectUri}&` +
            `state=${state}&` +
            `nonce=${nonce}`
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

        function buildAuthUrl(endpoint) {
            const redirectUri = encodeURIComponent(
                `${window.location.origin}/callback.html`
            )
            const state = crypto.randomUUID()
            const nonce = crypto.randomUUID()

            return `${SSO_URL}/${endpoint}?` +
                   `tenant_id=${TENANT_ID}&` +
                   `redirect_uri=${redirectUri}&` +
                   `state=${state}&` +
                   `nonce=${nonce}`
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
            sessionStorage.setItem('access_token', accessToken)
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
</html>
```

---

## Next Steps

→ [Token Handling](./05-token-handling.md) - Managing and using access tokens
