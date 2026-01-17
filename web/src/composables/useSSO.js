import { onMounted, watch } from 'vue'
import { useRoute } from 'vue-router'
import { useAuthStore } from '../stores/auth'

export function useSSO() {
    const route = useRoute()
    const authStore = useAuthStore()

    const generateSSOParams = () => {
        const queryParams = new URLSearchParams(window.location.search)
        
        // Initial extraction from URL query params (priority)
        const paramRedirectUri = queryParams.get('redirect_uri')
        const paramTenantId = queryParams.get('tenant_id')
        const paramRole = queryParams.get('role')

        // If params exist in URL, save to storage
        if (paramRedirectUri) sessionStorage.setItem('sso_redirect_uri', paramRedirectUri)
        if (paramTenantId) sessionStorage.setItem('sso_tenant_id', paramTenantId)
        if (paramRole) sessionStorage.setItem('sso_role', paramRole)

        // Retrieve from storage
        const redirectUri = sessionStorage.getItem('sso_redirect_uri')
        const tenantId = sessionStorage.getItem('sso_tenant_id')
        const role = sessionStorage.getItem('sso_role')

        if (redirectUri && tenantId) {
            let state = authStore.ssoState
            let nonce = authStore.ssoNonce

            // Only generate new if not already in store (preserves across navigation)
            if (!state || !nonce) {
                state = Math.random().toString(36).substring(2, 15) + Math.random().toString(36).substring(2, 15)
                nonce = Math.random().toString(36).substring(2, 15) + Math.random().toString(36).substring(2, 15)

                // Store in Pinia
                authStore.ssoState = state
                authStore.ssoNonce = nonce
            }

            // Check if current URL already has these params to avoid redundant replaceState
            const currentParams = new URLSearchParams(window.location.search)
            if (currentParams.get('state') === state && currentParams.get('nonce') === nonce) {
                return
            }

            // Update URL with full params without redirecting
            let newUrl = `${window.location.pathname}?tenant_id=${tenantId}&redirect_uri=${redirectUri}&response_type=code&scope=openid&state=${state}&nonce=${nonce}`
            if (role) {
                newUrl += `&role=${role}`
            }
            window.history.replaceState({}, '', newUrl)
        }
    }

    onMounted(() => {
        generateSSOParams()
    })

    // Watch route path changes to ensure params are re-applied if lost (though RouterLink fix helps)
    watch(() => route.path, () => {
        generateSSOParams()
    })

    return {
        generateSSOParams
    }
}
