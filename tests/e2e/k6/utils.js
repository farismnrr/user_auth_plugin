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
    // Fallback: Check Set-Cookie header
    const setCookie = response.headers['Set-Cookie'];
    if (setCookie) {
        // Handle array or single string
        const cookiesList = Array.isArray(setCookie) ? setCookie : [setCookie];
        for (const c of cookiesList) {
            if (c.includes('refresh_token=')) {
                const match = c.match(/refresh_token=([^;]+)/);
                if (match) return match[1];
            }
        }
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

    let body = null;
    try {
        body = JSON.parse(response.body);
    } catch (e) {
        // Body might not be JSON (e.g. 404 HTML)
    }

    if (expectedStatus >= 200 && expectedStatus < 300) {
        checks['success is true'] = (r) => {
            return body && body.success === true;
        };
    }

    if (checkMessage) {
        checks[`message contains "${checkMessage}"`] = (r) => {
            return body && body.message && body.message.includes(checkMessage);
        };
    }

    const res = check(response, checks);
    if (!res) {
        console.log(`[FAILED] ${response.request.method} ${response.request.url}`);
        console.log(`  Expected Status: ${expectedStatus}, Got: ${response.status}`);
        if (checkMessage) {
            console.log(`  Expected Message to contain: "${checkMessage}"`);
        }
        console.log(`  Got Body: ${response.body}`);
    }
    return res;
}

/**
 * Check if response is error
 */
export function checkError(response, expectedStatus, expectedMessageContains = null) {
    const checks = {
        [`status is ${expectedStatus}`]: (r) => r.status === expectedStatus,
    };

    if (expectedStatus === 401) {
        // 401 from middleware might not have a body, which is acceptable
        checks['response has body'] = (r) => (r.body && r.body.length > 0) || r.status === 401;
    } else {
        checks['response has body'] = (r) => r.body && r.body.length > 0;
    }

    let body = null;
    try {
        body = JSON.parse(response.body);
    } catch (e) {
        // Body might not be JSON
    }

    if (expectedMessageContains && body) {
        checks[`message contains "${expectedMessageContains}"`] = (r) => {
            return body && body.message && body.message.toLowerCase().includes(expectedMessageContains.toLowerCase());
        };
    } else if (expectedMessageContains && !body) {
        // If we expect a message but have no body, this specific check will fail if status isn't 401 (or if we strictly require a message)
        // For 401 empty body, we skip message check if body is missing
        if (expectedStatus === 401 && (!response.body || response.body.length === 0)) {
            // Skip message check for empty 401
        } else {
            checks[`message contains "${expectedMessageContains}"`] = () => false;
        }
    }

    const res = check(response, checks);
    if (!res) {
        console.log(`[FAILED] ${response.request.method} ${response.request.url}`);
        console.log(`  Expected Status: ${expectedStatus}, Got: ${response.status}`);
        if (expectedMessageContains) {
            console.log(`  Expected Message to contain: "${expectedMessageContains}"`);
        }
        console.log(`  Got Body: ${response.body}`);
    }
    return res;
}

/**
 * Sleep for a short duration
 */
export function shortSleep() {
    // k6 sleep is imported from 'k6' module
    // This is just a helper for consistent sleep duration
    return 0.1; // 100ms
}
