const axios = require('axios');
const { BASE_URL, TENANT_SECRET_KEY } = require('../config');

describe('POST /api/tenants - Create Tenant (Bootstrapping)', () => {

    // 1. Create tenant without authentication (401 Unauthorized)
    test('Scenario 1: Create tenant without authentication', async () => {
        try {
            await axios.post(`${BASE_URL}/api/tenants`, { name: "Tenant1" });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(401);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: "Unauthorized"
            }));
        }
    });

    // 2. Create tenant with invalid secret key (401 Unauthorized)
    test('Scenario 2: Create tenant with invalid secret key', async () => {
        try {
            await axios.post(`${BASE_URL}/api/tenants`, { name: "Tenant2" }, {
                headers: { 'X-Tenant-Secret-Key': 'invalid_key' }
            });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(401);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: "Unauthorized"
            }));
        }
    });

    // 3. Invalid Content-Type (415 Unsupported Media Type)
    test('Scenario 3: Invalid Content-Type', async () => {
        try {
            await axios.post(`${BASE_URL}/api/tenants`, 'name=Tenant3', {
                headers: {
                    'X-Tenant-Secret-Key': TENANT_SECRET_KEY,
                    'Content-Type': 'application/x-www-form-urlencoded'
                }
            });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(415);
        }
    });

    // 4. Malformed Request: Empty body or invalid JSON (400 Bad Request)
    test('Scenario 4: Malformed Request', async () => {
        try {
            await axios.post(`${BASE_URL}/api/tenants`, {}, {
                headers: { 'X-Tenant-Secret-Key': TENANT_SECRET_KEY }
            });
            // Some APIs return 400 for empty JSON if fields required, or validation error
            // Contract says 400 Bad Request
        } catch (error) {
            expect(error.response.status).toBe(400);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: "Bad Request"
            }));
        }
    });

    // 5. Create tenant with empty name (422 Validation Error)
    test('Scenario 5: Create tenant with empty name', async () => {
        try {
            await axios.post(`${BASE_URL}/api/tenants`, { name: "" }, {
                headers: { 'X-Tenant-Secret-Key': TENANT_SECRET_KEY }
            });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(422);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: "Validation Error",
                details: expect.arrayContaining([
                    expect.objectContaining({
                        field: "name",
                        message: "Name cannot be empty"
                    })
                ])
            }));
        }
    });

    // 6. Validation: Name too long (> 255 chars) (422 Validation Error)
    test('Scenario 6: Validation: Name too long', async () => {
        const longName = 'a'.repeat(256);
        try {
            await axios.post(`${BASE_URL}/api/tenants`, { name: longName }, {
                headers: { 'X-Tenant-Secret-Key': TENANT_SECRET_KEY }
            });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(422);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: "Validation Error",
                details: expect.arrayContaining([
                    expect.objectContaining({
                        field: "name",
                        message: "Name too long"
                    })
                ])
            }));
        }
    });

    // 7. Validation: Name too short / empty string (422 Validation Error)
    test('Scenario 7: Validation: Name too short', async () => {
        try {
            await axios.post(`${BASE_URL}/api/tenants`, { name: "" }, {
                headers: { 'X-Tenant-Secret-Key': TENANT_SECRET_KEY }
            });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(422);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: "Validation Error",
                details: expect.arrayContaining([
                    expect.objectContaining({
                        field: "name",
                        message: "Name cannot be empty"
                    })
                ])
            }));
        }
    });

    // 8. Validation: Invalid characters in name (422/Sanitized)
    test('Scenario 8: Validation: Invalid characters in name', async () => {
        try {
            const res = await axios.post(`${BASE_URL}/api/tenants`, { name: "Tenant😳" }, {
                headers: { 'X-Tenant-Secret-Key': TENANT_SECRET_KEY }
            });
            expect([201, 200]).toContain(res.status);
        } catch (error) {
            expect(error.response.status).toBe(422);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: "Validation Error",
                details: expect.arrayContaining([
                    expect.objectContaining({
                        field: "name",
                        message: "Invalid characters in name"
                    })
                ])
            }));
        }
    });

    // 9. Validation: SQL Injection payload in name (422/Sanitized)
    test('Scenario 9: Validation: SQL Injection payload', async () => {
        try {
            const res = await axios.post(`${BASE_URL}/api/tenants`, { name: "Tenant'; DROP TABLE tenants; --" }, {
                headers: { 'X-Tenant-Secret-Key': TENANT_SECRET_KEY }
            });
            expect([201, 200]).toContain(res.status);
        } catch (error) {
            expect(error.response.status).toBe(422);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: "Validation Error",
                details: expect.arrayContaining([
                    expect.objectContaining({
                        field: "name",
                        message: "Invalid characters in name"
                    })
                ])
            }));
        }
    });

    // 10. Create tenant with tenant secret key (201 Created)
    let createdTenantId;
    const uniqueName = "Test_Tenant_" + Date.now();

    test('Scenario 10: Create tenant with tenant secret key', async () => {
        const response = await axios.post(`${BASE_URL}/api/tenants`, {
            name: uniqueName,
            description: "Test Tenant"
        }, {
            headers: { 'X-Tenant-Secret-Key': TENANT_SECRET_KEY }
        });

        expect(response.status).toBe(201);
        expect(response.data.status).toBe(true);
        expect(response.data.message).toBe("Tenant created successfully");
        expect(response.data.data).toHaveProperty("tenant_id");
        createdTenantId = response.data.data.tenant_id;
    });

    // 11. Create tenant with duplicate name using secret key (200 OK - Idempotent)
    test('Scenario 11: Create tenant with duplicate name', async () => {
        const response = await axios.post(`${BASE_URL}/api/tenants`, {
            name: uniqueName, // Same name as Scenario 10
            description: "Duplicate"
        }, {
            headers: { 'X-Tenant-Secret-Key': TENANT_SECRET_KEY }
        });

        expect(response.status).toBe(200);
        expect(response.data.status).toBe(true);
        expect(response.data.message).toBe("Tenant already exists");
        expect(response.data.data).toHaveProperty("tenant_id");
        expect(response.data.data.tenant_id).toBe(createdTenantId);
    });

});
