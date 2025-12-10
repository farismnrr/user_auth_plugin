/**
 * =============================================================================
 * ENDPOINT: GET /users/details
 * =============================================================================
 * 
 * Description: Get current user's details (from JWT)
 * 
 * URL: http://localhost:5500/users/details
 * Method: GET
 * 
 * Headers:
 *   - Authorization: Bearer <access_token>
 * 
 * Success Response (200):
 *   {
 *     "success": true,
 *     "message": "User details retrieved successfully",
 *     "data": {
 *       "id": "uuid",
 *       "user_id": "uuid",
 *       "full_name": "string",
 *       "phone_number": "string",
 *       "address": "string",
 *       "date_of_birth": "date",
 *       "profile_picture_url": "http://localhost:5500/assets/profiles/...",
 *       "created_at": "datetime",
 *       "updated_at": "datetime"
 *     }
 *   }
 * 
 * Error Responses:
 *   - 401 Unauthorized: Missing JWT, invalid JWT
 *   - 404 Not Found: User details not found
 * 
 * Notes:
 *   - Requires valid JWT Bearer token
 *   - Returns full URL for profile_picture_url
 *   - Does NOT require API key
 * 
 * Test Scenarios:
 *   1. Successful retrieval of user details
 *   2. Get details without JWT (401)
 *   3. Verify profile_picture_url is full URL
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
    const detailsUrl = `${BASE_URL}/users/details`;

    // Setup: Create a test user
    const testUser = {
        username: randomUsername(),
        email: randomEmail(),
        password: randomPassword(),
        role: 'user',
    };

    http.post(registerUrl, JSON.stringify(testUser), { headers });
    sleep(shortSleep());

    // Login to get JWT
    const loginPayload = {
        email_or_username: testUser.email,
        password: testUser.password,
    };

    const loginResponse = http.post(loginUrl, JSON.stringify(loginPayload), { headers });
    const accessToken = extractAccessToken(loginResponse);
    sleep(shortSleep());

    /**
    * Test Case: Successful retrieval of user details
    * URL: {apiUrl}/users/details
    * Auth: Bearer <valid_jwt>
    * Expected: {
    *   "success": true,
    *   "message": "User details retrieved successfully",
    *   "data": { ... }
    * }
    */
    console.log('Test 1: Successful retrieval of user details');
    let response = http.get(detailsUrl, {
        headers: {
            'Authorization': `Bearer ${accessToken}`,
        },
    });

    checkSuccess(response, 200, 'retrieved successfully');

    // Verify response structure
    const body = JSON.parse(response.body);
    if (body.success && body.data) {
        console.log(`User details ID: ${body.data.id}`);
        console.log(`User ID: ${body.data.user_id}`);
        console.log(`Profile picture URL: ${body.data.profile_picture_url}`);

        // Verify profile_picture_url is a full URL
        if (body.data.profile_picture_url) {
            const isFullUrl = body.data.profile_picture_url.startsWith('http://') ||
                body.data.profile_picture_url.startsWith('https://');
            console.log(`Profile picture URL is full URL: ${isFullUrl}`);

            if (!isFullUrl) {
                console.error(`ERROR: profile_picture_url should be full URL, got: ${body.data.profile_picture_url}`);
            }
        }
    }
    sleep(shortSleep());

    /**
    * Test Case: Get details without JWT
    * URL: {apiUrl}/users/details
    * Auth: None
    * Expected (401): {
    *   "success": false,
    *   "message": "Missing authentication token"
    * }
    */
    console.log('Test 2: Get details without JWT');
    response = http.get(detailsUrl);

    checkError(response, 401);
    sleep(shortSleep());
}
