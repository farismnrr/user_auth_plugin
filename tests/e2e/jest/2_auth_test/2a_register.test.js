const axios = require("axios");
const { BASE_URL, API_KEY, TENANT_SECRET_KEY } = require("../config");

describe("POST /auth/register - Register User", () => {
  let uniqueEmail; // Shared for scenario 9 and 12

  // 1. Missing API key
  test("Scenario 1: Missing API key", async () => {
    try {
      await axios.post(`${BASE_URL}/auth/register`, {
        username: "UserKeyMissing",
        email: "keymissing@example.com",
        password: "StrongPassword123!",
        role: "user",
      }); // No Headers
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

  // 2. Invalid email format
  test("Scenario 2: Invalid email format", async () => {
    try {
      await axios.post(
        `${BASE_URL}/auth/register`,
        {
          username: "UserInvalidEmail",
          email: "not-an-email",
          password: "StrongPassword123!",
          role: "user",
        },
        { headers: { "X-API-Key": API_KEY } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(422);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "Invalid email format",
        }),
      );
    }
  });

  // 3. Missing required fields (Password missing)
  test("Scenario 3: Missing required fields", async () => {
    try {
      await axios.post(
        `${BASE_URL}/auth/register`,
        {
          username: "userMissingPass",
          email: "missingpass@mail.com",
          // Missing password
        },
        { headers: { "X-API-Key": API_KEY } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(400);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "Missing required fields",
        }),
      );
    }
  });

  // 4. Weak password
  test("Scenario 4: Weak password", async () => {
    try {
      await axios.post(
        `${BASE_URL}/auth/register`,
        {
          username: "UserWeakPass",
          email: "weakpass@example.com",
          password: "123",
          role: "user",
        },
        { headers: { "X-API-Key": API_KEY } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect([400, 422]).toContain(error.response.status);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "Password too weak",
        }),
      );
    }
  });

  // 5. Validation: Username with invalid chars
  test("Scenario 5: Validation: Username with invalid chars", async () => {
    try {
      await axios.post(
        `${BASE_URL}/auth/register`,
        {
          username: "User Name", // Space
          email: "spaceuser@example.com",
          password: "StrongPassword123!",
          role: "user",
        },
        { headers: { "X-API-Key": API_KEY } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(422);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "Invalid characters",
        }),
      );
    }
  });

  // 6. Validation: Username using reserved words
  test("Scenario 6: Validation: Username using reserved words", async () => {
    try {
      await axios.post(
        `${BASE_URL}/auth/register`,
        {
          username: "admin",
          email: `val_reserved_${Date.now()}@test.com`,
          password: "StrongPassword123!",
          role: "user",
        },
        { headers: { "X-API-Key": API_KEY } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect([400, 409]).toContain(error.response.status);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "Reserved Username",
        }),
      );
    }
  });

  // 7. Validation: Invalid Role
  test("Scenario 7: Validation: Invalid Role", async () => {
    try {
      await axios.post(
        `${BASE_URL}/auth/register`,
        {
          username: `val_role_${Date.now()}`,
          email: `val_role_${Date.now()}@test.com`,
          password: "StrongPassword123!",
          role: "GOD_MODE",
        },
        { headers: { "X-API-Key": API_KEY } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(400); // Contract says 400
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "Bad Request",
        }),
      );
    }
  });

  // 8. Validation: Password too long
  test("Scenario 8: Validation: Password too long", async () => {
    try {
      await axios.post(
        `${BASE_URL}/auth/register`,
        {
          username: `val_long_${Date.now()}`,
          email: `val_long_${Date.now()}@test.com`,
          password: "a".repeat(129),
          role: "user",
        },
        { headers: { "X-API-Key": API_KEY } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(422);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "Password too long",
          details: expect.arrayContaining([
            expect.objectContaining({
              field: "password",
              message: "Password too long",
            }),
          ]),
        }),
      );
    }
  });

  // 9. Successful registration
  test("Scenario 9: Successful registration", async () => {
    uniqueEmail = `success_${Date.now()}@test.com`;
    const uniqueUser = {
      username: `success_${Date.now()}`,
      email: uniqueEmail,
      password: "StrongPassword123!",
      role: "user",
    };

    const response = await axios.post(`${BASE_URL}/auth/register`, uniqueUser, {
      headers: { "X-API-Key": API_KEY },
    });

    expect([200, 201]).toContain(response.status);
    expect(response.data.status).toBe(true);
    expect(response.data.message).toMatch(/User registered/i);
    expect(response.data.data).toHaveProperty("user_id");
  });

  // 10. Duplicate email
  test("Scenario 10: Duplicate email", async () => {
    const user = {
      username: `dup_email_1_${Date.now()}`,
      email: `dup_${Date.now()}@test.com`,
      password: "Password123!",
      role: "user",
    };
    await axios.post(`${BASE_URL}/auth/register`, user, { headers: { "X-API-Key": API_KEY } });

    try {
      await axios.post(
        `${BASE_URL}/auth/register`,
        { ...user, username: `diff_name_${Date.now()}` },
        { headers: { "X-API-Key": API_KEY } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(409);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "Email already exists",
        }),
      );
    }
  });

  // 11. Duplicate username
  test("Scenario 11: Duplicate username", async () => {
    const user = {
      username: `dup_user_${Date.now()}`,
      email: `dup_user_1_${Date.now()}@test.com`,
      password: "Password123!",
      role: "user",
    };
    await axios.post(`${BASE_URL}/auth/register`, user, { headers: { "X-API-Key": API_KEY } });

    try {
      await axios.post(
        `${BASE_URL}/auth/register`,
        { ...user, email: `diff_${Date.now()}@mail.com` },
        { headers: { "X-API-Key": API_KEY } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(409);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "Username already exists",
        }),
      );
    }
  });

  // 12. Edge Case: Email case sensitivity
  test("Scenario 12: Edge Case: Email case sensitivity", async () => {
    try {
      await axios.post(
        `${BASE_URL}/auth/register`,
        {
          username: "UserCaseSensitive",
          email: uniqueEmail.toUpperCase(),
          password: "StrongPassword123!",
          role: "user",
        },
        { headers: { "X-API-Key": API_KEY } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(409);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "Email already exists",
        }),
      );
    }
  });

  // 13. Validation: Invalid SSO State (Special Chars)
  test("Scenario 13: Validation: Invalid SSO State (Special Chars)", async () => {
    try {
      await axios.post(
        `${BASE_URL}/auth/register`,
        {
          username: `val_state_${Date.now()}`,
          email: `val_state_${Date.now()}@test.com`,
          password: "StrongPassword123!",
          role: "user",
          state: "invalid_state!",
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

  // 14. Validation: SSO Nonce Too Long
  test("Scenario 14: Validation: SSO Nonce Too Long", async () => {
    try {
      await axios.post(
        `${BASE_URL}/auth/register`,
        {
          username: `val_nonce_${Date.now()}`,
          email: `val_nonce_${Date.now()}@test.com`,
          password: "StrongPassword123!",
          role: "user",
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

  // 15. Validation: Invalid Redirect URI (Injection)
  test("Scenario 15: Validation: Invalid Redirect URI (Injection)", async () => {
    try {
      await axios.post(
        `${BASE_URL}/auth/register`,
        {
          username: `val_ure_${Date.now()}`,
          email: `val_ure_${Date.now()}@test.com`,
          password: "StrongPassword123!",
          role: "user",
          redirect_uri: "https://example.com/<script>",
        },
        { headers: { "X-API-Key": API_KEY } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(422);
      expect(error.response.data).toEqual(
        expect.objectContaining({
          status: false,
          message: "Redirect URI contains invalid characters",
        }),
      );
    }
  });

  // 16. Validation: Redirect URI not in allowed origins
  test("Scenario 16: Redirect URI not in allowed origins", async () => {
    try {
      await axios.post(
        `${BASE_URL}/auth/register`,
        {
          username: `whitelist_test_${Date.now()}`,
          email: `whitelist_test_${Date.now()}@test.com`,
          password: "StrongPassword123!",
          role: "user",
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

  // 17. User Role Multi-Tenant SSO (Success)
  test("Scenario 17: User Role Multi-Tenant SSO (Success)", async () => {
    const tenantBName = `TenantB_${Date.now()}`;
    const tenantRes = await axios.post(
      `${BASE_URL}/api/tenants`,
      { name: tenantBName },
      { headers: { "X-Tenant-Secret-Key": TENANT_SECRET_KEY } },
    );
    const API_KEY_B = tenantRes.data.data.api_key;

    const email = `sso_user_${Date.now()}@test.com`;
    const username = `sso_user_${Date.now()}`;
    const userA = { username, email, password: "Password123!", role: "user" };
    const resA = await axios.post(`${BASE_URL}/auth/register`, userA, {
      headers: { "X-API-Key": API_KEY },
    });
    const userIdA = resA.data.data.user_id;

    const resB = await axios.post(`${BASE_URL}/auth/register`, userA, {
      headers: { "X-API-Key": API_KEY_B },
    });

    expect(resB.status).toBe(201);
    expect(resB.data.data.user_id).toBe(userIdA);
  });

  // 18. Admin Role Cannot Share Credentials (Conflict)
  test("Scenario 18: Admin Role Cannot Share Credentials (Conflict)", async () => {
    const tenantCName = `TenantC_${Date.now()}`;
    const tenantRes = await axios.post(
      `${BASE_URL}/api/tenants`,
      { name: tenantCName },
      { headers: { "X-Tenant-Secret-Key": TENANT_SECRET_KEY } },
    );
    const API_KEY_C = tenantRes.data.data.api_key;

    const inviteRes = await axios.post(
      `${BASE_URL}/auth/internal/invitations`,
      {},
      { headers: { "X-Tenant-Secret-Key": TENANT_SECRET_KEY } },
    );
    const code = typeof inviteRes.data === "string" ? inviteRes.data : inviteRes.data.code;

    const email = `sso_admin_${Date.now()}@test.com`;
    const username = `sso_admin_${Date.now()}`;
    const adminA = { username, email, password: "Password123!", role: "admin", invitation_code: code };
    await axios.post(`${BASE_URL}/auth/register`, adminA, { headers: { "X-API-Key": API_KEY } });

    const inviteResC = await axios.post(
      `${BASE_URL}/auth/internal/invitations`,
      {},
      { headers: { "X-Tenant-Secret-Key": TENANT_SECRET_KEY } },
    );
    const codeC = typeof inviteResC.data === "string" ? inviteResC.data : inviteResC.data.code;

    try {
      await axios.post(
        `${BASE_URL}/auth/register`,
        { ...adminA, invitation_code: codeC },
        { headers: { "X-API-Key": API_KEY_C } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(409);
      expect(error.response.data.message).toBe("Email already exists");
    }
  });

  // 19. Cannot Mix User and Admin Roles (Conflict)
  test("Scenario 19: Cannot Mix User and Admin Roles (Conflict)", async () => {
    const tenantDName = `TenantD_${Date.now()}`;
    const tenantRes = await axios.post(
      `${BASE_URL}/api/tenants`,
      { name: tenantDName },
      { headers: { "X-Tenant-Secret-Key": TENANT_SECRET_KEY } },
    );
    const API_KEY_D = tenantRes.data.data.api_key;

    const email = `mix_role_${Date.now()}@test.com`;
    const username = `mix_role_${Date.now()}`;
    const userA = { username, email, password: "Password123!", role: "user" };
    await axios.post(`${BASE_URL}/auth/register`, userA, { headers: { "X-API-Key": API_KEY } });

    const inviteRes = await axios.post(
      `${BASE_URL}/auth/internal/invitations`,
      {},
      { headers: { "X-Tenant-Secret-Key": TENANT_SECRET_KEY } },
    );
    const code = typeof inviteRes.data === "string" ? inviteRes.data : inviteRes.data.code;

    try {
      await axios.post(
        `${BASE_URL}/auth/register`,
        { username, email, password: "Password123!", role: "admin", invitation_code: code },
        { headers: { "X-API-Key": API_KEY_D } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(409);
      expect(error.response.data.message).toBe(
        "Cannot register as user - account exists with admin/non-user role",
      );
    }
  });
});
