/**
 * K6 Configuration File
 * 
 * Shared configuration for all k6 E2E tests
 */

// Base URL for the API
export const BASE_URL = __ENV.BASE_URL || 'http://localhost:5500';

// API Key for authentication (required for auth endpoints)
export const API_KEY = __ENV.API_KEY || 'your-api-key-for-endpoint-protection';

// Default test options
export const options = {
    vus: 1,
    iterations: 1,
    thresholds: {
        http_req_duration: ['p(95)<2000'], // 95% of requests should be below 2s
    },
};

// Common HTTP headers
export const headers = {
    'Content-Type': 'application/json',
    'X-API-Key': API_KEY,
};

// JWT Bearer header (to be used with token)
export function authHeader(token) {
    return {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}`,
    };
}
