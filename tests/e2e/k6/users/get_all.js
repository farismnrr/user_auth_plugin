/**
 * =============================================================================
 * ENDPOINT: GET /users/all
 * =============================================================================
 * 
 * Description: Get all users in the system
 * 
 * URL: http://localhost:5500/users/all
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
 *     "message": "Users retrieved successfully",
 *     "data": [
 *       {
 *         "id": "uuid",
 *         "username": "string",
 *         "email": "string",
 *         "role": "string",
 *         "created_at": "datetime",
 *         "updated_at": "datetime",
 *         "details": null | object
 *       },
 *       ...
 *     ]
 *   }
 * 
 * Error Responses:
 *   - 401 Unauthorized: Missing JWT, invalid JWT
 * 
 * Notes:
 *   - Requires valid JWT Bearer token
 *   - Returns array of all users
 *   - Does NOT require API key
 * 
 * Test Scenarios:
 *   1. Get all users with valid JWT
 *   2. Get all without JWT
 *   3. Get all with invalid JWT
 * 
 * =============================================================================
 */

import http from 'k6/http';
import { sleep } from 'k6';
import { check } from 'k6';
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
    const getAllUsersUrl = `${BASE_URL}/users/all`;

    // Setup: Create multiple test users
    const users = [];
    for (let i = 0; i < 3; i++) {
        const user = {
            username: randomUsername(),
            email: randomEmail(),
            password: randomPassword(),
            role: 'user',
        };
        users.push(user);
        http.post(registerUrl, JSON.stringify(user), { headers });
        sleep(shortSleep());
    }

    // Login with first user
    const loginPayload = {
        email_or_username: users[0].email,
        password: users[0].password,
    };

    const loginResponse = http.post(loginUrl, JSON.stringify(loginPayload), { headers });
    const accessToken = extractAccessToken(loginResponse);
    sleep(shortSleep());

    // Test 1: Get all users with valid JWT
    console.log('Test 1: Get all users with valid JWT');
    const validHeaders = {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${accessToken}`,
    };

    let response = http.get(getAllUsersUrl, { headers: validHeaders });
    checkSuccess(response, 200);

    // Verify response is an array
    check(response, {
        'data is array': (r) => {
            const body = JSON.parse(r.body);
            return Array.isArray(body.data);
        },
        'array has users': (r) => {
            const body = JSON.parse(r.body);
            return body.data.length >= 3; // At least our 3 test users
        },
    });

    const body = JSON.parse(response.body);
    console.log(`Total users returned: ${body.data.length}`);
    sleep(shortSleep());

    // Test 2: Get all without JWT
    console.log('Test 2: Get all without JWT');
    const noAuthHeaders = {
        'Content-Type': 'application/json',
    };

    response = http.get(getAllUsersUrl, { headers: noAuthHeaders });
    checkError(response, 401);
    sleep(shortSleep());

    // Test 3: Get all with invalid JWT
    console.log('Test 3: Get all with invalid JWT');
    const invalidJwtHeaders = {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer invalid_token_here',
    };

    response = http.get(getAllUsersUrl, { headers: invalidJwtHeaders });
    checkError(response, 401);
    sleep(shortSleep());
}
