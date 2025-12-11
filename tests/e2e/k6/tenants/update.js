/**
 * =============================================================================
 * ENDPOINT: PUT /tenants/:id
 * =============================================================================
 * 
 * Description: Update an existing tenant
 * 
 * URL: http://localhost:5500/tenants/:id
 * Method: PUT
 * 
 * Headers:
 *   - Content-Type: application/json
 *   - Authorization: Bearer <access_token>
 * 
 * Request Body (all fields optional):
 *   {
 *     "name": "string",
 *     "description": "string",
 *     "is_active": boolean
 *   }
 * 
 * Success Response (200):
 *   {
 *     "success": true,
 *     "message": "Tenant updated successfully",
 *     "data": {
 *       "id": "uuid",
 *       "name": "string",
 *       "description": "string" | null,
 *       "is_active": boolean,
 *       "created_at": "datetime",
 *       "updated_at": "datetime"  // Updated timestamp
 *     }
 *   }
 * 
 * Error Responses:
 *   - 401 Unauthorized: Missing or invalid JWT
 *   - 404 Not Found: Tenant with specified ID not found
 *   - 409 Conflict: New name conflicts with existing tenant
 *   - 422 Validation Error: Invalid input (empty name, name too long)
 * 
 * Notes:
 *   - Requires valid JWT Bearer token
 *   - All fields in request body are optional (partial update)
 *   - Name must be unique across all tenants
 *   - Name must be between 1-255 characters if provided
 *   - updated_at timestamp is automatically updated
 * 
 * Test Scenarios:
 *   1. Update tenant with valid data (200 OK)
 *   2. Update tenant without JWT (401 Unauthorized)
 *   3. Update non-existent tenant (404 Not Found)
 *   4. Update with duplicate name (409 Conflict)
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
    // JWT authentication is required for all tenant operations
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
    // This tenant will be updated in the tests
    const tenantName = `Tenant_${randomString(8)}`;
    const createPayload = {
        name: tenantName,
        description: 'Test tenant for update operations',
    };

    const createResponse = http.post(tenantsUrl, JSON.stringify(createPayload), { headers: authHeaders });
    const createdTenant = JSON.parse(createResponse.body).data;
    const tenantId = createdTenant.id;
    sleep(shortSleep());

    /**
     * Test Case: Update tenant with valid data
     */
    console.log('Test 1: Update tenant with valid data');
    const updateUrl = `${tenantsUrl}/${tenantId}`;
    const updatedName = `Updated_${randomString(8)}`;
    const updatePayload = {
        name: updatedName,
        description: 'Updated description',
        is_active: true,
    };

    let response = http.put(updateUrl, JSON.stringify(updatePayload), { headers: authHeaders });
    checkSuccess(response, 200, 'Tenant updated successfully');

    const updatedTenant = JSON.parse(response.body).data;
    console.log(`Tenant name updated: ${updatedTenant.name === updatedName ? 'Yes' : 'No'}`);
    sleep(shortSleep());

    /**
     * Test Case: Update tenant without JWT
     */
    console.log('Test 2: Update tenant without JWT');
    const noAuthHeaders = {
        'Content-Type': 'application/json',
    };

    const anotherUpdate = {
        name: `NoAuth_${randomString(8)}`,
    };

    response = http.put(updateUrl, JSON.stringify(anotherUpdate), { headers: noAuthHeaders });
    checkError(response, 401);
    sleep(shortSleep());

    /**
     * Test Case: Update non-existent tenant
     */
    console.log('Test 3: Update non-existent tenant');
    const fakeId = '00000000-0000-0000-0000-000000000000';
    const fakeUrl = `${tenantsUrl}/${fakeId}`;
    const fakeUpdate = {
        name: `Fake_${randomString(8)}`,
    };

    response = http.put(fakeUrl, JSON.stringify(fakeUpdate), { headers: authHeaders });
    checkError(response, 404, 'not found');
    sleep(shortSleep());

    /**
     * Test Case: Update with duplicate name
     */
    console.log('Test 4: Update with duplicate name');
    // Create another tenant
    const anotherTenant = {
        name: `Another_${randomString(8)}`,
        description: 'Another tenant',
    };
    const anotherResponse = http.post(tenantsUrl, JSON.stringify(anotherTenant), { headers: authHeaders });
    const anotherTenantData = JSON.parse(anotherResponse.body).data;
    sleep(shortSleep());

    // Try to update first tenant with second tenant's name
    const duplicateUpdate = {
        name: anotherTenantData.name,
    };

    response = http.put(updateUrl, JSON.stringify(duplicateUpdate), { headers: authHeaders });
    checkError(response, 409, 'already exists');
    sleep(shortSleep());
}
