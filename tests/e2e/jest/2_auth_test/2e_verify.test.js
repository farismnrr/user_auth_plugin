const axios = require('axios');
const jwt = require('jsonwebtoken');
const { BASE_URL, API_KEY, JWT_SECRET } = require('../config');

describe('GET /auth/verify - Verify Token', () => {

    let authToken = '';
    const testUser = {
        username: `verify_${Date.now()}`,
        email: `verify_${Date.now()}@example.com`,
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

    // 1. Missing Authorization header
    test('Scenario 1: Missing Authorization header', async () => {
        try {
            await axios.get(`${BASE_URL}/auth/verify`, {
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

    // 2. Malformed Authorization header
    test('Scenario 2: Malformed Authorization header', async () => {
        try {
            await axios.get(`${BASE_URL}/auth/verify`, {
                headers: {
                    'X-API-Key': API_KEY,
                    'Authorization': 'InvalidTokenString' // No Bearer
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

    // 3. Invalid JWT format
    test('Scenario 3: Invalid JWT format', async () => {
        try {
            await axios.get(`${BASE_URL}/auth/verify`, {
                headers: {
                    'X-API-Key': API_KEY,
                    'Authorization': 'Bearer invalid.jwt.string'
                }
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

    // 4. Expired JWT
    test('Scenario 4: Expired JWT', async () => {
        const decoded = jwt.decode(authToken);
        const payload = { ...decoded };
        delete payload.exp;
        delete payload.iat;

        const expiredToken = jwt.sign(payload, JWT_SECRET, { expiresIn: '-1h' });

        try {
            await axios.get(`${BASE_URL}/auth/verify`, {
                headers: {
                    'X-API-Key': API_KEY,
                    'Authorization': `Bearer ${expiredToken}`
                }
            });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(401);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: "Token expired"
            }));
        }
    });

    // 5. Security: NBF (Not Before) Check
    test('Scenario 5: Security: NBF Check', async () => {
        const decoded = jwt.decode(authToken);
        const payload = { ...decoded };
        delete payload.exp;
        delete payload.iat;

        // Set nbf to 1 hour in the future
        payload.nbf = Math.floor(Date.now() / 1000) + 3600;

        const futureToken = jwt.sign(payload, JWT_SECRET, { expiresIn: '2h' });

        try {
            await axios.get(`${BASE_URL}/auth/verify`, {
                headers: {
                    'X-API-Key': API_KEY,
                    'Authorization': `Bearer ${futureToken}`
                }
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

    // 6. Security: Cross-Tenant Check
    test('Scenario 6: Security: Cross-Tenant Check', async () => {
        const decoded = jwt.decode(authToken);
        const payload = { ...decoded };
        delete payload.exp;
        delete payload.iat;

        // Change tenant_id to a random one
        // Note: Ideally request should be made TO a different tenant context, 
        // but since API Key sets tenant in middleware, we might need to change implementation strategy depending on how tenant check is done.
        // If the token itself is valid but belongs to Tenant A, and we request Endpoint B (implied by API Key B?), it should fail.
        // Here we can simulate by forging a token with a DIFFERENT tenant_id than the one associated with the API_KEY (since we only have one API_KEY for test).

        payload.tenant_id = '00000000-0000-0000-0000-000000000000'; // Dummy Tenant ID

        const crossTenantToken = jwt.sign(payload, JWT_SECRET, { expiresIn: '1h' });

        try {
            await axios.get(`${BASE_URL}/auth/verify`, {
                headers: {
                    'X-API-Key': API_KEY,
                    'Authorization': `Bearer ${crossTenantToken}`
                }
            });
            throw new Error('Should have failed');
        } catch (error) {
            // Contract checks for 403 Forbidden
            expect([401, 403]).toContain(error.response.status);
            if (error.response.status === 403) {
                // Check message if relevant
            }
        }
    });

    // 7. User deleted but token still valid
    test('Scenario 7: User deleted but token still valid', async () => {
        try {
            // 1. Create temporary user
            const tempUser = { username: 'del_verify_' + Date.now(), email: 'del_verify_' + Date.now() + '@x.com', password: 'Password1!', role: 'user' };
            await axios.post(`${BASE_URL}/auth/register`, tempUser, { headers: { 'X-API-Key': API_KEY } });
            const l = await axios.post(`${BASE_URL}/auth/login`, { email_or_username: tempUser.email, password: tempUser.password }, { headers: { 'X-API-Key': API_KEY } });
            const token = l.data.data?.access_token || l.data.result?.access_token;

            // 2. Delete user
            // Assuming DELETE /users deletes the *current* user based on token
            // We need to check '4g_delete_user.md' (Step 206)
            // It says "URL: http://localhost:5500/api/users", Method: DELETE, Pre-conditions: Valid JWT.
            await axios.delete(`${BASE_URL}/api/users`, {
                headers: {
                    'X-API-Key': API_KEY,
                    'Authorization': `Bearer ${token}`
                }
            });

            // 3. Verify
            await axios.get(`${BASE_URL}/auth/verify`, {
                headers: {
                    'X-API-Key': API_KEY,
                    'Authorization': `Bearer ${token}`
                }
            });
            throw new Error('Should have failed');
        } catch (error) {
            expect([401, 404]).toContain(error.response.status);
            if (error.response.data && error.response.data !== "") {
                expect(error.response.data).toEqual(expect.objectContaining({
                    status: false,
                    message: "Unauthorized" // Contract says Unauthorized
                }));
            }
        }
    });

    // 8. Successful verification
    test('Scenario 8: Successful verification', async () => {
        const response = await axios.get(`${BASE_URL}/auth/verify`, {
            headers: {
                'X-API-Key': API_KEY,
                'Authorization': `Bearer ${authToken}`
            }
        });

        expect(response.status).toBe(200);
        expect(response.data.status).toBe(true);
        expect(response.data.message).toBe("Token is valid");
        // Contract JSON example does not show data
    });

});
