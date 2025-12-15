const axios = require('axios');
const { BASE_URL, API_KEY } = require('../config');

describe('DELETE /api/users - Delete Current User', () => {

    let authToken = '';
    const testUser = {
        username: `deluser_${Date.now()}`,
        email: `deluser_${Date.now()}@example.com`,
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

    // 1. Delete without JWT
    test('Scenario 1: Delete without JWT', async () => {
        try {
            await axios.delete(`${BASE_URL}/api/users`, {
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

    // 2. Delete current user
    test('Scenario 2: Delete current user', async () => {
        const response = await axios.delete(`${BASE_URL}/api/users`, {
            headers: {
                'X-API-Key': API_KEY,
                'Authorization': `Bearer ${authToken}`
            }
        });

        expect(response.status).toBe(200);
        expect(response.data.status).toBe(true);
        expect(response.data.message).toBe("User deleted successfully");
        expect(response.status).toBe(200);
        expect(response.data.status).toBe(true);
        expect(response.data.message).toBe("User deleted successfully");
        // Contract JSON example does not show data
    });

    // 3. Verify login after deletion
    test('Scenario 3: Verify login after deletion', async () => {
        try {
            await axios.post(`${BASE_URL}/auth/login`, {
                email_or_username: testUser.email,
                password: testUser.password
            }, { headers: { 'X-API-Key': API_KEY } });
            throw new Error('Should have failed');
        } catch (error) {
            expect([401, 404]).toContain(error.response.status);
        }
    });

    // 4. Verify access with old token
    test('Scenario 4: Verify access with old token', async () => {
        try {
            await axios.get(`${BASE_URL}/api/users`, {
                headers: {
                    'X-API-Key': API_KEY,
                    'Authorization': `Bearer ${authToken}` // Old token
                }
            });
            throw new Error('Should have failed');
        } catch (error) {
            expect([401, 404]).toContain(error.response.status);
        }
    });

    // 5. Re-create user (Data Restoration)
    test('Scenario 5: Re-create user (Data Restoration)', async () => {
        const response = await axios.post(`${BASE_URL}/auth/register`, testUser, { // Same details
            headers: { 'X-API-Key': API_KEY }
        });

        expect([200, 201]).toContain(response.status);
        expect(response.data.status).toBe(true);
        expect(response.data.message).toBe("User registered successfully");
        expect(response.data.data).toHaveProperty("user_id"); // Register response returns 'user_id'
    });

});
