// Allowed origins whitelist - must match backend ALLOWED_ORIGINS
export const ALLOWED_ORIGINS = [
    'http://localhost:3000',
    'http://127.0.0.1:3000',
    'http://localhost:5500',
    'http://127.0.0.1:5500',
    'https://app.farismunir.my.id',
    'https://sso.farismunir.my.id',
    'https://app.iotunnel.my.id',
    'https://sso.iotunnel.my.id'
]

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
        const origin = `${url.protocol}//${url.host}`
        return ALLOWED_ORIGINS.some(allowed => allowed === origin)
    } catch {
        // Invalid URL format
        return false
    }
}
