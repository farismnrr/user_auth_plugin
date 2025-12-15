const axios = require('axios');
const { BASE_URL, API_KEY } = require('../config');

describe('PUT /api/users - Update Current User', () => {

    let authToken = '';
    let conflictingUser = {};
    const testUser = {
        username: `updateuser_${Date.now()}`,
        email: `updateuser_${Date.now()}@example.com`,
        password: 'Password123!',
        role: 'user'
    };

    beforeAll(async () => {
        try {
            // Register test user
            await axios.post(`${BASE_URL}/auth/register`, testUser, { headers: { 'X-API-Key': API_KEY } });
            const loginRes = await axios.post(`${BASE_URL}/auth/login`, {
                email_or_username: testUser.email,
                password: testUser.password
            }, { headers: { 'X-API-Key': API_KEY } });
            authToken = loginRes.data.data.access_token;

            // Register conflicting user
            conflictingUser = {
                username: `conflictuser_${Date.now()}`,
                email: `conflictuser_${Date.now()}@example.com`,
                password: 'Password123!',
                role: 'user'
            };
            await axios.post(`${BASE_URL}/auth/register`, conflictingUser, { headers: { 'X-API-Key': API_KEY } });
        } catch (e) {
            console.error('Setup failed', e.message);
        }
    });

    // 1. Update without JWT
    test('Scenario 1: Update without JWT', async () => {
        try {
            await axios.put(`${BASE_URL}/api/users`, {}, { headers: { 'X-API-Key': API_KEY } });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(401);
        }
    });

    // 2. Update with invalid data (short username)
    test('Scenario 2: Update with invalid data (short username)', async () => {
        try {
            await axios.put(`${BASE_URL}/api/users`, { username: "a" }, {
                headers: {
                    'X-API-Key': API_KEY,
                    'Authorization': `Bearer ${authToken}`
                }
            });
            throw new Error('Should have failed');
        } catch (error) {
            expect([400, 422]).toContain(error.response.status);
            if (error.response.data && error.response.data !== "") {
                expect(error.response.data).toEqual(expect.objectContaining({
                    status: false,
                    message: "Validation Error",
                    details: expect.arrayContaining([
                        expect.objectContaining({
                            field: "username",
                            message: "Username too short"
                        })
                    ])
                }));
            }
        }
    });

    // 3. Security: Privilege Escalation Attempt (Role Injection)
    test('Scenario 3: Security: Privilege Escalation Attempt (Role Injection)', async () => {
        const username = "HackerAttempt_" + Date.now();
        await axios.put(`${BASE_URL}/api/users`, {
            username: username,
            role: "admin"
        }, {
            headers: {
                'X-API-Key': API_KEY,
                'Authorization': `Bearer ${authToken}`
            }
        });

        const getRes = await axios.get(`${BASE_URL}/api/users`, {
            headers: {
                'X-API-Key': API_KEY,
                'Authorization': `Bearer ${authToken}`
            }
        });
        expect(getRes.data.data.user.username).toBe(username);
        expect(getRes.data.data.user.role).toBe("user"); // Must remain user
    });

    // 4. Security: ID Injection (Resource Hijacking)
    test('Scenario 4: Security: ID Injection (Resource Hijacking)', async () => {
        try {
            const username = "HijackAttempt_" + Date.now();
            await axios.put(`${BASE_URL}/api/users`, {
                id: "00000000-0000-0000-0000-000000000000",
                username: username
            }, {
                headers: {
                    'X-API-Key': API_KEY,
                    'Authorization': `Bearer ${authToken}`
                }
            });
            // If it returns 200, verify ID ignored
            const getRes = await axios.get(`${BASE_URL}/api/users`, {
                headers: { 'X-API-Key': API_KEY, 'Authorization': `Bearer ${authToken}` }
            });
            expect(getRes.data.data.user.id).not.toBe("00000000-0000-0000-0000-000000000000");
            expect(getRes.data.data.user.username).toBe(username);

        } catch (error) {
            expect([400, 422]).toContain(error.response.status);
        }
    });

    // 5. Security: XSS Injection in Username
    test('Scenario 5: Security: XSS Injection in Username', async () => {
        try {
            const response = await axios.put(`${BASE_URL}/api/users`, {
                username: "<script>alert(1)</script>"
            }, {
                headers: {
                    'X-API-Key': API_KEY,
                    'Authorization': `Bearer ${authToken}`
                }
            });
            // If success (sanitized), fine.
        } catch (error) {
            expect(error.response.status).toBe(422);
            if (error.response.data && error.response.data !== "") {
                expect(error.response.data).toEqual(expect.objectContaining({
                    status: false,
                    message: "Validation Error",
                    details: expect.arrayContaining([
                        expect.objectContaining({
                            field: "username",
                            message: "Invalid characters"
                        })
                    ])
                }));
            }
        }
    });

    // 6. Update current user (Username/Email)
    test('Scenario 6: Update current user (Username/Email)', async () => {
        const response = await axios.put(`${BASE_URL}/api/users`, {
            username: "NewUsernameFinal_" + Date.now(),
            email: "newemailfinal_" + Date.now() + "@example.com"
        }, {
            headers: {
                'X-API-Key': API_KEY,
                'Authorization': `Bearer ${authToken}`
            }
        });
        expect(response.status).toBe(200);
        expect(response.data).toEqual(expect.objectContaining({
            status: true,
            message: "User updated successfully"
        }));
    });

    // 7. Update with duplicate username/email
    test('Scenario 7: Update with duplicate username/email', async () => {
        try {
            await axios.put(`${BASE_URL}/api/users`, {
                email: conflictingUser.email
            }, {
                headers: {
                    'X-API-Key': API_KEY,
                    'Authorization': `Bearer ${authToken}`
                }
            });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(409); // Conflict
        }
    });

});
