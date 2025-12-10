/**
 * =============================================================================
 * ENDPOINT: DELETE /users
 * =============================================================================
 * 
 * Description: Delete current user (from JWT)
 * 
 * URL: http://localhost:5500/users
 * Method: DELETE
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
 *     "message": "User deleted successfully",
 *     "data": {
 *       "id": "uuid"
 *     }
 *   }
 * 
 * Error Responses:
 *   - 401 Unauthorized: Missing JWT, invalid JWT
 *   - 404 Not Found: User already deleted
 * 
 * Notes:
 *   - Requires valid JWT Bearer token
 *   - Deletes current user based on JWT user_id
 *   - Returns deleted user ID
 *   - User cannot login after deletion
 *   - Does NOT require API key
 * 
 * Test Scenarios:
 *   1. Successful deletion
 *   2. User cannot login after deletion
 *   3. Delete without JWT
 *   4. Delete with invalid JWT
 *   5. Delete already deleted user (simulated)
 * 
 * =============================================================================
 */

import http from 'k6/http';
import { sleep } from 'k6';
import { BASE_URL, options, headers } from '../config.js';
// ... imports
import {
    randomEmail,
    randomUsername,
    randomPassword,
    extractAccessToken,
    extractUserId,
    checkSuccess,
    checkError,
    shortSleep
} from '../utils.js';

export { options };

export default function () {
    const registerUrl = `${BASE_URL}/api/auth/register`;
    const loginUrl = `${BASE_URL}/api/auth/login`;
    const deleteUserUrl = `${BASE_URL}/users`;

    // Setup: Create a test user
    const testUser = {
        username: randomUsername(),
        email: randomEmail(),
        password: randomPassword(),
        role: 'user',
    };

    http.post(registerUrl, JSON.stringify(testUser), { headers });
    sleep(shortSleep());

    // Login
    const loginPayload = {
        email_or_username: testUser.email,
        password: testUser.password,
    };

    const loginResponse = http.post(loginUrl, JSON.stringify(loginPayload), { headers });
    const accessToken = extractAccessToken(loginResponse);
    sleep(shortSleep());

    const validHeaders = {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${accessToken}`,
    };

    /**
     * Test Case: Successful deletion
     * URL: {apiUrl}/users
     * Body: null
     * Auth: Bearer <valid_jwt>
     * Expected: {
     *   "success": true,
     *   "message": "User deleted successfully",
     *   "data": { "id": "uuid" }
     * }
     */
    console.log('Test 1: Successful deletion');
    let response = http.del(deleteUserUrl, null, { headers: validHeaders });
    checkSuccess(response, 200, 'deleted successfully');

    const deletedUserId = extractUserId(response);
    console.log(`Deleted user ID returned: ${deletedUserId ? 'Yes' : 'No'}`);
    sleep(shortSleep());

    /**
     * Test Case: User cannot login after deletion
     * URL: {apiUrl}/api/auth/login
     * Body: { email, password }
     * Auth: X-API-Key
     * Expected (401): {
     *   "success": false,
     *   "message": "User not found"
     * }
     */
    console.log('Test 2: User cannot login after deletion');
    response = http.post(loginUrl, JSON.stringify(loginPayload), { headers });
    checkError(response, 401); // User not found (treated as invalid credentials)
    sleep(shortSleep());

    /**
     * Test Case: Delete without JWT
     * URL: {apiUrl}/users
     * Body: null
     * Auth: None
     * Expected (401): {
     *   "success": false,
     *   "message": "Missing authentication token"
     * }
     */
    console.log('Test 3: Delete without JWT');
    // Create another user for this test
    const testUser2 = {
        username: randomUsername(),
        email: randomEmail(),
        password: randomPassword(),
        role: 'user',
    };

    http.post(registerUrl, JSON.stringify(testUser2), { headers });
    sleep(shortSleep());

    const noAuthHeaders = {
        'Content-Type': 'application/json',
    };

    response = http.del(deleteUserUrl, null, { headers: noAuthHeaders });
    response = http.del(deleteUserUrl, null, { headers: noAuthHeaders });
    checkError(response, 401);
    sleep(shortSleep());

    /**
     * Test Case: Delete with invalid JWT
     * URL: {apiUrl}/users
     * Body: null
     * Auth: Bearer invalid_token_here
     * Expected (401): {
     *   "success": false,
     *   "message": "Invalid token"
     * }
     */
    console.log('Test 4: Delete with invalid JWT');
    const invalidJwtHeaders = {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer invalid_token_here',
    };

    response = http.del(deleteUserUrl, null, { headers: invalidJwtHeaders });
    checkError(response, 401);
    sleep(shortSleep());

    /**
     * Test Case: Delete already deleted user
     * URL: {apiUrl}/users
     * Body: null
     * Auth: Bearer <old_token>
     * Expected (404): {
     *   "success": false,
     *   "message": "User not found"
     * }
     */
    console.log('Test 5: Delete already deleted user (using old token)');
    // Try to delete with the first user's token (already deleted)
    response = http.del(deleteUserUrl, null, { headers: validHeaders });
    checkError(response, 404); // User not found
    sleep(shortSleep());
}
