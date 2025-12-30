const axios = require('axios');
const { BASE_URL, API_KEY } = require('../config');

describe('POST /auth/register - Register User', () => {

    let uniqueEmail; // Shared for scenario 9 and 12

    // 1. Missing API key
    test('Scenario 1: Missing API key', async () => {
        try {
            await axios.post(`${BASE_URL}/auth/register`, {
                username: "UserKeyMissing",
                email: "keymissing@example.com",
                password: "StrongPassword123!",
                role: "user"
            }); // No Headers
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(401);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: "Unauthorized"
            }));
        }
    });

    // 2. Invalid email format
    test('Scenario 2: Invalid email format', async () => {
        try {
            await axios.post(`${BASE_URL}/auth/register`, {
                username: "UserInvalidEmail",
                email: "not-an-email",
                password: "StrongPassword123!",
                role: "user"
            }, { headers: { 'X-API-Key': API_KEY } });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(422);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: expect.stringMatching(/Validation Error|Bad Request/i)
            }));
        }
    });

    // 3. Missing required fields (Password missing)
    test('Scenario 3: Missing required fields', async () => {
        try {
            await axios.post(`${BASE_URL}/auth/register`, {
                username: "userMissingPass",
                email: "missingpass@mail.com"
                // Missing password
            }, { headers: { 'X-API-Key': API_KEY } });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(400);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: expect.stringMatching(/Missing required fields|Validation Error|Bad Request/i)
            }));
        }
    });

    // 4. Weak password
    test('Scenario 4: Weak password', async () => {
        try {
            await axios.post(`${BASE_URL}/auth/register`, {
                username: "UserWeakPass",
                email: "weakpass@example.com",
                password: "123",
                role: "user"
            }, { headers: { 'X-API-Key': API_KEY } });
            throw new Error('Should have failed');
        } catch (error) {
            expect([400, 422]).toContain(error.response.status);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: expect.stringMatching(/Validation Error|Bad Request/i)
            }));
        }
    });

    // 5. Validation: Username with invalid chars
    test('Scenario 5: Validation: Username with invalid chars', async () => {
        try {
            await axios.post(`${BASE_URL}/auth/register`, {
                username: "User Name", // Space
                email: "spaceuser@example.com",
                password: "StrongPassword123!",
                role: "user"
            }, { headers: { 'X-API-Key': API_KEY } });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(422);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: expect.stringMatching(/Validation Error/i)
            }));
        }
    });

    // 6. Validation: Username using reserved words
    test('Scenario 6: Validation: Username using reserved words', async () => {
        try {
            await axios.post(`${BASE_URL}/auth/register`, {
                username: "admin",
                email: `val_reserved_${Date.now()}@test.com`,
                password: "StrongPassword123!",
                role: "user"
            }, { headers: { 'X-API-Key': API_KEY } });
            throw new Error('Should have failed');
        } catch (error) {
            expect([400, 409]).toContain(error.response.status);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: expect.stringMatching(/Validation Error|Reserved Username/i)
            }));
        }
    });

    // 7. Validation: Invalid Role
    test('Scenario 7: Validation: Invalid Role', async () => {
        try {
            await axios.post(`${BASE_URL}/auth/register`, {
                username: `val_role_${Date.now()}`,
                email: `val_role_${Date.now()}@test.com`,
                password: "StrongPassword123!",
                role: "GOD_MODE"
            }, { headers: { 'X-API-Key': API_KEY } });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(400); // Contract says 400
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: "Bad Request"
            }));
        }
    });

    // 8. Validation: Password too long
    test('Scenario 8: Validation: Password too long', async () => {
        try {
            await axios.post(`${BASE_URL}/auth/register`, {
                username: `val_long_${Date.now()}`,
                email: `val_long_${Date.now()}@test.com`,
                password: "a".repeat(129),
                role: "user"
            }, { headers: { 'X-API-Key': API_KEY } });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(422);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: "Validation Error",
                details: expect.arrayContaining([
                    expect.objectContaining({
                        field: "password",
                        message: "Password too long"
                    })
                ])
            }));
        }
    });

    // 9. Successful registration
    // Pre-condition: Tenant must exist (X-API-Key). Contract does not ask to pass tenant_id.
    test('Scenario 9: Successful registration', async () => {
        uniqueEmail = `success_${Date.now()}@test.com`;
        const uniqueUser = {
            username: `success_${Date.now()}`,
            email: uniqueEmail,
            password: "StrongPassword123!",
            role: "user"
        };

        const response = await axios.post(`${BASE_URL}/auth/register`, uniqueUser, { headers: { 'X-API-Key': API_KEY } });

        expect([200, 201]).toContain(response.status);
        expect(response.data.status).toBe(true);
        expect(response.data.message).toMatch(/User registered/i);
        expect(response.data.data).toHaveProperty("user_id");
    });

    // 10. Duplicate email
    test('Scenario 10: Duplicate email', async () => {
        // First register
        const user = { username: `dup_email_1_${Date.now()}`, email: `dup_${Date.now()}@test.com`, password: "Password123!", role: "user" };
        await axios.post(`${BASE_URL}/auth/register`, user, { headers: { 'X-API-Key': API_KEY } });

        // Register again same email
        try {
            await axios.post(`${BASE_URL}/auth/register`, { ...user, username: `diff_name_${Date.now()}` }, { headers: { 'X-API-Key': API_KEY } });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(409);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: expect.stringMatching(/Email already exists/i)
            }));
        }
    });

    // 11. Duplicate username
    test('Scenario 11: Duplicate username', async () => {
        // First register
        const user = { username: `dup_user_${Date.now()}`, email: `dup_user_1_${Date.now()}@test.com`, password: "Password123!", role: "user" };
        await axios.post(`${BASE_URL}/auth/register`, user, { headers: { 'X-API-Key': API_KEY } });

        // Register again same username
        try {
            await axios.post(`${BASE_URL}/auth/register`, { ...user, email: `diff_${Date.now()}@mail.com` }, { headers: { 'X-API-Key': API_KEY } });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(409);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: expect.stringMatching(/Username already exists/i)
            }));
        }
    });

    // 12. Edge Case: Email case sensitivity
    test('Scenario 12: Edge Case: Email case sensitivity', async () => {
        try {
            await axios.post(`${BASE_URL}/auth/register`, {
                username: "UserCaseSensitive",
                email: uniqueEmail.toUpperCase(), // Same email but upper case
                password: "StrongPassword123!",
                role: "user"
            }, { headers: { 'X-API-Key': API_KEY } });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(409);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: expect.stringMatching(/Email already exists/i)
            }));
        }
    });

    // 13. Validation: Invalid SSO State (Special Chars)
    test('Scenario 13: Validation: Invalid SSO State (Special Chars)', async () => {
        try {
            await axios.post(`${BASE_URL}/auth/register`, {
                username: `val_state_${Date.now()}`,
                email: `val_state_${Date.now()}@test.com`,
                password: "StrongPassword123!",
                role: "user",
                state: "invalid_state!"
            }, { headers: { 'X-API-Key': API_KEY } });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(422);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: expect.stringMatching(/State parameter must be alphanumeric/i)
            }));
        }
    });

    // 14. Validation: SSO Nonce Too Long
    test('Scenario 14: Validation: SSO Nonce Too Long', async () => {
        try {
            await axios.post(`${BASE_URL}/auth/register`, {
                username: `val_nonce_${Date.now()}`,
                email: `val_nonce_${Date.now()}@test.com`,
                password: "StrongPassword123!",
                role: "user",
                nonce: "a".repeat(129)
            }, { headers: { 'X-API-Key': API_KEY } });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(422);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: expect.stringMatching(/Nonce parameter too long/i)
            }));
        }
    });

    // 15. Validation: Invalid Redirect URI (Injection)
    test('Scenario 15: Validation: Invalid Redirect URI (Injection)', async () => {
        try {
            await axios.post(`${BASE_URL}/auth/register`, {
                username: `val_ure_${Date.now()}`,
                email: `val_ure_${Date.now()}@test.com`,
                password: "StrongPassword123!",
                role: "user",
                redirect_uri: "https://example.com/<script>"
            }, { headers: { 'X-API-Key': API_KEY } });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(422);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: expect.stringMatching(/Redirect URI contains invalid characters/i)
            }));
        }
    });

    // 16. Validation: Redirect URI not in allowed origins whitelist
    test('Scenario 16: Redirect URI not in allowed origins', async () => {
        try {
            await axios.post(`${BASE_URL}/auth/register`, {
                username: `whitelist_test_${Date.now()}`,
                email: `whitelist_test_${Date.now()}@test.com`,
                password: "StrongPassword123!",
                role: "user",
                redirect_uri: "https://evil-site.com/callback"
            }, { headers: { 'X-API-Key': API_KEY } });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(403);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: expect.stringMatching(/not in allowed origins|Forbidden/i)
            }));
        }
    });

});
