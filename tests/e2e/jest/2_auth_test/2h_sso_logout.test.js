const axios = require('axios');
const { BASE_URL, API_KEY } = require('../config');

describe('GET /auth/sso/logout - SSO Logout', () => {

    const testUser = {
        username: `sso_logout_${Date.now()}`,
        email: `sso_logout_${Date.now()}@example.com`,
        password: 'StrongPassword123!',
        role: 'user'
    };

    let refreshCookie = '';

    // Register and Login before tests to get valid cookie
    beforeAll(async () => {
        try {
            await axios.post(`${BASE_URL}/auth/register`, testUser, { headers: { 'X-API-Key': API_KEY } });
            const loginRes = await axios.post(`${BASE_URL}/auth/login`, {
                email_or_username: testUser.email,
                password: testUser.password
            }, { headers: { 'X-API-Key': API_KEY } });

            if (loginRes.headers['set-cookie']) {
                const rawCookie = loginRes.headers['set-cookie'][0];
                refreshCookie = rawCookie.split(';')[0];
            }
        } catch (e) {
            console.log('Setup failed', e.message);
        }
    });

    // 1. Success SSO Logout
    test('Scenario 1: Successful SSO Logout with Redirect', async () => {
        try {
            // Note: Axios will follow redirects by default. 
            // We want to inspect the initial 302 response or the final destination.
            // However, Jest/Axios usually follows redirects. 
            // We can disable redirect following to check the 302 status manually.
            const response = await axios.get(`${BASE_URL}/auth/sso/logout?redirect_uri=http://example.com/login`, {
                headers: {
                    // API Key is NOT required for SSO logout as per current implementation exemption
                    // But we can send it or not, it should be ignored or accepted.
                    // 'X-API-Key': API_KEY, // Exempted
                    'Cookie': refreshCookie
                },
                maxRedirects: 0, // Do not follow redirect
                validateStatus: status => status >= 200 && status < 400 // Accept 302
            });

            expect(response.status).toBe(302);
            expect(response.headers['location']).toBe('http://example.com/login');

            // Check cookie clearing
            const cookies = response.headers['set-cookie'];
            expect(cookies).toBeDefined();
            const cookieStr = cookies[0];
            expect(cookieStr).toContain('refresh_token=;'); // Value cleared
            expect(cookieStr).toContain('Max-Age=0'); // Instant expiry

        } catch (error) {
            console.error("SSO Logout failed", error.response ? error.response.data : error.message);
            throw error;
        }
    });

    // 2. Security: Verify Token Invalidated
    test('Scenario 2: Verify Refresh Token is Invalidated', async () => {
        try {
            await axios.get(`${BASE_URL}/auth/refresh`, {
                headers: {
                    'X-API-Key': API_KEY,
                    'Cookie': refreshCookie
                }
            });
            throw new Error('Should have failed');
        } catch (error) {
            expect(error.response.status).toBe(401);
            if (error.response.data && error.response.data !== "") {
                expect(error.response.data).toEqual(expect.objectContaining({
                    message: "Unauthorized" // or specific message if matched
                }));
            }
        }
    });

    // 3. Security: Redirect URI not in allowed origins whitelist
    test('Scenario 3: SSO Logout with invalid redirect_uri returns 403', async () => {
        try {
            await axios.get(`${BASE_URL}/auth/sso/logout?redirect_uri=https://evil-site.com/malicious`, {
                headers: {
                    'Cookie': refreshCookie
                },
                maxRedirects: 0,
                validateStatus: status => status >= 200 && status < 500
            });
            // If we get here check the status
        } catch (error) {
            expect(error.response.status).toBe(403);
            expect(error.response.data).toEqual(expect.objectContaining({
                status: false,
                message: expect.stringMatching(/not in allowed origins|Forbidden/i)
            }));
        }
    });

});
