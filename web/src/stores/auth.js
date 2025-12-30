import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import router from '../router'
import AuthService from '../services/auth.service'
import { useToast } from '../composables/useToast'
import { parseError, ERROR_TYPES } from '../utils/errorMessages'

export const useAuthStore = defineStore('auth', () => {
    const toast = useToast()

    // State
    const user = ref(null)
    const accessToken = ref(null)
    const loading = ref(false)
    const error = ref(null)
    const isInitialized = ref(false)

    // SSO state (in memory, not persisted)
    const ssoState = ref(null)
    const ssoNonce = ref(null)

    // Getters
    const isAuthenticated = computed(() => !!accessToken.value)

    // Actions
    const login = async (emailOrUsername, password) => {
        loading.value = true
        error.value = null
        try {
            // Prepare SSO params
            const urlParams = new URLSearchParams(window.location.search)
            const redirectUri = urlParams.get('redirect_uri') || sessionStorage.getItem('sso_redirect_uri')

            const ssoParams = {
                state: ssoState.value,
                nonce: ssoNonce.value,
                redirect_uri: redirectUri
            }

            const response = await AuthService.login(emailOrUsername, password, ssoParams)

            // Response structure: { status, message, data: { access_token } }
            if (response?.data?.access_token) {
                const { access_token } = response.data
                accessToken.value = access_token

                // Fetch user details immediately using the new token
                try {
                    const verifyResponse = await AuthService.verify(access_token)
                    if (verifyResponse?.data?.user) {
                        user.value = verifyResponse.data.user
                    }
                } catch (e) {
                    // console.error("Failed to fetch user details:", e)
                    // Optional: handle error, maybe logout if verify fails?
                }

                // SSO: Check for redirect params from URL query first
                const urlParams = new URLSearchParams(window.location.search)
                const redirectUri = urlParams.get('redirect_uri') || sessionStorage.getItem('sso_redirect_uri')
                const state = ssoState.value || ''

                if (redirectUri) {
                    // Clear SSO data
                    sessionStorage.removeItem('sso_redirect_uri')
                    sessionStorage.removeItem('sso_tenant_id')
                    ssoState.value = null
                    ssoNonce.value = null

                    // Redirect back to calling app with access_token in hash fragment
                    const finalUrl = `${redirectUri}#access_token=${access_token}&state=${state}`

                    // Don't set loading=false, just redirect immediately
                    window.location.href = finalUrl
                    // Exit early - don't run finally block
                    return
                }

                // No SSO redirect - show success message
                toast.success('Login successful! You can close this window.')
            }
        } catch (err) {
            // console.error(err)
            const { message, type } = parseError(err)

            if (type === ERROR_TYPES.CREDENTIAL) {
                // Show inline error for credential issues
                error.value = message
            } else {
                // Show toast for network/system errors
                toast.error(message)
                error.value = null
            }
        } finally {
            // Only set loading=false if we didn't redirect
            loading.value = false
        }
    }

    const register = async (username, email, password, role) => {
        loading.value = true
        error.value = null
        try {
            // Prepare SSO params
            const ssoParams = {
                state: ssoState.value,
                nonce: ssoNonce.value,
                redirect_uri: sessionStorage.getItem('sso_redirect_uri')
            }

            await AuthService.register(username, email, password, role, ssoParams)
            toast.success('Registration successful! Please sign in to continue.')

            // SSO: Preserve redirect params when going to login
            // Note: state/nonce will be regenerated fresh on login page
            const redirectUri = sessionStorage.getItem('sso_redirect_uri')
            if (redirectUri) {
                const tenantId = sessionStorage.getItem('sso_tenant_id') || ''
                router.push({
                    name: 'login',
                    query: { redirect_uri: redirectUri, tenant_id: tenantId }
                })
            } else {
                router.push('/login')
            }
        } catch (err) {
            // console.error(err)
            const { message, type } = parseError(err)

            if (type === ERROR_TYPES.CREDENTIAL) {
                // Show inline error for credential issues
                error.value = message
            } else {
                // Show toast for network/system errors
                toast.error(message)
                error.value = null
            }
        } finally {
            loading.value = false
        }
    }

    const logout = async () => {
        try {
            if (accessToken.value) {
                await AuthService.logout(accessToken.value)
            }
        } catch (err) {
            // console.error("Logout error", err)
        } finally {
            user.value = null
            accessToken.value = null
            router.push('/login')
        }
    }

    const refreshToken = async () => {
        if (loading.value) return

        loading.value = true
        try {
            const data = await AuthService.refresh()
            if (data?.access_token) {
                accessToken.value = data.access_token
                // Note: user info might need to be fetched if not in refresh response
            }
        } catch {
            // console.log("No valid session found or refresh failed.")
        } finally {
            loading.value = false
            isInitialized.value = true
        }
    }

    return {
        // State
        user,
        accessToken,
        loading,
        error,
        isInitialized,
        ssoState,
        ssoNonce,
        // Getters
        isAuthenticated,
        // Actions
        login,
        register,
        logout,
        refreshToken
    }
})
