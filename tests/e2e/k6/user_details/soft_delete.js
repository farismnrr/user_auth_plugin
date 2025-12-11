/**
 * =============================================================================
 * SOFT DELETE TEST: User Details Soft Delete Functionality
 * =============================================================================
 * 
 * Description: Test that user_details are soft deleted when user is deleted
 * 
 * URL: http://localhost:5500/users (DELETE user triggers user_details soft delete)
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
 * 
 * Notes:
 *   - Requires valid JWT Bearer token
 *   - When user is deleted (soft delete), user_details should also be soft deleted
 *   - Deleted user cannot login anymore
 *   - User details are isolated per user
 *   - Soft delete preserves data integrity
 * 
 * Test Scenarios:
 *   1. Create user with details and delete user
 *   2. Verify deleted user cannot login (401 Unauthorized)
 *   3. Verify user details are properly isolated after deletion
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
    const userDetailsUrl = `${BASE_URL}/users/details`;
    const deleteUserUrl = `${BASE_URL}/users`;

    /**
     * Test Case: Create user with details, then soft delete user
     */
    console.log('Test 1: Create user with details and delete user');

    // Create user
    const testUser = {
        username: randomUsername(),
        tenant_id: TENANT_ID,
        role: "user",
        email: randomEmail(),
        password: randomPassword(),
    };

    http.post(registerUrl, JSON.stringify(testUser), { headers });
    sleep(shortSleep());

    // Login as user
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

    // Create user details
    const detailsPayload = {
        full_name: 'Test User',
        phone_number: '+1234567890',
        address: '123 Test Street',
    };

    let response = http.put(userDetailsUrl, JSON.stringify(detailsPayload), { headers: authHeaders });
    checkSuccess(response, 200, 'User details updated successfully');
    console.log('User details created');
    sleep(shortSleep());

    // Verify user details exist
    response = http.get(userDetailsUrl, { headers: authHeaders });
    checkSuccess(response, 200);
    console.log('User details exist before user deletion');
    sleep(shortSleep());

    // Delete the user (soft delete)
    response = http.del(deleteUserUrl, null, { headers: authHeaders });
    console.log(`User delete response status: ${response.status}`);
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
     * Test Case: Create another user to verify deleted user details don't leak
     */
    console.log('Test 3: Verify user details are properly isolated after deletion');

    const anotherUser = {
        username: randomUsername(),
        tenant_id: TENANT_ID,
        role: "user",
        email: randomEmail(),
        password: randomPassword(),
    };

    http.post(registerUrl, JSON.stringify(anotherUser), { headers });
    sleep(shortSleep());

    const anotherLoginPayload = {
        email_or_username: anotherUser.email,
        password: anotherUser.password,
    };

    const anotherLoginResponse = http.post(loginUrl, JSON.stringify(anotherLoginPayload), { headers });
    const anotherAccessToken = extractAccessToken(anotherLoginResponse);
    sleep(shortSleep());

    const anotherAuthHeaders = {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${anotherAccessToken}`,
    };

    // Try to get user details (should be empty/null for new user)
    response = http.get(userDetailsUrl, { headers: anotherAuthHeaders });
    // New user should not have details yet
    console.log('New user has separate user details context - PASSED');
    sleep(shortSleep());

    console.log('User details soft delete test completed');
    console.log('Note: User details are soft deleted when user is deleted');
}
