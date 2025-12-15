const axios = require('axios');
const { BASE_URL, API_KEY } = require('../config');

describe('GET /api/users/details - Get User Details', () => {

    let authToken = '';
    const testUser = {
        username: `getdetails_${Date.now()}`,
        email: `getdetails_${Date.now()}@example.com`,
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

    // 1. Get details without JWT
    test('Scenario 1: Get details without JWT', async () => {
        try {
            await axios.get(`${BASE_URL}/api/users/details`, {
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

    // 2. Get user details
    test('Scenario 2: Get user details', async () => {
        const response = await axios.get(`${BASE_URL}/api/users/details`, {
            headers: {
                'X-API-Key': API_KEY,
                'Authorization': `Bearer ${authToken}`
            }
        });

        expect(response.status).toBe(200);
        expect(response.data.status).toBe(true);
        expect(response.data.message).toBe("User details retrieved successfully");
        expect(response.data.data).toHaveProperty("user_details");
        expect(response.data.data.user_details).toHaveProperty("first_name");
        expect(response.data.data.user_details).toHaveProperty("last_name");
    });

    // 3. Get details when empty
    test('Scenario 3: Get details when empty', async () => {
        const response = await axios.get(`${BASE_URL}/api/users/details`, {
            headers: {
                'X-API-Key': API_KEY,
                'Authorization': `Bearer ${authToken}`
            }
        });

        expect(response.status).toBe(200);
        const details = response.data.data.user_details;
        // Since we just created user and didn't update details, fields should be null or empty
        expect(details.first_name).toBeNull();
        expect(details.last_name).toBeNull();
    });

});
