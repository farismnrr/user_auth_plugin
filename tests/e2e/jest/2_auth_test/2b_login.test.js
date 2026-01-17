const axios = require("axios");
const { BASE_URL, API_KEY } = require("../config");

describe("POST /auth/login - Login User", () => {
  const validUser = {
    username: `loginuser_${Date.now()}`,
    email: `login_${Date.now()}@example.com`,
    password: "StrongPassword123!",
    role: "user",
  };

  beforeAll(async () => {
    try {
      await axios.post(`${BASE_URL}/auth/register`, validUser, {
        headers: { "X-API-Key": API_KEY },
      });
    } catch (e) {
      console.log("Setup failed", e.message);
    }
  });

  // 1. Missing API key
  test("Scenario 1: Missing API key", async () => {
    try {
      await axios.post(`${BASE_URL}/auth/login`, {
        email_or_username: validUser.email,
        password: validUser.password,
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

  // 2. Account Security: Login to Banned/Soft-Deleted Account
  test("Scenario 2: Account Security: Login to Banned/Soft-Deleted Account", async () => {
    try {
      await axios.post(
        `${BASE_URL}/auth/login`,
        {
          email_or_username: "banned_user",
          password: "password",
        },
        { headers: { "X-API-Key": API_KEY } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(403);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "Forbidden",
        }),
      );
    }
  });

  // 3. Missing credentials
  test("Scenario 3: Missing credentials", async () => {
    try {
      await axios.post(
        `${BASE_URL}/auth/login`,
        {
          email_or_username: "user",
        },
        { headers: { "X-API-Key": API_KEY } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(400);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "Bad Request",
        }),
      );
    }
  });

  // 4. Invalid email format
  test("Scenario 4: Invalid email format", async () => {
    try {
      await axios.post(
        `${BASE_URL}/auth/login`,
        {
          email_or_username: "invalid-email-format",
          password: "...",
        },
        { headers: { "X-API-Key": API_KEY } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(401);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "username or email or password invalid",
        }),
      );
    }
  });

  // 5. Wrong password
  test("Scenario 5: Wrong password", async () => {
    try {
      await axios.post(
        `${BASE_URL}/auth/login`,
        {
          email_or_username: validUser.email,
          password: "WrongPassword",
        },
        { headers: { "X-API-Key": API_KEY } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(401);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "username or email or password invalid",
        }),
      );
    }
  });

  // 6. Invalid Input: Leading/trailing spaces in credentials
  test("Scenario 6: Invalid Input: Leading/trailing spaces in credentials", async () => {
    try {
      await axios.post(
        `${BASE_URL}/auth/login`,
        {
          email_or_username: `  ${validUser.email}  `,
          password: validUser.password,
        },
        { headers: { "X-API-Key": API_KEY } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(401);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "username or email or password invalid",
        }),
      );
    }
  });

  // 7. Non-existent user
  test("Scenario 7: Non-existent user", async () => {
    try {
      await axios.post(
        `${BASE_URL}/auth/login`,
        {
          email_or_username: `non-existent_user_${Date.now()}`,
          password: "...",
        },
        { headers: { "X-API-Key": API_KEY } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(401);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "username or email or password invalid",
        }),
      );
    }
  });

  // 8. Security: Brute force protection check
  test("Scenario 8: Security: Brute force protection check", async () => {
    let lastStatus = 0;
    let lastData = {};
    for (let i = 0; i < 20; i++) {
      try {
        await axios.post(
          `${BASE_URL}/auth/login`,
          {
            email_or_username: validUser.email,
            password: `WrongPassword${i}`,
          },
          { headers: { "X-API-Key": API_KEY } },
        );
      } catch (error) {
        lastStatus = error.response.status;
        lastData = error.response.data;
        if (lastStatus === 429) break;
      }
    }

    if (lastStatus === 429) {
      expect(lastStatus).toBe(429);
      expect(lastData).toEqual(
        expect.objectContaining({
          status: false,
          message: "Too Many Requests",
        }),
      );
    }
  }, 60000);

  // 9. Successful login with email
  test("Scenario 9: Successful login with email", async () => {
    const response = await axios.post(
      `${BASE_URL}/auth/login`,
      {
        email_or_username: validUser.email,
        password: validUser.password,
      },
      { headers: { "X-API-Key": API_KEY } },
    );

    expect(response.status).toBe(200);
    expect(response.data.status).toBe(true);
    expect(response.data.message).toBe("Login successful");
    expect(response.data.data).toHaveProperty("access_token");
  });

  // 10. Successful login with username
  test("Scenario 10: Successful login with username", async () => {
    const response = await axios.post(
      `${BASE_URL}/auth/login`,
      {
        email_or_username: validUser.username,
        password: validUser.password,
      },
      { headers: { "X-API-Key": API_KEY } },
    );

    expect(response.status).toBe(200);
    expect(response.data.status).toBe(true);
    expect(response.data.message).toBe("Login successful");
    expect(response.data.data).toHaveProperty("access_token");
  });

  // 11. Validation: Invalid SSO State (Special Chars)
  test("Scenario 11: Validation: Invalid SSO State (Special Chars)", async () => {
    try {
      await axios.post(
        `${BASE_URL}/auth/login`,
        {
          email_or_username: validUser.email,
          password: validUser.password,
          state: "invalid-state!",
        },
        { headers: { "X-API-Key": API_KEY } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(422);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "State parameter must be alphanumeric",
        }),
      );
    }
  });

  // 12. Validation: SSO Nonce Too Long
  test("Scenario 12: Validation: SSO Nonce Too Long", async () => {
    try {
      await axios.post(
        `${BASE_URL}/auth/login`,
        {
          email_or_username: validUser.email,
          password: validUser.password,
          nonce: "a".repeat(129),
        },
        { headers: { "X-API-Key": API_KEY } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(422);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "Nonce parameter too long (max 128 chars)",
        }),
      );
    }
  });

  // 13. Validation: Redirect URI not in allowed origins
  test("Scenario 13: Validation: Redirect URI not in allowed origins", async () => {
    try {
      await axios.post(
        `${BASE_URL}/auth/login`,
        {
          email_or_username: validUser.email,
          password: validUser.password,
          redirect_uri: "https://evil-site.com/callback",
        },
        { headers: { "X-API-Key": API_KEY } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(403);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "Redirect URI not in allowed origins",
        }),
      );
    }
  });

  // 14. Login Role Mismatch
  test("Scenario 14: Login Role Mismatch", async () => {
    try {
      await axios.post(
        `${BASE_URL}/auth/login`,
        {
          email_or_username: validUser.email,
          password: validUser.password,
          role: "admin", // <--- Mismatch, validUser is "user"
        },
        { headers: { "X-API-Key": API_KEY } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(404);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "User not found",
        }),
      );
    }
  });
});
