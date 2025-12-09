/**
 * K6 Utility Functions
 * 
 * Helper functions for k6 tests
 */

import { check } from 'k6';

/**
 * Generate random string
 */
export function randomString(length = 10) {
    const chars = 'abcdefghijklmnopqrstuvwxyz0123456789';
    let result = '';
    for (let i = 0; i < length; i++) {
        result += chars.charAt(Math.floor(Math.random() * chars.length));
    }
    return result;
}

/**
 * Generate random email
 */
export function randomEmail() {
    return `test_${randomString(8)}@example.com`;
}

/**
 * Generate random username
 */
export function randomUsername() {
    return `user_${randomString(8)}`;
}

/**
 * Generate random password
 */
export function randomPassword() {
    return `Pass${randomString(8)}123!`;
}

/**
 * Extract access token from response
 */
export function extractAccessToken(response) {
    const body = JSON.parse(response.body);
    return body.data?.access_token || null;
}

/**
 * Extract user ID from response
 */
export function extractUserId(response) {
    const body = JSON.parse(response.body);
    return body.data?.id || body.data?.user?.id || null;
}

/**
 * Extract refresh token from cookies
 */
export function extractRefreshToken(response) {
    const cookies = response.cookies;
    if (cookies && cookies.refresh_token && cookies.refresh_token.length > 0) {
        return cookies.refresh_token[0].value;
    }
    return null;
}

/**
 * Check if response is successful
 */
export function checkSuccess(response, expectedStatus = 200, checkMessage = null) {
    const checks = {
        [`status is ${expectedStatus}`]: (r) => r.status === expectedStatus,
        'response has body': (r) => r.body && r.body.length > 0,
    };

    if (expectedStatus >= 200 && expectedStatus < 300) {
        checks['success is true'] = (r) => {
            const body = JSON.parse(r.body);
            return body.success === true;
        };
    }

    if (checkMessage) {
        checks[`message contains "${checkMessage}"`] = (r) => {
            const body = JSON.parse(r.body);
            return body.message && body.message.includes(checkMessage);
        };
    }

    return check(response, checks);
}

/**
 * Check if response is error
 */
export function checkError(response, expectedStatus, expectedMessageContains = null) {
    const checks = {
        [`status is ${expectedStatus}`]: (r) => r.status === expectedStatus,
        'response has body': (r) => r.body && r.body.length > 0,
    };

    if (expectedMessageContains) {
        checks[`message contains "${expectedMessageContains}"`] = (r) => {
            const body = JSON.parse(r.body);
            return body.message && body.message.toLowerCase().includes(expectedMessageContains.toLowerCase());
        };
    }

    return check(response, checks);
}

/**
 * Sleep for a short duration
 */
export function shortSleep() {
    // k6 sleep is imported from 'k6' module
    // This is just a helper for consistent sleep duration
    return 0.1; // 100ms
}
