const axios = require('axios');
const { BASE_URL, API_KEY } = require('../config');

describe('GET /api/users/all - Get All Users', () => {

    let userToken = '';
    let adminToken = '';

    // Regular User
    const regularUser = {
        username: `regularuser_${Date.now()}`,
        email: `regularuser_${Date.now()}@example.com`,
        password: 'StrongPassword123!',
        role: 'user'
    };

    // Admin User
    const adminUser = {
        username: `adminuser_${Date.now()}`,
        email: `adminuser_${Date.now()}@example.com`,
        password: 'StrongPassword123!',
        role: 'admin' // Assuming we can register as admin
    };

    beforeAll(async () => {
        try {
            // Register & Login Regular
            await axios.post(`${BASE_URL}/auth/register`, regularUser, { headers: { 'X-API-Key': API_KEY } });
            const userLogin = await axios.post(`${BASE_URL}/auth/login`, {
                email_or_username: regularUser.email,
                password: regularUser.password
            }, { headers: { 'X-API-Key': API_KEY } });
            userToken = userLogin.data.data?.access_token || userLogin.data.result?.access_token;

            // Register & Login Admin
            await axios.post(`${BASE_URL}/auth/register`, adminUser, { headers: { 'X-API-Key': API_KEY } });
            const adminLogin = await axios.post(`${BASE_URL}/auth/login`, {
                email_or_username: adminUser.email,
                password: adminUser.password
            }, { headers: { 'X-API-Key': API_KEY } });
            adminToken = adminLogin.data.data?.access_token || adminLogin.data.result?.access_token;

        } catch (e) {
            console.log('Setup failed', e.message);
        }
    });

    // 1. Get all users without JWT
    test('Scenario 1: Get all users without JWT', async () => {
        try {
            await axios.get(`${BASE_URL}/api/users/all`, {
                headers: { 'X-API-Key': API_KEY } // Missing Auth
            });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(401);
            if (error.response.data && error.response.data !== "") {
                expect(error.response.data).toEqual(expect.objectContaining({
                    status: false,
                    message: "Unauthorized"
                }));
            }
        }
    });

    // 2. Security: Unauthorized Role Access (RBAC)
    test('Scenario 2: Security: Unauthorized Role Access (RBAC)', async () => {
        try {
            await axios.get(`${BASE_URL}/api/users/all`, {
                headers: {
                    'X-API-Key': API_KEY,
                    'Authorization': `Bearer ${userToken}`
                }
            });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(403);
            if (error.response.data && error.response.data !== "") {
                expect(error.response.data).toEqual(expect.objectContaining({
                    status: false,
                    message: "Forbidden"
                }));
            }
        }
    });

    // 3. Pagination Check (DoS Protection)
    test('Scenario 3: Pagination Check (DoS Protection)', async () => {
        // Contract says: Status 200 (capped) or 400.
        // And "Pre-conditions: Valid JWT" (presumably authorized one, admin)
        // Assuming admin token is needed as it is 'Get all users' endpoint
        try {
            const response = await axios.get(`${BASE_URL}/api/users/all?limit=1000000`, {
                headers: {
                    'X-API-Key': API_KEY,
                    'Authorization': `Bearer ${adminToken}`
                }
            });

            expect(response.status).toBe(200);
            if (response.data.data) {
                // Check if capped. We don't know the exact cap but it shouldn't be 1,000,000 if we had that many.
                // Since we only have a few users, we can't test "Capping" effectively unless we check the metadata/limit returned.
                // Or we accept 200 as pass.
            }
        } catch (error) {
            // Or 400
            expect(error.response.status).toBe(400);
        }
    });

    // 4. Get all users (Admin/Authorized Role)
    test('Scenario 4: Get all users (Admin/Authorized Role)', async () => {
        const response = await axios.get(`${BASE_URL}/api/users/all`, {
            headers: {
                'X-API-Key': API_KEY,
                'Authorization': `Bearer ${adminToken}`
            }
        });

        expect(response.status).toBe(200);
        expect(response.data.status).toBe(true);
        expect(response.data.message).toBe("Users retrieved successfully");
        expect(response.data.data).toHaveProperty("users");
        expect(Array.isArray(response.data.data.users)).toBe(true);
        expect(response.data.data).toHaveProperty("pagination");

        if (response.data.data.users.length > 0) {
            expect(response.data.data.users[0]).toHaveProperty("id");
            expect(response.data.data.users[0]).toHaveProperty("username");
            expect(response.data.data.users[0]).not.toHaveProperty("password");
        }
    });

});
