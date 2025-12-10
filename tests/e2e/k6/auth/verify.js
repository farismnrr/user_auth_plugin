/**
 * =============================================================================
 * ENDPOINT: POST /auth/verify
 * =============================================================================
 * 
 * Description: Verify JWT token and return user data
 * 
 * URL: http://localhost:5500/auth/verify
 * Method: POST
 * 
 * Headers:
 *   - Content-Type: application/json
 *   - Authorization: Bearer <access_token>
 * 
 * Request Body: None
 * 
 * Success Response (200):
 *   {
 *     "success": true,
 *     "message": "Token is valid",
 *     "data": {
 *       "id": "uuid",
 *       "username": "string",
 *       "email": "string",
 *       "role": "string",
 *       "created_at": "datetime",
 *       "updated_at": "datetime",
 *       "details": null | object
 *     }
 *   }
 * 
 * Error Responses:
 *   - 401 Unauthorized: Invalid JWT, expired JWT, missing Authorization header
 *   - 404 Not Found: User deleted but token still valid
 * 
 * Notes:
 *   - Requires valid JWT Bearer token
 *   - Does NOT require API key
 *   - Validates token and checks if user still exists
 * 
 * Test Scenarios:
 *   1. Successful verification with valid JWT
 *   2. Invalid JWT format
 *   3. Expired JWT (simulated with malformed token)
 *   4. Missing Authorization header
 *   5. Malformed Authorization header (no Bearer)
 *   6. User deleted but token still valid
 * 
 * =============================================================================
 */

import http from 'k6/http';
import { sleep } from 'k6';
import { BASE_URL, options, headers } from '../config.js';
import {
    randomEmail,
    randomUsername,
    randomPassword,
    extractAccessToken,
    checkSuccess,
    checkError,
    shortSleep
} from '../utils.js';

export { options };

export default function () {
    const registerUrl = `${BASE_URL}/api/auth/register`;
    const loginUrl = `${BASE_URL}/api/auth/login`;
    const verifyUrl = `${BASE_URL}/auth/verify`;
    const deleteUrl = `${BASE_URL}/users`;

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
    const accessToken = extractAccessToken(loginResponse);
    sleep(shortSleep());

    /**
     * Test Case: Successful verification with valid JWT
     * URL: {apiUrl}/auth/verify
     * Body: null
     * Auth: Bearer <valid_jwt>
     * Expected: {
     *   "success": true,
     *   "message": "Token is valid",
     *   "data": { "id": "...", "username": "...", ... }
     * }
     */
    console.log('Test 1: Successful verification with valid JWT');
    const validHeaders = {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${accessToken}`,
    };

    let response = http.post(verifyUrl, null, { headers: validHeaders });
    checkSuccess(response, 200, 'Token is valid');
    sleep(shortSleep());

    /**
     * Test Case: Invalid JWT format
     * URL: {apiUrl}/auth/verify
     * Body: null
     * Auth: Bearer invalid_token_format
     * Expected (401): {
     *   "success": false,
     *   "message": "Invalid authentication token"
     * }
     */
    console.log('Test 2: Invalid JWT format');
    const invalidJwtHeaders = {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer invalid_token_format',
    };

    response = http.post(verifyUrl, null, { headers: invalidJwtHeaders });
    checkError(response, 401);
    sleep(shortSleep());

    /**
     * Test Case: Malformed/Expired JWT
     * URL: {apiUrl}/auth/verify
     * Body: null
     * Auth: Bearer <expired_token>
     * Expected (401): {
     *   "success": false,
     *   "message": "Invalid authentication token"
     * }
     */
    console.log('Test 3: Malformed/Expired JWT');
    const expiredJwtHeaders = {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.expired.signature',
    };

    response = http.post(verifyUrl, null, { headers: expiredJwtHeaders });
    checkError(response, 401);
    sleep(shortSleep());

    /**
     * Test Case: Missing Authorization header
     * URL: {apiUrl}/auth/verify
     * Body: null
     * Auth: None
     * Expected (401): {
     *   "success": false,
     *   "message": "Missing authentication token"
     * }
     */
    console.log('Test 4: Missing Authorization header');
    const noAuthHeaders = {
        'Content-Type': 'application/json',
    };

    response = http.post(verifyUrl, null, { headers: noAuthHeaders });
    checkError(response, 401); // Framework returns 401 with empty body here
    sleep(shortSleep());

    /**
     * Test Case: Malformed Authorization header
     * URL: {apiUrl}/auth/verify
     * Body: null
     * Auth: <jwt_only> (Missing Bearer prefix)
     * Expected (401): {
     *   "success": false,
     *   "message": "Invalid authentication token"
     * }
     */
    console.log('Test 5: Malformed Authorization header');
    const malformedAuthHeaders = {
        'Content-Type': 'application/json',
        'Authorization': accessToken, // Missing "Bearer " prefix
    };

    response = http.post(verifyUrl, null, { headers: malformedAuthHeaders });
    checkError(response, 401); // Framework returns 401 with empty body here
    sleep(shortSleep());

    /**
     * Test Case: User deleted but token still valid
     * URL: {apiUrl}/auth/verify
     * Body: null
     * Auth: Bearer <valid_jwt> (User Deleted)
     * Expected (404): {
     *   "success": false,
     *   "message": "User not found"
     * }
     */
    console.log('Test 6: User deleted but token still valid');
    // First delete the user
    http.del(deleteUrl, null, { headers: validHeaders });
    sleep(shortSleep());

    // Then try to verify with the same token
    response = http.post(verifyUrl, null, { headers: validHeaders });
    checkError(response, 404);
    sleep(shortSleep());
}
