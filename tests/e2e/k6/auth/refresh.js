/**
 * =============================================================================
 * ENDPOINT: POST /auth/refresh
 * =============================================================================
 * 
 * Description: Refresh access token using refresh token from cookie
 * 
 * URL: http://localhost:5500/auth/refresh
 * Method: POST
 * 
 * Headers:
 *   - Content-Type: application/json
 *   - Cookie: refresh_token=<token>
 * 
 * Request Body: None
 * 
 * Success Response (200):
 *   {
 *     "success": true,
 *     "message": "Token refreshed successfully",
 *     "data": {
 *       "access_token": "string"
 *     }
 *   }
 * 
 * Error Responses:
 *   - 401 Unauthorized: Invalid token, expired token, missing token
 * 
 * Notes:
 *   - Does NOT require API key
 *   - Requires valid refresh token in cookie
 *   - Returns new access token
 * 
 * Test Scenarios:
 *   1. Successful token refresh with valid refresh token
 *   2. Refresh with invalid token
 *   3. Refresh without token cookie
 *   4. Refresh with expired token (simulated with malformed token)
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
    extractAccessToken,
    extractRefreshToken,
    checkSuccess,
    checkError,
    shortSleep
} from '../utils.js';

export { options };

export default function () {
    const registerUrl = `${BASE_URL}/api/auth/register`;
    const loginUrl = `${BASE_URL}/api/auth/login`;
    const refreshUrl = `${BASE_URL}/api/auth/refresh`;

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
    checkSuccess(loginResponse, 200, 'Login successful');
    const refreshToken = extractRefreshToken(loginResponse);
    console.log(`Setup: Login status ${loginResponse.status}, Refresh token found: ${refreshToken ? 'Yes' : 'No'}`);
    sleep(shortSleep());

    /**
    * Test Case: Successful token refresh
    * URL: {apiUrl}/api/auth/refresh
    * Body: null
    * Auth: Cookie: refresh_token=<valid_token>
    * Expected: {
    *   "success": true,
    *   "message": "Token refreshed successfully",
    *   "data": { "access_token": "..." }
    * }
    */
    console.log('Test 1: Successful token refresh');
    const refreshHeaders = {
        ...headers,
        'Cookie': `refresh_token=${refreshToken}`,
    };

    let response = http.post(refreshUrl, null, { headers: refreshHeaders });
    checkSuccess(response, 200, 'Token refreshed successfully');

    const newAccessToken = extractAccessToken(response);
    console.log(`New access token received: ${newAccessToken ? 'Yes' : 'No'}`);
    sleep(shortSleep());

    /**
     * Test Case: Refresh with invalid token
     * URL: {apiUrl}/api/auth/refresh
     * Body: null
     * Auth: Cookie: refresh_token=invalid_token_here
     * Expected (401): {
     *   "success": false,
     *   "message": "Invalid refresh token"
     * }
     */
    console.log('Test 2: Refresh with invalid token');
    const invalidTokenHeaders = {
        ...headers,
        'Cookie': 'refresh_token=invalid_token_here',
    };

    response = http.post(refreshUrl, null, { headers: invalidTokenHeaders });
    checkError(response, 401);
    sleep(shortSleep());

    /**
     * Test Case: Refresh without token cookie
     * URL: {apiUrl}/api/auth/refresh
     * Body: null
     * Auth: None (Missing cookie)
     * Expected (401): {
     *   "success": false,
     *   "message": "Missing refresh token"
     * }
     */
    console.log('Test 3: Refresh without token cookie');
    // Clear cookies to ensure no refresh_token is sent
    const jar = http.cookieJar();
    jar.set(refreshUrl, 'refresh_token', 'deleted', { max_age: 0 }); // Attempt to expire it
    jar.clear(BASE_URL); // Clear by base URL

    const noTokenHeaders = {
        ...headers,
    };

    response = http.post(refreshUrl, null, { headers: noTokenHeaders });
    checkError(response, 401);
    sleep(shortSleep());

    /**
     * Test Case: Refresh with malformed/expired token
     * URL: {apiUrl}/api/auth/refresh
     * Body: null
     * Auth: Cookie: refresh_token=<expired_token>
     * Expected (401): {
     *   "success": false,
     *   "message": "Invalid refresh token"
     * }
     */
    console.log('Test 4: Refresh with malformed/expired token');
    const expiredTokenHeaders = {
        ...headers,
        'Cookie': 'refresh_token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.expired.token',
    };

    response = http.post(refreshUrl, null, { headers: expiredTokenHeaders });
    checkError(response, 401);
    sleep(shortSleep());
}
