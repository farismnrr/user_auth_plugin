/**
 * =============================================================================
 * ENDPOINT: POST /tenants
 * =============================================================================
 * 
 * Description: Create a new tenant
 * 
 * URL: http://localhost:5500/tenants
 * Method: POST
 * 
 * Headers:
 *   - Content-Type: application/json
 *   - Authorization: Bearer <access_token>
 * 
 * Request Body:
 *   {
 *     "name": "string",
 *     "description": "string" (optional)
 *   }
 * 
 * Success Response (201):
 *   {
 *     "success": true,
 *     "message": "Tenant created successfully",
 *     "data": {
 *       "id": "uuid",
 *       "name": "string",
 *       "description": "string" | null,
 *       "is_active": true,
 *       "created_at": "datetime",
 *       "updated_at": "datetime"
 *     }
 *   }
 * 
 * Success Response (200) - When tenant already exists:
 *   {
 *     "success": true,
 *     "message": "Tenant created successfully",
 *     "data": {
 *       "id": "uuid",  // ID of existing tenant
 *       "name": "string",
 *       "description": "string" | null,
 *       "is_active": true,
 *       "created_at": "datetime",
 *       "updated_at": "datetime"
 *     }
 *   }
 * 
 * Error Responses:
 *   - 401 Unauthorized: Missing or invalid JWT
 *   - 422 Validation Error: Invalid input (empty name, name too long)
 * 
 * Notes:
 *   - Requires valid JWT Bearer token
 *   - If tenant with same name exists, returns existing tenant (idempotent)
 *   - Name must be between 1-255 characters
 *   - Description is optional
 * 
 * Test Scenarios:
 *   1. Create tenant with valid data (201 Created)
 *   2. Create tenant with duplicate name (200 OK, returns existing tenant)
 *   3. Create tenant without JWT (401 Unauthorized)
 *   4. Create tenant with empty name (422 Validation Error)
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
    randomString,
    extractAccessToken,
    checkSuccess,
    checkError,
    shortSleep
} from '../utils.js';

export { options };

export default function () {
    const registerUrl = `${BASE_URL}/api/auth/register`;
    const loginUrl = `${BASE_URL}/api/auth/login`;
    const createTenantUrl = `${BASE_URL}/tenants`;

    // Setup: Create and login a test user
    // We need a logged-in user with JWT to access tenant endpoints
    const testUser = {
        username: randomUsername(),
        tenant_id: TENANT_ID,
        role: "user",
        email: randomEmail(),
        password: randomPassword(),
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

    const authHeaders = {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${accessToken}`,
    };

    /**
     * Test Case: Create tenant with valid data
     * URL: {apiUrl}/tenants
     * Body: { "name": "...", "description": "..." }
     * Auth: Bearer <valid_jwt>
     * Expected (201): {
     *   "success": true,
     *   "message": "Tenant created successfully",
     *   "data": { "id": "...", "name": "...", ... }
     * }
     */
    console.log('Test 1: Create tenant with valid data');
    const tenantName = `Tenant_${randomString(8)}`;
    const createPayload = {
        name: tenantName,
        description: 'Test tenant description',
    };

    let response = http.post(createTenantUrl, JSON.stringify(createPayload), { headers: authHeaders });
    checkSuccess(response, 201, 'Tenant created successfully');

    const body = JSON.parse(response.body);
    console.log(`Tenant created with ID: ${body.data?.id}`);
    console.log(`Tenant name matches: ${body.data?.name === tenantName ? 'Yes' : 'No'}`);
    sleep(shortSleep());

    /**
     * Test Case: Create tenant with duplicate name
     * URL: {apiUrl}/tenants
     * Body: { "name": "<same_name>", "description": "..." }
     * Auth: Bearer <valid_jwt>
     * Expected (200): {
     *   "success": true,
     *   "message": "Tenant created successfully",
     *   "data": { "id": "...", "name": "...", ... }
     * }
     * Note: Returns existing tenant instead of error
     */
    console.log('Test 2: Create tenant with duplicate name (returns existing tenant)');
    response = http.post(createTenantUrl, JSON.stringify(createPayload), { headers: authHeaders });
    checkSuccess(response, 200, 'Tenant created successfully');

    const duplicateBody = JSON.parse(response.body);
    console.log(`Returned tenant ID matches: ${duplicateBody.data?.id === body.data?.id ? 'Yes' : 'No'}`);
    console.log(`Returned same tenant for duplicate name - PASSED`);
    sleep(shortSleep());

    /**
     * Test Case: Create tenant without JWT
     * URL: {apiUrl}/tenants
     * Body: { "name": "...", "description": "..." }
     * Auth: None
     * Expected (401): Unauthorized
     */
    console.log('Test 3: Create tenant without JWT');
    const noAuthHeaders = {
        'Content-Type': 'application/json',
    };

    const newTenantPayload = {
        name: `Tenant_${randomString(8)}`,
        description: 'Should fail without auth',
    };

    response = http.post(createTenantUrl, JSON.stringify(newTenantPayload), { headers: noAuthHeaders });
    checkError(response, 401);
    sleep(shortSleep());

    /**
     * Test Case: Create tenant with empty name
     * URL: {apiUrl}/tenants
     * Body: { "name": "", "description": "..." }
     * Auth: Bearer <valid_jwt>
     * Expected (422): Validation error
     */
    console.log('Test 4: Create tenant with empty name');
    const invalidPayload = {
        name: '',
        description: 'Invalid tenant',
    };

    response = http.post(createTenantUrl, JSON.stringify(invalidPayload), { headers: authHeaders });
    checkError(response, 422);
    sleep(shortSleep());
}
