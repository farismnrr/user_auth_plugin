/**
 * =============================================================================
 * ENDPOINT: POST /auth/logout
 * =============================================================================
 * 
 * Description: Clear refresh token cookie (logout user)
 * 
 * URL: http://localhost:5500/auth/logout
 * Method: POST
 * 
 * Headers:
 *   - Content-Type: application/json
 *   - X-API-Key: <api-key>
 *   - Authorization: Bearer <jwt-token>
 * 
 * Request Body: None
 * 
 * Success Response (200):
 *   {
 *     "success": true,
 *     "message": "Logged out successfully",
 *     "data": null
 *   }
 * 
 * Cookies Set:
 *   - refresh_token: (cleared - max-age=0)
 * 
 * Error Responses:
 *   - 401 Unauthorized: Missing API key or invalid/missing JWT token
 * 
 * Notes:
 *   - Logout requires JWT authentication
 *   - User must be logged in to logout
 *   - Clears the refresh token cookie
 * 
 * Test Scenarios:
 *   1. Successful logout with valid JWT
 *   2. Logout without JWT token (should fail)
 *   3. Logout with invalid JWT token
 *   4. Missing API key
 * 
 * =============================================================================
 */

import http from 'k6/http';
import { sleep } from 'k6';
import { BASE_URL, headers, options } from '../config.js';
import {
    randomEmail,
    randomUsername,
    randomPassword,
    checkSuccess,
    checkError,
    shortSleep
} from '../utils.js';

export { options };

export default function () {
    const registerUrl = `${BASE_URL}/api/auth/register`;
    const loginUrl = `${BASE_URL}/api/auth/login`;
    const logoutUrl = `${BASE_URL}/auth/logout`;

    // Setup: Create and login a test user
    const testUser = {
        username: randomUsername(),
        email: randomEmail(),
        password: randomPassword(),
        role: 'user',
    };

    http.post(registerUrl, JSON.stringify(testUser), { headers });
    sleep(shortSleep());

    const loginPayload = {
        email_or_username: testUser.email,
        password: testUser.password,
    };

    const loginResponse = http.post(loginUrl, JSON.stringify(loginPayload), { headers });
    const loginData = JSON.parse(loginResponse.body);
    const accessToken = loginData.data.access_token;
    sleep(shortSleep());

    // Test 1: Successful logout with valid JWT
    console.log('Test 1: Successful logout with valid JWT');
    const authHeaders = {
        ...headers,
        'Authorization': `Bearer ${accessToken}`,
    };
    let response = http.post(logoutUrl, null, { headers: authHeaders });
    checkSuccess(response, 200, 'Logged out successfully');
    sleep(shortSleep());

    // Test 2: Logout without JWT token (should fail)
    console.log('Test 2: Logout without JWT token');
    response = http.post(logoutUrl, null, { headers });
    checkError(response, 401);
    sleep(shortSleep());

    // Test 3: Logout with invalid JWT token
    console.log('Test 3: Logout with invalid JWT token');
    const invalidAuthHeaders = {
        ...headers,
        'Authorization': 'Bearer invalid_token_here',
    };
    response = http.post(logoutUrl, null, { headers: invalidAuthHeaders });
    checkError(response, 401);
    sleep(shortSleep());

    // Test 4: Missing API key
    console.log('Test 4: Missing API key');
    const noApiKeyHeaders = {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${accessToken}`,
    };

    response = http.post(logoutUrl, null, { headers: noApiKeyHeaders });
    checkError(response, 401);
    sleep(shortSleep());
}
