/**
 * =============================================================================
 * ENDPOINT: PUT /auth/change-password
 * =============================================================================
 * 
 * Description: Change authenticated user's password
 * 
 * URL: http://localhost:5500/auth/change-password
 * Method: PUT
 * 
 * Headers:
 *   - Content-Type: application/json
 *   - Authorization: Bearer <access-token>
 * 
 * Request Body:
 *   {
 *     "old_password": "string",
 *     "new_password": "string",
 *     "confirm_new_password": "string"
 *   }
 * 
 * Success Response (200):
 *   {
 *     "success": true,
 *     "message": "Password changed successfully"
 *   }
 * 
 * Error Responses:
 *   - 401 Unauthorized: Wrong old password, missing/invalid JWT token
 *   - 422 Validation Error: Passwords don't match, weak password
 * 
 * Test Scenarios:
 *   1. Successful password change
 *   2. Wrong old password
 *   3. New passwords don't match
 *   4. Weak new password
 *   5. Missing JWT token
 *   6. Verify login with new password works
 * 
 * =============================================================================
 */

import http from 'k6/http';
import { sleep } from 'k6';
import { options } from '../config.js';
import { BASE_URL, API_KEY, getTestTenantId, registerTestUser } from '../helpers.js';
import {
    randomPassword,
    extractAccessToken,
    checkSuccess,
    checkError,
    shortSleep
} from '../utils.js';

const headers = { 'Content-Type': 'application/json', 'X-API-Key': API_KEY };

export { options };

export default function () {
    const loginUrl = `${BASE_URL}/api/auth/login`;
    const changePasswordUrl = `${BASE_URL}/auth/change-password`;

    // Setup: Create a test user first
    const tenantId = getTestTenantId();
    const testUser = registerTestUser(tenantId, 'user');
    sleep(shortSleep());

    // Login to get access token
    const loginPayload = {
        email_or_username: testUser.email,
        password: testUser.password,
        tenant_id: tenantId,
    };

    let response = http.post(loginUrl, JSON.stringify(loginPayload), { headers });
    const accessToken = extractAccessToken(response);
    sleep(shortSleep());

    const authHeaders = {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${accessToken}`,
    };

    /**
     * Test Case: Successful password change
     * URL: {apiUrl}/auth/change-password
     * Body: { old_password, new_password, confirm_new_password }
     * Auth: Bearer token
     * Expected: {
     *   "success": true,
     *   "message": "Password changed successfully"
     * }
     */
    console.log('Test 1: Successful password change');
    const newPassword = randomPassword();
    const changePasswordData = {
        old_password: testUser.password,
        new_password: newPassword,
        confirm_new_password: newPassword,
    };

    response = http.put(changePasswordUrl, JSON.stringify(changePasswordData), { headers: authHeaders });
    checkSuccess(response, 200, 'Password changed successfully');
    sleep(shortSleep());

    /**
     * Test Case: Verify login with new password works
     * URL: {apiUrl}/api/auth/login
     * Body: { email_or_username: <email>, password: <new_password> }
     * Auth: X-API-Key
     * Expected: {
     *   "success": true,
     *   "message": "Login successful"
     * }
     */
    console.log('Test 2: Verify login with new password');
    const loginWithNewPassword = {
        email_or_username: testUser.email,
        password: newPassword,
    };

    response = http.post(loginUrl, JSON.stringify(loginWithNewPassword), { headers });
    checkSuccess(response, 200, 'Login successful');
    const newAccessToken = extractAccessToken(response);
    sleep(shortSleep());

    const newAuthHeaders = {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${newAccessToken}`,
    };

    /**
     * Test Case: Wrong old password
     * URL: {apiUrl}/auth/change-password
     * Body: { old_password: 'WrongPassword123!', new_password, confirm_new_password }
     * Auth: Bearer token
     * Expected (401): {
     *   "success": false,
     *   "message": "Old password is incorrect"
     * }
     */
    console.log('Test 3: Wrong old password');
    const wrongOldPassword = {
        old_password: 'WrongPassword123!',
        new_password: randomPassword(),
        confirm_new_password: randomPassword(),
    };

    response = http.put(changePasswordUrl, JSON.stringify(wrongOldPassword), { headers: newAuthHeaders });
    checkError(response, 401);
    sleep(shortSleep());

    /**
     * Test Case: New passwords don't match
     * URL: {apiUrl}/auth/change-password
     * Body: { old_password, new_password: 'Pass1', confirm_new_password: 'Pass2' }
     * Auth: Bearer token
     * Expected (422): {
     *   "success": false,
     *   "message": "New passwords do not match"
     * }
     */
    console.log('Test 4: New passwords don\'t match');
    const passwordMismatch = {
        old_password: newPassword,
        new_password: 'NewPassword123!',
        confirm_new_password: 'DifferentPassword123!',
    };

    response = http.put(changePasswordUrl, JSON.stringify(passwordMismatch), { headers: newAuthHeaders });
    checkError(response, 422);
    sleep(shortSleep());

    /**
     * Test Case: Weak new password
     * URL: {apiUrl}/auth/change-password
     * Body: { old_password, new_password: 'weak', confirm_new_password: 'weak' }
     * Auth: Bearer token
     * Expected (422): {
     *   "success": false,
     *   "message": "Password must be at least 8 characters..."
     * }
     */
    console.log('Test 5: Weak new password');
    const weakPassword = {
        old_password: newPassword,
        new_password: 'weak',
        confirm_new_password: 'weak',
    };

    response = http.put(changePasswordUrl, JSON.stringify(weakPassword), { headers: newAuthHeaders });
    checkError(response, 422);
    sleep(shortSleep());

    /**
     * Test Case: Missing JWT token
     * URL: {apiUrl}/auth/change-password
     * Body: { old_password, new_password, confirm_new_password }
     * Auth: None (no Authorization header)
     * Expected (401): {
     *   "success": false,
     *   "message": "Missing authorization header"
     * }
     */
    console.log('Test 6: Missing JWT token');
    const noAuthHeaders = {
        'Content-Type': 'application/json',
    };

    response = http.put(changePasswordUrl, JSON.stringify(changePasswordData), { headers: noAuthHeaders });
    checkError(response, 401);
    sleep(shortSleep());
}
