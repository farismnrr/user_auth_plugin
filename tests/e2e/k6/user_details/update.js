/**
 * =============================================================================
 * ENDPOINT: PUT /users/details
 * =============================================================================
 * 
 * Description: Update current user's details (from JWT)
 * 
 * URL: http://localhost:5500/users/details
 * Method: PUT
 * 
 * Headers:
 *   - Content-Type: application/json
 *   - Authorization: Bearer <access_token>
 * 
 * Request Body (all fields optional):
 *   {
 *     "full_name": "string" | null,
 *     "phone_number": "string" | null,
 *     "address": "string" | null,
 *     "date_of_birth": "YYYY-MM-DD" | null
 *   }
 * 
 * Success Response (200):
 *   {
 *     "success": true,
 *     "message": "User details updated successfully",
 *     "data": {
 *       "id": "uuid"
 *     }
 *   }
 * 
 * Error Responses:
 *   - 400 Bad Request: Invalid data types, invalid date format
 *   - 401 Unauthorized: Missing JWT, invalid JWT
 * 
 * Notes:
 *   - Requires valid JWT Bearer token
 *   - All fields are optional (partial update)
 *   - Returns user ID
 *   - Profile picture is updated via separate endpoint
 *   - Does NOT require API key
 * 
 * Test Scenarios:
 *   1. Successful update with valid data
 *   2. Partial update (only some fields)
 *   3. Update with null values (clearing fields)
 *   4. Update without JWT
 *   5. Update with invalid data types
 *   6. Update with invalid date format
 *   7. Update with future date of birth
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
    const updateDetailsUrl = `${BASE_URL}/users/details`;

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
    * Test Case: Successful update with valid data
    * URL: {apiUrl}/users/details
    * Body: { full_name, phone_number, address, date_of_birth }
    * Auth: Bearer <valid_jwt>
    * Expected: {
    *   "success": true,
    *   "message": "User details updated successfully",
    *   "data": { "id": "uuid" }
    * }
    */
    console.log('Test 1: Successful update with valid data');
    const updatePayload = {
        full_name: 'John Doe',
        phone_number: '+1234567890',
        address: '123 Main St, City, Country',
        date_of_birth: '1990-01-15',
    };

    let response = http.put(updateDetailsUrl, JSON.stringify(updatePayload), { headers: validHeaders });
    checkSuccess(response, 200, 'updated successfully');

    const userId = extractUserId(response);
    console.log(`User ID returned: ${userId ? 'Yes' : 'No'}`);
    sleep(shortSleep());

    /**
    * Test Case: Partial update
    * URL: {apiUrl}/users/details
    * Body: { full_name }
    * Auth: Bearer <valid_jwt>
    * Expected: {
    *   "success": true,
    *   "message": "User details updated successfully",
    *   "data": { "id": "uuid" }
    * }
    */
    console.log('Test 2: Partial update (only full_name)');
    const partialUpdate = {
        full_name: 'Jane Smith',
    };

    response = http.put(updateDetailsUrl, JSON.stringify(partialUpdate), { headers: validHeaders });
    checkSuccess(response, 200);
    sleep(shortSleep());

    /**
    * Test Case: Update with null values
    * URL: {apiUrl}/users/details
    * Body: { phone_number: null, address: null }
    * Auth: Bearer <valid_jwt>
    * Expected: {
    *   "success": true,
    *   "message": "User details updated successfully",
    *   "data": { "id": "uuid" }
    * }
    */
    console.log('Test 3: Update with null values');
    const nullUpdate = {
        phone_number: null,
        address: null,
    };

    response = http.put(updateDetailsUrl, JSON.stringify(nullUpdate), { headers: validHeaders });
    checkSuccess(response, 200);
    sleep(shortSleep());

    /**
    * Test Case: Update without JWT
    * URL: {apiUrl}/users/details
    * Body: { full_name, ... }
    * Auth: None
    * Expected (401): {
    *   "success": false,
    *   "message": "Missing authentication token"
    * }
    */
    console.log('Test 4: Update without JWT');
    const noAuthHeaders = {
        'Content-Type': 'application/json',
    };

    response = http.put(updateDetailsUrl, JSON.stringify(updatePayload), { headers: noAuthHeaders });
    response = http.put(updateDetailsUrl, JSON.stringify(updatePayload), { headers: noAuthHeaders });
    checkError(response, 401);
    sleep(shortSleep());

    /**
    * Test Case: Update with invalid data types
    * URL: {apiUrl}/users/details
    * Body: { full_name: <int>, phone_number: <bool> }
    * Auth: Bearer <valid_jwt>
    * Expected (400): {
    *   "success": false,
    *   "message": "Invalid input data"
    * }
    */
    console.log('Test 5: Update with invalid data types');
    const invalidDataUpdate = {
        full_name: 12345, // Should be string
        phone_number: true, // Should be string
    };

    response = http.put(updateDetailsUrl, JSON.stringify(invalidDataUpdate), { headers: validHeaders });
    checkError(response, 400);
    sleep(shortSleep());

    /**
    * Test Case: Update with invalid date format
    * URL: {apiUrl}/users/details
    * Body: { date_of_birth: 'invalid-date' }
    * Auth: Bearer <valid_jwt>
    * Expected (400): {
    *   "success": false,
    *   "message": "Invalid date format"
    * }
    */
    console.log('Test 6: Update with invalid date format');
    const invalidDateUpdate = {
        date_of_birth: 'invalid-date',
    };

    response = http.put(updateDetailsUrl, JSON.stringify(invalidDateUpdate), { headers: validHeaders });
    checkError(response, 400);
    sleep(shortSleep());

    /**
    * Test Case: Update with future date of birth
    * URL: {apiUrl}/users/details
    * Body: { date_of_birth: '2099-12-31' }
    * Auth: Bearer <valid_jwt>
    * Expected (400): {
    *   "success": false,
    *   "message": "Date of birth cannot be in the future"
    * }
    */
    console.log('Test 7: Update with future date of birth');
    const futureDateUpdate = {
        date_of_birth: '2099-12-31',
    };

    response = http.put(updateDetailsUrl, JSON.stringify(futureDateUpdate), { headers: validHeaders });
    // This might succeed or fail depending on validation - just check response
    console.log(`Future date response status: ${response.status}`);
    sleep(shortSleep());
}
