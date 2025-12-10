/**
 * =============================================================================
 * ENDPOINT: POST /auth/register
 * =============================================================================
 * 
 * Description: Register a new user account
 * 
 * URL: http://localhost:5500/auth/register
 * Method: POST
 * 
 * Headers:
 *   - Content-Type: application/json
 *   - X-API-Key: <api-key>
 * 
 * Request Body:
 *   {
 *     "username": "string",
 *     "email": "string",
 *     "password": "string",
 *     "role": "string"
 *   }
 * 
 * Success Response (200):
 *   {
 *     "success": true,
 *     "message": "User registered successfully",
 *     "data": {
 *       "id": "uuid",
 *       "access_token": "string"
 *     }
 *   }
 * 
 * Error Responses:
 *   - 400 Bad Request: Invalid email format, missing fields, weak password
 *   - 401 Unauthorized: Missing or invalid API key
 *   - 409 Conflict: Duplicate email or username
 * 
 * Test Scenarios:
 *   1. Successful registration
 *   2. Duplicate email
 *   3. Duplicate username
 *   4. Invalid email format
 *   5. Missing required fields
 *   6. Weak password (too short)
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
    checkSuccess,
    checkError,
    shortSleep
} from '../utils.js';

export { options };

export default function () {
    const baseUrl = `${BASE_URL}/api/auth/register`;

    /**
     * Test Case: Successful registration
     * URL: {apiUrl}/api/auth/register
     * Body: { username, email, password, role: 'user' }
     * Auth: X-API-Key
     * Expected: {
     *   "success": true,
     *   "message": "User registered successfully",
     *   "data": { "id": "...", "access_token": "..." }
     * }
     */
    console.log('Test 1: Successful registration');
    const validPayload = {
        username: randomUsername(),
        email: randomEmail(),
        password: randomPassword(),
        role: 'user',
    };

    let response = http.post(baseUrl, JSON.stringify(validPayload), { headers });
    checkSuccess(response, 200, 'registered successfully');
    sleep(shortSleep());

    /**
     * Test Case: Duplicate email
     * URL: {apiUrl}/api/auth/register
     * Body: { username: <random>, email: <existing-email>, password, role }
     * Auth: X-API-Key
     * Expected (409): {
     *   "success": false,
     *   "message": "Email already exists"
     * }
     */
    console.log('Test 2: Duplicate email');
    const duplicateEmailPayload = {
        username: randomUsername(),
        email: validPayload.email, // Same email
        password: randomPassword(),
        role: 'user',
    };

    response = http.post(baseUrl, JSON.stringify(duplicateEmailPayload), { headers });
    checkError(response, 409, 'email');
    sleep(shortSleep());

    /**
     * Test Case: Duplicate username
     * URL: {apiUrl}/api/auth/register
     * Body: { username: <existing-username>, email: <random>, password, role }
     * Auth: X-API-Key
     * Expected (409): {
     *   "success": false,
     *   "message": "Username already exists"
     * }
     */
    console.log('Test 3: Duplicate username');
    const duplicateUsernamePayload = {
        username: validPayload.username, // Same username
        email: randomEmail(),
        password: randomPassword(),
        role: 'user',
    };

    response = http.post(baseUrl, JSON.stringify(duplicateUsernamePayload), { headers });
    checkError(response, 409, 'username');
    sleep(shortSleep());

    /**
     * Test Case: Invalid email format
     * URL: {apiUrl}/api/auth/register
     * Body: { email: 'invalid-email', ... }
     * Auth: X-API-Key
     * Expected (422): {
     *   "success": false,
     *   "message": "Invalid email format"
     * }
     */
    console.log('Test 4: Invalid email format');
    const invalidEmailPayload = {
        username: randomUsername(),
        email: 'invalid-email',
        password: randomPassword(),
        role: 'user',
    };

    response = http.post(baseUrl, JSON.stringify(invalidEmailPayload), { headers });
    checkError(response, 422);
    sleep(shortSleep());

    /**
     * Test Case: Missing required fields
     * URL: {apiUrl}/api/auth/register
     * Body: { ... } (missing email)
     * Auth: X-API-Key
     * Expected (400): {
     *   "success": false,
     *   "message": "Missing required fields"
     * }
     */
    console.log('Test 5: Missing required fields (no email)');
    const missingFieldPayload = {
        username: randomUsername(),
        password: randomPassword(),
        role: 'user',
    };

    response = http.post(baseUrl, JSON.stringify(missingFieldPayload), { headers });
    checkError(response, 400);
    sleep(shortSleep());

    /**
     * Test Case: Weak password
     * URL: {apiUrl}/api/auth/register
     * Body: { password: '123', ... }
     * Auth: X-API-Key
     * Expected (422): {
     *   "success": false,
     *   "message": "Password too short"
     * }
     */
    console.log('Test 6: Weak password');
    const weakPasswordPayload = {
        username: randomUsername(),
        email: randomEmail(),
        password: '123',
        role: 'user',
    };

    response = http.post(baseUrl, JSON.stringify(weakPasswordPayload), { headers });
    checkError(response, 422);
    sleep(shortSleep());

    /**
     * Test Case: Missing API key
     * URL: {apiUrl}/api/auth/register
     * Body: { ... } (valid payload)
     * Auth: None (Headers missing X-API-Key)
     * Expected (401): {
     *   "success": false,
     *   "message": "Missing API Key"
     * }
     */
    console.log('Test 7: Missing API key');
    const noApiKeyHeaders = {
        'Content-Type': 'application/json',
    };

    response = http.post(baseUrl, JSON.stringify(validPayload), { headers: noApiKeyHeaders });
    checkError(response, 401);
    sleep(shortSleep());
}
