/**
 * =============================================================================
 * ENDPOINT: GET /tenants & GET /tenants/:id
 * =============================================================================
 * 
 * Description: Retrieve tenant information
 * 
 * URL: http://localhost:5500/tenants (get all)
 *      http://localhost:5500/tenants/:id (get by ID)
 * Method: GET
 * 
 * Headers:
 *   - Content-Type: application/json
 *   - Authorization: Bearer <access_token>
 * 
 * Request Body: None
 * 
 * Success Response (200) - Get All:
 *   {
 *     "success": true,
 *     "message": "Tenants retrieved successfully",
 *     "data": [
 *       {
 *         "id": "uuid",
 *         "name": "string",
 *         "description": "string" | null,
 *         "is_active": true,
 *         "created_at": "datetime",
 *         "updated_at": "datetime"
 *       },
 *       ...
 *     ]
 *   }
 * 
 * Success Response (200) - Get by ID:
 *   {
 *     "success": true,
 *     "message": "Tenant retrieved successfully",
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
 * Error Responses:
 *   - 401 Unauthorized: Missing or invalid JWT
 *   - 404 Not Found: Tenant with specified ID not found
 * 
 * Notes:
 *   - Requires valid JWT Bearer token
 *   - GET /tenants returns only active tenants (is_active = true)
 *   - GET /tenants/:id returns specific tenant by UUID
 * 
 * Test Scenarios:
 *   1. Get all tenants with valid JWT (200 OK)
 *   2. Get tenant by ID with valid JWT (200 OK)
 *   3. Get tenants without JWT (401 Unauthorized)
 *   4. Get non-existent tenant (404 Not Found)
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
    const tenantsUrl = `${BASE_URL}/tenants`;

    // Setup: Create and login a test user
    // We need JWT authentication to access tenant endpoints
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

    // Create a test tenant first
    // This tenant will be used for GET operations testing
    const tenantName = `Tenant_${randomString(8)}`;
    const createPayload = {
        name: tenantName,
        description: 'Test tenant for get operations',
    };

    const createResponse = http.post(tenantsUrl, JSON.stringify(createPayload), { headers: authHeaders });
    const createdTenant = JSON.parse(createResponse.body).data;
    const tenantId = createdTenant.id;
    sleep(shortSleep());

    /**
     * Test Case: Get all tenants
     */
    console.log('Test 1: Get all tenants with valid JWT');
    let response = http.get(tenantsUrl, { headers: authHeaders });
    checkSuccess(response, 200, 'Tenants retrieved successfully');

    const allTenants = JSON.parse(response.body).data;
    console.log(`Number of tenants: ${allTenants.length}`);
    sleep(shortSleep());

    /**
     * Test Case: Get tenant by ID
     */
    console.log('Test 2: Get tenant by ID with valid JWT');
    const getTenantUrl = `${tenantsUrl}/${tenantId}`;
    response = http.get(getTenantUrl, { headers: authHeaders });
    checkSuccess(response, 200, 'Tenant retrieved successfully');

    const tenant = JSON.parse(response.body).data;
    console.log(`Tenant name matches: ${tenant.name === tenantName ? 'Yes' : 'No'}`);
    sleep(shortSleep());

    /**
     * Test Case: Get tenant without JWT
     */
    console.log('Test 3: Get tenants without JWT');
    const noAuthHeaders = {
        'Content-Type': 'application/json',
    };

    response = http.get(tenantsUrl, { headers: noAuthHeaders });
    checkError(response, 401);
    sleep(shortSleep());

    /**
     * Test Case: Get non-existent tenant
     */
    console.log('Test 4: Get non-existent tenant');
    const fakeId = '00000000-0000-0000-0000-000000000000';
    const fakeUrl = `${tenantsUrl}/${fakeId}`;
    response = http.get(fakeUrl, { headers: authHeaders });
    checkError(response, 404, 'not found');
    sleep(shortSleep());
}
