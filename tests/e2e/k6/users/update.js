/**
 * =============================================================================
 * ENDPOINT: PUT /users
 * =============================================================================
 * 
 * Description: Update current user information (from JWT)
 * 
 * URL: http://localhost:5500/users
 * Method: PUT
 * 
 * Headers:
 *   - Content-Type: application/json
 *   - Authorization: Bearer <access_token>
 * 
 * Request Body (all fields optional):
 *   {
 *     "username": "string" | null,
 *     "email": "string" | null,
 *     "password": "string" | null,
 *     "role": "string" | null
 *   }
 * 
 * Success Response (200):
 *   {
 *     "success": true,
 *     "message": "User updated successfully",
 *     "data": {
 *       "id": "uuid"
 *     }
 *   }
 * 
 * Error Responses:
 *   - 400 Bad Request: Invalid email format, invalid data types
 *   - 401 Unauthorized: Missing JWT, invalid JWT
 *   - 409 Conflict: Duplicate email or username
 * 
 * Notes:
 *   - Requires valid JWT Bearer token
 *   - All fields are optional (partial update)
 *   - Returns only user ID
 *   - Does NOT require API key
 * 
 * Test Scenarios:
 *   1. Successful update with valid data
 *   2. Partial update (only some fields)
 *   3. Update with duplicate email
 *   4. Update with duplicate username
 *   5. Update without JWT
 *   6. Update with invalid email format
 *   7. Update with invalid data types
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
    const updateUserUrl = `${BASE_URL}/users`;

    // Setup: Create two test users
    const testUser1 = {
        username: randomUsername(),
        email: randomEmail(),
        password: randomPassword(),
        role: 'user',
    };

    const testUser2 = {
        username: randomUsername(),
        email: randomEmail(),
        password: randomPassword(),
        role: 'user',
    };

    http.post(registerUrl, JSON.stringify(testUser1), { headers });
    sleep(shortSleep());
    http.post(registerUrl, JSON.stringify(testUser2), { headers });
    sleep(shortSleep());

    // Login with first user
    const loginPayload = {
        email_or_username: testUser1.email,
        password: testUser1.password,
    };

    const loginResponse = http.post(loginUrl, JSON.stringify(loginPayload), { headers });
    const accessToken = extractAccessToken(loginResponse);
    sleep(shortSleep());

    const validHeaders = {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${accessToken}`,
    };

    // Test 1: Successful update with valid data
    console.log('Test 1: Successful update with valid data');
    const updatePayload = {
        username: randomUsername(),
        email: randomEmail(),
    };

    let response = http.put(updateUserUrl, JSON.stringify(updatePayload), { headers: validHeaders });
    checkSuccess(response, 200, 'updated successfully');

    const userId = extractUserId(response);
    console.log(`User ID returned: ${userId ? 'Yes' : 'No'}`);
    sleep(shortSleep());

    // Test 2: Partial update (only some fields)
    console.log('Test 2: Partial update (only username)');
    const partialUpdate = {
        username: randomUsername(),
    };

    response = http.put(updateUserUrl, JSON.stringify(partialUpdate), { headers: validHeaders });
    checkSuccess(response, 200);
    sleep(shortSleep());

    // Test 3: Update with duplicate email
    console.log('Test 3: Update with duplicate email');
    const duplicateEmailUpdate = {
        email: testUser2.email, // Email from second user
    };

    response = http.put(updateUserUrl, JSON.stringify(duplicateEmailUpdate), { headers: validHeaders });
    checkError(response, 409, 'email');
    sleep(shortSleep());

    // Test 4: Update with duplicate username
    console.log('Test 4: Update with duplicate username');
    const duplicateUsernameUpdate = {
        username: testUser2.username, // Username from second user
    };

    response = http.put(updateUserUrl, JSON.stringify(duplicateUsernameUpdate), { headers: validHeaders });
    checkError(response, 409, 'username');
    sleep(shortSleep());

    // Test 5: Update without JWT
    console.log('Test 5: Update without JWT');
    const noAuthHeaders = {
        'Content-Type': 'application/json',
    };

    response = http.put(updateUserUrl, JSON.stringify(updatePayload), { headers: noAuthHeaders });
    checkError(response, 401);
    sleep(shortSleep());

    // Test 6: Update with invalid email format
    console.log('Test 6: Update with invalid email format');
    const invalidEmailUpdate = {
        email: 'invalid-email-format',
    };

    response = http.put(updateUserUrl, JSON.stringify(invalidEmailUpdate), { headers: validHeaders });
    checkError(response, 400);
    sleep(shortSleep());

    // Test 7: Update with invalid data types
    console.log('Test 7: Update with invalid data types');
    const invalidDataUpdate = {
        username: 12345, // Should be string
    };

    response = http.put(updateUserUrl, JSON.stringify(invalidDataUpdate), { headers: validHeaders });
    checkError(response, 400);
    sleep(shortSleep());
}
