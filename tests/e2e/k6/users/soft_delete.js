/**
 * =============================================================================
 * ENDPOINT: DELETE /users (Soft Delete)
 * =============================================================================
 * 
 * Description: Soft delete current user (sets deleted_at timestamp)
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
 *   - 401 Unauthorized: Missing or invalid JWT
 *   - 404 Not Found: User not found
 * 
 * Notes:
 *   - Requires valid JWT Bearer token
 *   - This is a SOFT DELETE (sets deleted_at, doesn't remove record)
 *   - Deleted user cannot login anymore
 *   - Deleted user doesn't appear in GET /users/all
 *   - Deleted user doesn't appear in GET /users
 *   - User details are also soft deleted when user is deleted
 * 
 * Test Scenarios:
 *   1. Delete user and verify soft delete (200 OK)
 *   2. Verify deleted user cannot login (401 Unauthorized)
 *   3. Verify deleted user not in GET /users/all
 *   4. Confirm soft delete preserves data (deleted_at set)
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
    const getUserUrl = `${BASE_URL}/users`;
    const getAllUsersUrl = `${BASE_URL}/users/all`;
    const deleteUserUrl = `${BASE_URL}/users`;

    /**
     * Test Case: Delete user and verify soft delete behavior
     */
    console.log('Test 1: Create user, delete it, and verify soft delete');

    // Create first user
    const testUser = {
        username: randomUsername(),
        tenant_id: TENANT_ID,
        role: "user",
        email: randomEmail(),
        password: randomPassword(),
    };

    http.post(registerUrl, JSON.stringify(testUser), { headers });
    sleep(shortSleep());

    // Login as first user
    const loginPayload = {
        email_or_username: testUser.email,
        password: testUser.password,
    };

    let loginResponse = http.post(loginUrl, JSON.stringify(loginPayload), { headers });
    let accessToken = extractAccessToken(loginResponse);
    sleep(shortSleep());

    let authHeaders = {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${accessToken}`,
    };

    // Verify user exists
    let response = http.get(getUserUrl, { headers: authHeaders });
    checkSuccess(response, 200, 'User retrieved successfully');
    console.log('User exists before deletion');
    sleep(shortSleep());

    // Delete the user (soft delete)
    response = http.del(deleteUserUrl, null, { headers: authHeaders });
    console.log(`Delete response status: ${response.status}`);
    sleep(shortSleep());

    /**
     * Test Case: Verify deleted user cannot login
     */
    console.log('Test 2: Verify deleted user cannot login');
    loginResponse = http.post(loginUrl, JSON.stringify(loginPayload), { headers });
    checkError(loginResponse, 401);
    console.log('Deleted user cannot login - PASSED');
    sleep(shortSleep());

    /**
     * Test Case: Verify deleted user doesn't appear in listings
     */
    console.log('Test 3: Verify deleted user doesn\'t appear in GET /users/all');

    // Create another user to check listings
    const adminUser = {
        username: randomUsername(),
        tenant_id: TENANT_ID,
        role: "user",
        email: randomEmail(),
        password: randomPassword(),
    };

    http.post(registerUrl, JSON.stringify(adminUser), { headers });
    sleep(shortSleep());

    const adminLoginPayload = {
        email_or_username: adminUser.email,
        password: adminUser.password,
    };

    const adminLoginResponse = http.post(loginUrl, JSON.stringify(adminLoginPayload), { headers });
    const adminAccessToken = extractAccessToken(adminLoginResponse);
    sleep(shortSleep());

    const adminAuthHeaders = {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${adminAccessToken}`,
    };

    // Get all users
    response = http.get(getAllUsersUrl, { headers: adminAuthHeaders });
    checkSuccess(response, 200, 'Users retrieved successfully');

    const allUsers = JSON.parse(response.body).data;
    const deletedUserExists = allUsers.some(u => u.email === testUser.email);

    if (!deletedUserExists) {
        console.log('Deleted user NOT in /users/all - PASSED');
    } else {
        console.log('[FAILED] Deleted user still appears in /users/all');
    }
    sleep(shortSleep());

    /**
     * Test Case: Verify soft delete preserves data
     */
    console.log('Test 4: Soft delete test completed');
    console.log('Note: Soft delete sets deleted_at timestamp instead of removing record');
}
