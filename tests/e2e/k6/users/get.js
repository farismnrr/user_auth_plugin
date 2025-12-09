/**
 * =============================================================================
 * ENDPOINT: GET /users
 * =============================================================================
 * 
 * Description: Get current user information (from JWT)
 * 
 * URL: http://localhost:5500/users
 * Method: GET
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
 *     "message": "User retrieved successfully",
 *     "data": {
 *       "id": "uuid",
 *       "username": "string",
 *       "email": "string",
 *       "role": "string",
 *       "created_at": "datetime",
 *       "updated_at": "datetime",
 *       "details": null | {
 *         "id": "uuid",
 *         "user_id": "uuid",
 *         "full_name": "string" | null,
 *         "phone_number": "string" | null,
 *         "address": "string" | null,
 *         "date_of_birth": "date" | null,
 *         "profile_picture_url": "string" | null,
 *         "created_at": "datetime",
 *         "updated_at": "datetime"
 *       }
 *     }
 *   }
 * 
 * Error Responses:
 *   - 401 Unauthorized: Missing JWT, invalid JWT, expired JWT
 * 
 * Notes:
 *   - Requires valid JWT Bearer token
 *   - Returns current user based on JWT user_id
 *   - Does NOT require API key
 * 
 * Test Scenarios:
 *   1. Get current user with valid JWT
 *   2. Get without JWT
 *   3. Get with invalid JWT
 *   4. Get with expired/malformed JWT
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
    checkSuccess,
    checkError,
    shortSleep
} from '../utils.js';

export { options };

export default function () {
    const registerUrl = `${BASE_URL}/api/auth/register`;
    const loginUrl = `${BASE_URL}/api/auth/login`;
    const getUserUrl = `${BASE_URL}/users`;

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
    const accessToken = extractAccessToken(loginResponse);
    sleep(shortSleep());

    // Test 1: Get current user with valid JWT
    console.log('Test 1: Get current user with valid JWT');
    const validHeaders = {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${accessToken}`,
    };

    let response = http.get(getUserUrl, { headers: validHeaders });
    checkSuccess(response, 200);

    // Verify user data is correct
    const body = JSON.parse(response.body);
    console.log(`User email matches: ${body.data.email === testUser.email ? 'Yes' : 'No'}`);
    console.log(`User username matches: ${body.data.username === testUser.username ? 'Yes' : 'No'}`);
    sleep(shortSleep());

    // Test 2: Get without JWT
    console.log('Test 2: Get without JWT');
    const noAuthHeaders = {
        'Content-Type': 'application/json',
    };

    response = http.get(getUserUrl, { headers: noAuthHeaders });
    checkError(response, 401);
    sleep(shortSleep());

    // Test 3: Get with invalid JWT
    console.log('Test 3: Get with invalid JWT');
    const invalidJwtHeaders = {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer invalid_token_here',
    };

    response = http.get(getUserUrl, { headers: invalidJwtHeaders });
    checkError(response, 401);
    sleep(shortSleep());

    // Test 4: Get with expired/malformed JWT
    console.log('Test 4: Get with malformed JWT');
    const malformedJwtHeaders = {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.malformed.signature',
    };

    response = http.get(getUserUrl, { headers: malformedJwtHeaders });
    checkError(response, 401);
    sleep(shortSleep());
}
