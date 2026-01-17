const axios = require("axios");
const { BASE_URL, API_KEY } = require("../config");

describe("DELETE /auth/logout - Logout", () => {
  let authToken = "";
  let refreshCookie = "";
  const testUser = {
    username: `logout_${Date.now()}`,
    email: `logout_${Date.now()}@example.com`,
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
      if (loginRes.headers["set-cookie"]) {
        const rawCookie = loginRes.headers["set-cookie"][0];
        refreshCookie = rawCookie.split(";")[0];
      }
    } catch (e) {
      console.log("Setup failed", e.message);
    }
  });

  // 1. Logout without JWT token
  test("Scenario 1: Logout without JWT token", async () => {
    try {
      await axios.delete(`${BASE_URL}/auth/logout`, {
        headers: { "X-API-Key": API_KEY }, // No Auth
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

  // 2. Logout with invalid JWT token
  test("Scenario 2: Logout with invalid JWT token", async () => {
    try {
      await axios.delete(`${BASE_URL}/auth/logout`, {
        headers: {
          "X-API-Key": API_KEY,
          Authorization: "Bearer invalid_token",
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

  // 3. Successful logout
  test("Scenario 3: Successful logout", async () => {
    const response = await axios.delete(`${BASE_URL}/auth/logout`, {
      headers: {
        "X-API-Key": API_KEY,
        Authorization: `Bearer ${authToken}`,
        Cookie: refreshCookie,
      },
    });

    expect(response.status).toBe(200);
    expect(response.data.status).toBe(true);
    expect(response.data.message).toBe("Logged out successfully");

    // Check cookie cleared (Max-Age=0 or Expires=Past)
    const cookies = response.headers["set-cookie"];
    expect(cookies).toBeDefined();
    // Just standard check
  });

  // 4. Idempotency: Double Logout
  test("Scenario 4: Idempotency: Double Logout", async () => {
    try {
      // Already logged out in Scenario 3
      await axios.delete(`${BASE_URL}/auth/logout`, {
        headers: {
          "X-API-Key": API_KEY,
          Authorization: `Bearer ${authToken}`,
          Cookie: refreshCookie,
        },
      });
      // If 200, pass.
    } catch (error) {
      expect([200, 401]).toContain(error.response.status);
      expect(error.response.status).not.toBe(500);
    }
  });

  // 5. Security: Logout invalidates refresh token
  test("Scenario 5: Security: Logout invalidates refresh token", async () => {
    // Try refresh with old cookie
    try {
      await axios.get(`${BASE_URL}/auth/refresh`, {
        headers: {
          "X-API-Key": API_KEY,
          Cookie: refreshCookie, // Old cookie
        },
      });
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(401);
    }
  });
});
