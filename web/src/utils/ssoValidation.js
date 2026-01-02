import { ALLOWED_ORIGINS_CONFIG } from '../config'

// Allowed origins whitelist - priority:
// 1. Runtime config (from window.config)
// 2. Build-time env (VITE_ALLOWED_ORIGINS)
// 3. Defaults
const envOrigins = ALLOWED_ORIGINS_CONFIG || import.meta.env.VITE_ALLOWED_ORIGINS

const DEFAULT_ORIGINS = [
    'http://localhost:3000',
    'http://127.0.0.1:3000',
    'http://localhost:5500',
    'http://127.0.0.1:5500'
]

export const ALLOWED_ORIGINS = envOrigins
    ? envOrigins.split(',').map(o => o.trim()).filter(o => o.length > 0)
    : DEFAULT_ORIGINS

/**
 * Validates if a redirect_uri is in the allowed origins whitelist.
 * Extracts the origin (protocol + host) and performs exact match.
 * @param {string} redirectUri - The redirect URI to validate
 * @returns {boolean} - True if valid, false otherwise
 */
export const isValidRedirectUri = (redirectUri) => {
    if (!redirectUri) return true // No redirect_uri is okay (non-SSO flow)

    try {
        const url = new URL(redirectUri)

        // Explicitly allow only http and https protocols to prevent XSS (like javascript: urls)
        if (url.protocol !== 'http:' && url.protocol !== 'https:') {
            return false
        }

        const origin = `${url.protocol}//${url.host}`
        return ALLOWED_ORIGINS.some(allowed => allowed === origin)
    } catch {
        // Invalid URL format
        return false
    }
}
