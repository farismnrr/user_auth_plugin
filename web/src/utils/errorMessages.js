/**
 * Error Message Utility
 * Maps technical errors to user-friendly messages and categorizes them
 */

export const ERROR_TYPES = {
    CREDENTIAL: 'credential',  // Show inline
    NETWORK: 'network',        // Show toast
    SYSTEM: 'system'           // Show toast
}

/**
 * Parse error and return user-friendly message with error type
 * @param {Error} error - The error object from API call
 * @returns {{ message: string, type: string }}
 */
export function parseError(error) {
    // Network errors (no response from server)
    if (!error.response) {
        return {
            message: 'Unable to connect. Please check your internet connection.',
            type: ERROR_TYPES.NETWORK
        }
    }

    const status = error.response?.status
    const apiMessage = error.response?.data?.message

    // Map status codes to user-friendly messages
    switch (status) {
        // Credential/Authentication errors (4xx - user errors)
        case 400:
            return {
                message: apiMessage || 'Invalid request. Please check your input.',
                type: ERROR_TYPES.CREDENTIAL
            }
        case 401:
            return {
                message: apiMessage || 'Invalid username or password.',
                type: ERROR_TYPES.CREDENTIAL
            }
        case 403:
            return {
                message: 'Access denied. Please contact your administrator.',
                type: ERROR_TYPES.CREDENTIAL
            }
        case 409:
            return {
                message: apiMessage || 'This username or email is already taken.',
                type: ERROR_TYPES.CREDENTIAL
            }
        case 422:
            return {
                message: apiMessage || 'Please check your input and try again.',
                type: ERROR_TYPES.CREDENTIAL
            }

        // Network/System errors (4xx/5xx - system errors)
        case 404:
            // Handle specific resource not found errors from API (e.g. "User not found")
            if (apiMessage) {
                return {
                    message: apiMessage,
                    type: ERROR_TYPES.CREDENTIAL
                }
            }
            return {
                message: 'Authentication service not found. Please contact support.',
                type: ERROR_TYPES.NETWORK
            }
        case 408:
            return {
                message: 'Request timeout. Please try again.',
                type: ERROR_TYPES.NETWORK
            }
        case 429:
            return {
                message: 'Too many attempts. Please wait a moment and try again.',
                type: ERROR_TYPES.NETWORK
            }
        case 500:
            return {
                message: 'Something went wrong on our end. Please try again.',
                type: ERROR_TYPES.SYSTEM
            }
        case 502:
        case 503:
            return {
                message: 'Service temporarily unavailable. Please try again later.',
                type: ERROR_TYPES.SYSTEM
            }
        case 504:
            return {
                message: 'Server timeout. Please try again.',
                type: ERROR_TYPES.SYSTEM
            }

        // Default fallback
        default:
            if (status >= 500) {
                return {
                    message: 'Something went wrong. Please try again.',
                    type: ERROR_TYPES.SYSTEM
                }
            }
            return {
                message: apiMessage || 'An error occurred. Please try again.',
                type: ERROR_TYPES.CREDENTIAL
            }
    }
}

/**
 * Get user-friendly error message for specific contexts
 */
export const ERROR_MESSAGES = {
    LOGIN_FAILED: 'Invalid username or password.',
    REGISTRATION_FAILED: 'Unable to create account. Please try again.',
    PASSWORD_MISMATCH: 'Passwords do not match.',
    NETWORK_ERROR: 'Unable to connect. Please check your connection.',
    SERVER_ERROR: 'Something went wrong. Please try again.',
    SSO_CALLBACK_FAILED: 'Sign-in process failed. Please try again or contact your administrator.',
    SESSION_EXPIRED: 'Your session has expired. Please sign in again.',
    ACCOUNT_LOCKED: 'Your account has been locked. Please contact support.'
}
