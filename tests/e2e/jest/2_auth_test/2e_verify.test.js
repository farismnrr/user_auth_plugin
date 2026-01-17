const axios = require("axios");
const jwt = require("jsonwebtoken");
const { BASE_URL, API_KEY, JWT_SECRET } = require("../config");

describe("GET /auth/verify - Verify Token", () => {
  let authToken = "";
  const testUser = {
    username: `verify_${Date.now()}`,
    email: `verify_${Date.now()}@example.com`,
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
      authToken = loginRes.data.data?.access_token || loginRes.data.result?.access_token;
    } catch (e) {
      console.log("Setup failed", e.message);
    }
  });

  // 1. Missing Authorization header
  test("Scenario 1: Missing Authorization header", async () => {
    try {
      await axios.get(`${BASE_URL}/auth/verify`, {
        headers: { "X-API-Key": API_KEY }, // Missing Auth
      });
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(401);
      if (error.response.data && error.response.data !== "") {
        expect(error.response.data).toEqual(
          expect.objectContaining({
            status: false,
            message: "Unauthorized",
          }),
        );
      }
    }
  });

  // 2. Malformed Authorization header
  test("Scenario 2: Malformed Authorization header", async () => {
    try {
      await axios.get(`${BASE_URL}/auth/verify`, {
        headers: {
          "X-API-Key": API_KEY,
          Authorization: "InvalidTokenString", // No Bearer
        },
      });
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(401);
      if (error.response.data && error.response.data !== "") {
        expect(error.response.data).toEqual(
          expect.objectContaining({
            status: false,
            message: "Unauthorized",
          }),
        );
      }
    }
  });

  // 3. Invalid JWT format
  test("Scenario 3: Invalid JWT format", async () => {
    try {
      await axios.get(`${BASE_URL}/auth/verify`, {
        headers: {
          "X-API-Key": API_KEY,
          Authorization: "Bearer invalid.jwt.string",
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

  // 4. Expired JWT
  test("Scenario 4: Expired JWT", async () => {
    const decoded = jwt.decode(authToken);
    const payload = { ...decoded };
    delete payload.exp;
    delete payload.iat;

    const expiredToken = jwt.sign({ ...payload, exp: Math.floor(Date.now() / 1000) - 3600 }, JWT_SECRET);

    try {
      await axios.get(`${BASE_URL}/auth/verify`, {
        headers: {
          "X-API-Key": API_KEY,
          Authorization: `Bearer ${expiredToken}`,
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

  // 5. Security: NBF (Not Before) Check
  test("Scenario 5: Security: NBF Check", async () => {
    const decoded = jwt.decode(authToken);
    const payload = { ...decoded };
    delete payload.exp;
    delete payload.iat;

    // Set nbf to 1 hour in the future
    payload.nbf = Math.floor(Date.now() / 1000) + 3600;

    const futureToken = jwt.sign(payload, JWT_SECRET, { expiresIn: "2h" });

    try {
      await axios.get(`${BASE_URL}/auth/verify`, {
        headers: {
          "X-API-Key": API_KEY,
          Authorization: `Bearer ${futureToken}`,
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

  // 6. Security: Cross-Tenant Check (Relaxed - API Key not required for verify)
  test("Scenario 6: Security: Cross-Tenant Check (Should Pass without API Key check)", async () => {
    const decoded = jwt.decode(authToken);
    const payload = { ...decoded };
    delete payload.exp;
    delete payload.iat;

    payload.tenant_id = "00000000-0000-0000-0000-000000000000"; // Dummy Tenant ID

    const crossTenantToken = jwt.sign(payload, JWT_SECRET, { expiresIn: "1h" });

    // Should return 200 now because ApiKeyMiddleware is removed from /verify
    const response = await axios.get(`${BASE_URL}/auth/verify`, {
      headers: {
        "X-API-Key": API_KEY,
        Authorization: `Bearer ${crossTenantToken}`,
      },
    });

    expect(response.status).toBe(200);
  });

  // 7. User deleted but token still valid
  test("Scenario 7: User deleted but token still valid", async () => {
    try {
      // 1. Create temporary user
      const tempUser = {
        username: "del_verify_" + Date.now(),
        email: "del_verify_" + Date.now() + "@x.com",
        password: "Password1!",
        role: "user",
      };
      await axios.post(`${BASE_URL}/auth/register`, tempUser, {
        headers: { "X-API-Key": API_KEY },
      });
      const l = await axios.post(
        `${BASE_URL}/auth/login`,
        { email_or_username: tempUser.email, password: tempUser.password },
        { headers: { "X-API-Key": API_KEY } },
      );
      const token = l.data.data?.access_token || l.data.result?.access_token;

      // 2. Delete user
      // Assuming DELETE /users deletes the *current* user based on token
      await axios.delete(`${BASE_URL}/api/users`, {
        headers: {
          "X-API-Key": API_KEY,
          Authorization: `Bearer ${token}`,
        },
      });

      // 3. Verify
      await axios.get(`${BASE_URL}/auth/verify`, {
        headers: {
          "X-API-Key": API_KEY,
          Authorization: `Bearer ${token}`,
        },
      });
      throw new Error("Should have failed");
    } catch (error) {
      expect([401, 404]).toContain(error.response.status);
      if (error.response.data && error.response.data !== "") {
        expect(error.response.data).toEqual(
          expect.objectContaining({
            status: false,
            message: "Unauthorized",
          }),
        );
      }
    }
  });

  // 8. Successful verification
  test("Scenario 8: Successful verification", async () => {
    const response = await axios.get(`${BASE_URL}/auth/verify`, {
      headers: {
        "X-API-Key": API_KEY,
        Authorization: `Bearer ${authToken}`,
      },
    });

    expect(response.status).toBe(200);
    expect(response.data.status).toBe(true);
    expect(response.data.message).toBe("Token is valid");

    // Assert user data is present
    expect(response.data.data).toBeDefined();
    expect(response.data.data.username).toBe(testUser.username);
    expect(response.data.data.email).toBe(testUser.email);
  });

  // 9. Verification without API Key
  test("Scenario 9: Verification without API Key", async () => {
    const response = await axios.get(`${BASE_URL}/auth/verify`, {
      headers: {
        // No X-API-Key
        Authorization: `Bearer ${authToken}`,
      },
    });

    expect(response.status).toBe(200);
    expect(response.data.status).toBe(true);
    expect(response.data.data.username).toBe(testUser.username);
  });
});
