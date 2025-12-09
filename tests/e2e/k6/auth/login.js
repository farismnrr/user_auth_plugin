/**
 * =============================================================================
 * ENDPOINT: POST /auth/login
 * =============================================================================
 * 
 * Description: Authenticate user and receive access token
 * 
 * URL: http://localhost:5500/auth/login
 * Method: POST
 * 
 * Headers:
 *   - Content-Type: application/json
 *   - X-API-Key: <api-key>
 * 
 * Request Body:
 *   {
 *     "email_or_username": "string",  // Can be email or username
 *     "password": "string"
 *   }
 * 
 * Success Response (200):
 *   {
 *     "success": true,
 *     "message": "Login successful",
 *     "data": {
 *       "user": {
 *         "id": "uuid",
 *         "username": "string",
 *         "email": "string",
 *         "role": "string",
 *         "created_at": "datetime",
 *         "updated_at": "datetime",
 *         "details": null | object
 *       },
 *       "access_token": "string"
 *     }
 *   }
 * 
 * Cookies Set:
 *   - refresh_token: HttpOnly, Secure, SameSite=Strict
 * 
 * Error Responses:
 *   - 400 Bad Request: Missing credentials, invalid format
 *   - 401 Unauthorized: Wrong password, missing API key
 *   - 404 Not Found: User not found
 * 
 * Test Scenarios:
 *   1. Successful login with email
 *   2. Successful login with username
 *   3. Wrong password
 *   4. Non-existent user
 *   5. Missing credentials
 *   6. Invalid email format (when using email)
 *   7. Missing API key
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

    // Setup: Create a test user first
    const testUser = {
        username: randomUsername(),
        email: randomEmail(),
        password: randomPassword(),
        role: 'user',
    };

    http.post(registerUrl, JSON.stringify(testUser), { headers });
    sleep(shortSleep());

    // Test 1: Successful login with email
    console.log('Test 1: Successful login with email');
    const loginWithEmail = {
        email_or_username: testUser.email,
        password: testUser.password,
    };

    let response = http.post(loginUrl, JSON.stringify(loginWithEmail), { headers });
    checkSuccess(response, 200, 'Login successful');

    const accessToken = extractAccessToken(response);
    const refreshToken = extractRefreshToken(response);
    console.log(`Access token received: ${accessToken ? 'Yes' : 'No'}`);
    console.log(`Refresh token cookie set: ${refreshToken ? 'Yes' : 'No'}`);
    sleep(shortSleep());

    // Test 2: Successful login with username
    console.log('Test 2: Successful login with username');
    const loginWithUsername = {
        email_or_username: testUser.username,
        password: testUser.password,
    };

    response = http.post(loginUrl, JSON.stringify(loginWithUsername), { headers });
    checkSuccess(response, 200, 'Login successful');
    sleep(shortSleep());

    // Test 3: Wrong password
    console.log('Test 3: Wrong password');
    const wrongPassword = {
        email_or_username: testUser.email,
        password: 'WrongPassword123!',
    };

    response = http.post(loginUrl, JSON.stringify(wrongPassword), { headers });
    checkError(response, 401);
    sleep(shortSleep());

    // Test 4: Non-existent user
    console.log('Test 4: Non-existent user');
    const nonExistentUser = {
        email_or_username: 'nonexistent@example.com',
        password: randomPassword(),
    };

    response = http.post(loginUrl, JSON.stringify(nonExistentUser), { headers });
    checkError(response, 404);
    sleep(shortSleep());

    // Test 5: Missing credentials
    console.log('Test 5: Missing credentials (no password)');
    const missingPassword = {
        email_or_username: testUser.email,
    };

    response = http.post(loginUrl, JSON.stringify(missingPassword), { headers });
    checkError(response, 400);
    sleep(shortSleep());

    // Test 6: Invalid email format (when using email)
    console.log('Test 6: Invalid email format');
    const invalidEmail = {
        email_or_username: 'invalid-email',
        password: testUser.password,
    };

    response = http.post(loginUrl, JSON.stringify(invalidEmail), { headers });
    checkError(response, 401); // Will be treated as username and not found
    sleep(shortSleep());

    // Test 7: Missing API key
    console.log('Test 7: Missing API key');
    const noApiKeyHeaders = {
        'Content-Type': 'application/json',
    };

    response = http.post(loginUrl, JSON.stringify(loginWithEmail), { headers: noApiKeyHeaders });
    checkError(response, 401);
    sleep(shortSleep());
}
