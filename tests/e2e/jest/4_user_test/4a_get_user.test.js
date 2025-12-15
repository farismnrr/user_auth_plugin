const axios = require('axios');
const { BASE_URL, API_KEY } = require('../config');

describe('GET /api/users - Get Current User', () => {

    let authToken = '';
    const testUser = {
        username: `getme_${Date.now()}`,
        email: `getme_${Date.now()}@example.com`,
        password: 'StrongPassword123!',
        role: 'user'
    };

    beforeAll(async () => {
        try {
            await axios.post(`${BASE_URL}/auth/register`, testUser, { headers: { 'X-API-Key': API_KEY } });
            const loginRes = await axios.post(`${BASE_URL}/auth/login`, {
                email_or_username: testUser.email,
                password: testUser.password
            }, { headers: { 'X-API-Key': API_KEY } });
            authToken = loginRes.data.data?.access_token || loginRes.data.result?.access_token;
        } catch (e) {
            console.log('Setup failed', e.message);
        }
    });

    // 1. Get user without JWT
    test('Scenario 1: Get user without JWT', async () => {
        try {
            await axios.get(`${BASE_URL}/api/users`, {
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

    // 2. Get user with invalid JWT
    test('Scenario 2: Get user with invalid JWT', async () => {
        try {
            await axios.get(`${BASE_URL}/api/users`, {
                headers: {
                    'X-API-Key': API_KEY,
                    'Authorization': 'Bearer invalid_token'
                }
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

    // 3. Get current user profile
    test('Scenario 3: Get current user profile', async () => {
        const response = await axios.get(`${BASE_URL}/api/users`, {
            headers: {
                'X-API-Key': API_KEY,
                'Authorization': `Bearer ${authToken}`
            }
        });

        expect(response.status).toBe(200);
        expect(response.data.status).toBe(true);
        expect(response.data.message).toBe("User retrieved successfully");
        expect(response.data.data).toHaveProperty("user");
        expect(response.data.data.user).toHaveProperty("id");
        expect(response.data.data.user).toHaveProperty("username", testUser.username);
        expect(response.data.data.user).toHaveProperty("email", testUser.email);
        expect(response.data.data.user).toHaveProperty("role", "user");
    });

    // 4. Verify password exclusion
    test('Scenario 4: Verify password exclusion', async () => {
        const response = await axios.get(`${BASE_URL}/api/users`, {
            headers: {
                'X-API-Key': API_KEY,
                'Authorization': `Bearer ${authToken}`
            }
        });

        expect(response.status).toBe(200);
        expect(response.data.status).toBe(true);
        expect(response.data.data).not.toHaveProperty("password"); // Check root data just in case
        if (response.data.data.user) {
            expect(response.data.data.user).not.toHaveProperty("password");
        }
    });

});
