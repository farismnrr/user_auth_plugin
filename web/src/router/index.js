import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { isValidRedirectUri } from '../utils/ssoValidation'

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            redirect: '/login'
        },
        {
            path: '/login',
            name: 'login',
            component: () => import('../views/Login.vue'),
            meta: { guestOnly: true, title: 'Sign In' }
        },
        {
            path: '/register',
            name: 'register',
            component: () => import('../views/Register.vue'),
            meta: { guestOnly: true, title: 'Create Account' }
        },
        {
            path: '/forbidden',
            name: 'forbidden',
            component: () => import('../views/Forbidden.vue'),
            meta: { title: 'Access Forbidden' }
        }
    ]
})

// SSO: Store redirect params in sessionStorage when navigating to login/register
router.beforeEach(async (to, from, next) => {
    const authStore = useAuthStore()

    // Parse SSO params from URL for login/register pages
    // Store redirect_uri and tenant_id in sessionStorage
    // state and nonce will be generated fresh in the component
    if (to.name === 'login' || to.name === 'register') {
        const redirectUri = to.query.redirect_uri
        const tenantId = to.query.tenant_id

        // Validate redirect_uri against whitelist
        if (redirectUri && !isValidRedirectUri(redirectUri)) {
            // Invalid redirect_uri - redirect to forbidden page
            console.warn('[SSO Security] Blocked invalid redirect_uri:', redirectUri)
            return next({ name: 'forbidden' })
        }

        if (redirectUri) {
            sessionStorage.setItem('sso_redirect_uri', redirectUri)
            if (tenantId) sessionStorage.setItem('sso_tenant_id', tenantId)
        }
    }

    if (to.meta.title) {
        document.title = `${to.meta.title} - IoTNet`
    }

    if (!authStore.isInitialized) {
        await authStore.refreshToken()
    }

    // Only login and register routes exist, both are guest-only
    if (to.meta.guestOnly && authStore.isAuthenticated) {
        // User is authenticated but trying to access guest-only page
        // Redirect back to calling app if SSO, otherwise stay
        const redirectUri = sessionStorage.getItem('sso_redirect_uri')
        if (redirectUri) {
            const state = authStore.ssoState || ''
            sessionStorage.removeItem('sso_redirect_uri')
            sessionStorage.removeItem('sso_tenant_id')
            authStore.ssoState = null
            authStore.ssoNonce = null
            const separator = redirectUri.includes('?') ? '&' : '?'
            window.location.href = `${redirectUri}${separator}access_token=${authStore.accessToken}&state=${state}`
        }
    } else {
        next()
    }
})

export default router
