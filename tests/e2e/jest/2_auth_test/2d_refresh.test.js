const axios = require("axios");
const jwt = require("jsonwebtoken");
const { BASE_URL, API_KEY, JWT_SECRET } = require("../config");

describe("POST /auth/refresh - Refresh Token", () => {
  let refreshCookie = "";
  const testUser = {
    username: `refresh_${Date.now()}`,
    email: `refresh_${Date.now()}@example.com`,
    password: "StrongPassword123!",
    role: "user",
  };

  beforeAll(async () => {
    try {
      await axios.post(`${BASE_URL}/auth/register`, testUser, {
        headers: { "X-API-Key": API_KEY },
      });
      const loginRes = await axios.post(
        `${BASE_URL}/auth/login`,
        {
          email_or_username: testUser.email,
          password: testUser.password,
        },
        { headers: { "X-API-Key": API_KEY } },
      );

      const cookies = loginRes.headers["set-cookie"];
      if (cookies) {
        const rawCookie = cookies[0];
        refreshCookie = rawCookie.split(";")[0];
      }
    } catch (e) {
      console.log("Setup failed", e.message);
    }
  });

  // 1. Refresh without token cookie
  test("Scenario 1: Refresh without token cookie", async () => {
    try {
      await axios.post(`${BASE_URL}/auth/refresh`, {}, {
        headers: { "X-API-Key": API_KEY },
        // No Cookie
      });
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(401);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "Unauthorized",
        }),
      );
    }
  });

  // 2. Refresh with invalid token
  test("Scenario 2: Refresh with invalid token", async () => {
    try {
      await axios.post(`${BASE_URL}/auth/refresh`, {}, {
        headers: {
          "X-API-Key": API_KEY,
          Cookie: "refresh_token=invalid_jwt_token_string",
        },
      });
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(401);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "Unauthorized",
        }),
      );
    }
  });

  // 3. Refresh with expired token
  test("Scenario 3: Refresh with expired token", async () => {
    // Decode the valid cookie to get the payload structure
    const token = refreshCookie.split("=")[1];
    const decoded = jwt.decode(token);
    if (!decoded)
      throw new Error("Failed to decode existing refresh cookie to mock an expired one.");

    // Reconstruct payload and remove timing fields
    const payload = { ...decoded };
    delete payload.exp;
    delete payload.iat;

    // Sign a new token that is expired
    const expiredToken = jwt.sign({ ...payload, exp: Math.floor(Date.now() / 1000) - 7202 }, JWT_SECRET);

    try {
      await axios.post(`${BASE_URL}/auth/refresh`, {}, {
        headers: {
          "X-API-Key": API_KEY,
          Cookie: `refresh_token=${expiredToken}`,
        },
      });
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(401);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "Token expired",
        }),
      );
    }
  });

  // 4. Edge Case: Refresh token for different Tenant
  test("Scenario 4: Edge Case: Refresh token for different Tenant", async () => {
    // Simulate a token from a different tenant by signing with a different secret key.
    const token = refreshCookie.split("=")[1];
    const decoded = jwt.decode(token);
    if (!decoded) throw new Error("Failed to decode existing refresh cookie.");

    const payload = { ...decoded };
    delete payload.exp;
    delete payload.iat;

    const fakeTenantSecret = "DIFFERENT_OR_WRONG_SECRET_KEY";
    const forgedToken = jwt.sign({ ...payload, exp: Math.floor(Date.now() / 1000) + 3600 }, fakeTenantSecret);

    try {
      await axios.post(`${BASE_URL}/auth/refresh`, {}, {
        headers: {
          "X-API-Key": API_KEY,
          Cookie: `refresh_token=${forgedToken}`,
        },
      });
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(401);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "Unauthorized",
        }),
      );
    }
  });

  // 5. Security: Token Reuse Detection
  test("Scenario 5: Security: Token Reuse Detection", async () => {
    // Pre-condition: Token already consumed.
    // We use the valid `refreshCookie` ONCE in Scenario 7 (Success).
    // Then we use it AGAIN here?
    // Let's run Scenario 7 FIRST? No, strict order usually 1..7.
    // But Scenario 7 is "Successful". Scenario 5 expects "Unauthorized".
    // To test S5, we need to consume it first.

    // Let's rely on Scenario 7 logic to consume it, then run S5?
    // But Jest doesn't guarantee order unless in single file (it does run top to bottom).
    // So I will implement S5 AFTER S7 physically?
    // User said "strict scenarios". Ordering in file usually matches numbering.
    // I will implement helper loop or sequence inside S5 if needed.
    // Or I will just duplicate the consumption:

    /* 
           This test depends on S7 passing or consuming the token. 
           But if I execute it here, I consume it for S7!
           So S7 will fail if S5 consumes it first (and rotation is on).
           Conflict in ordering.
           Strategy: Create a NEW user/token specifically for S5.
        */
    let reuseCookie = "";
    try {
      // 1. Get new token
      const timestamp = Date.now();
      await axios.post(
        `${BASE_URL}/auth/register`,
        { ...testUser, username: "reuse_" + timestamp, email: "reuse_" + timestamp + "@x.com" },
        { headers: { "X-API-Key": API_KEY } },
      );
      const l = await axios.post(
        `${BASE_URL}/auth/login`,
        { email_or_username: "reuse_" + timestamp + "@x.com", password: testUser.password },
        { headers: { "X-API-Key": API_KEY } },
      );
      const reuseRawCookie = l.headers["set-cookie"][0];
      reuseCookie = reuseRawCookie.split(";")[0];

      // 2. Consume it once (Success)
      await axios.post(`${BASE_URL}/auth/refresh`, {}, {
        headers: { "X-API-Key": API_KEY, Cookie: reuseCookie },
      });

      // 3. Consume again (Reuse detection)
      await axios.post(`${BASE_URL}/auth/refresh`, {}, {
        headers: { "X-API-Key": API_KEY, Cookie: reuseCookie },
      });

      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(401);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "Unauthorized",
        }),
      );
    }
  });

  // 6. Security: User State Check
  test("Scenario 6: Security: User State Check", async () => {
    // User deleted/banned.
    // Create user, login, DELETE user (using 4g logic?), then refresh.
    // We don't have delete helper imported, but we can call API.
    try {
      const u = {
        username: "deleted_" + Date.now(),
        email: "deleted_" + Date.now() + "@x.com",
        password: "Password1!",
        role: "user",
      };
      await axios.post(`${BASE_URL}/auth/register`, u, { headers: { "X-API-Key": API_KEY } });
      const l = await axios.post(
        `${BASE_URL}/auth/login`,
        { email_or_username: u.email, password: u.password },
        { headers: { "X-API-Key": API_KEY } },
      );
      const rawC = l.headers["set-cookie"][0];
      const c = rawC.split(";")[0];
      const token = l.data.data?.access_token || l.data.result?.access_token;

      // Delete user
      await axios.delete(`${BASE_URL}/api/users`, {
        headers: { "X-API-Key": API_KEY, Authorization: `Bearer ${token}` },
      });

      // Try refresh
      await axios.post(`${BASE_URL}/auth/refresh`, {}, { headers: { "X-API-Key": API_KEY, Cookie: c } });

      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(401);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "Unauthorized",
        }),
      );
    }
  });

  // 7. Successful token refresh
  test("Scenario 7: Successful token refresh", async () => {
    const response = await axios.post(`${BASE_URL}/auth/refresh`, {}, {
      headers: {
        "X-API-Key": API_KEY,
        Cookie: refreshCookie,
      },
    });

    expect(response.status).toBe(200);
    expect(response.data.status).toBe(true);
    expect(response.data.message).toBe("Token refreshed successfully");
    expect(response.data.data).toHaveProperty("access_token");
  });
});
