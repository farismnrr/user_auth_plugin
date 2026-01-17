const axios = require("axios");
const { BASE_URL, API_KEY, TENANT_SECRET_KEY } = require("../config");

describe("POST /auth/internal/invitations & Register with Code", () => {
  async function getInvitationCode() {
    const response = await axios.post(
      `${BASE_URL}/auth/internal/invitations`,
      {},
      {
        headers: { "X-Tenant-Secret-Key": TENANT_SECRET_KEY },
      },
    );
    // Assuming API returns raw string or { code: "..." }
    return typeof response.data === "string" ? response.data : response.data.code || response.data;
  }

  // 1. Generate Invitation Code
  test("Scenario 1: Generate Invitation Code", async () => {
    const response = await axios.post(
      `${BASE_URL}/auth/internal/invitations`,
      {},
      {
        headers: { "X-Tenant-Secret-Key": TENANT_SECRET_KEY },
      },
    );
    expect(response.status).toBe(200);
    const code =
      typeof response.data === "string" ? response.data : response.data.code || response.data;
    expect(code).toBeTruthy();
    expect(code.length).toBe(8);
  });

  // 2. Register Admin with Valid Code
  test("Scenario 2: Register Admin with Valid Code", async () => {
    const code = await getInvitationCode();

    const adminUser = {
      username: `valid_admin_${Date.now()}`,
      email: `valid_admin_${Date.now()}@test.com`,
      password: "Password123!",
      role: "admin",
      invitation_code: code,
    };

    const response = await axios.post(`${BASE_URL}/auth/register`, adminUser, {
      headers: { "X-API-Key": API_KEY },
    });

    expect([200, 201]).toContain(response.status);
    expect(response.data.status).toBe(true);
  });

  // 3. Register Admin with Invalid Code
  test("Scenario 3: Register Admin with Invalid Code", async () => {
    const adminUser = {
      username: `invalid_code_${Date.now()}`,
      email: `invalid_code_${Date.now()}@test.com`,
      password: "Password123!",
      role: "admin",
      invitation_code: "WRONGCODE",
    };

    try {
      await axios.post(`${BASE_URL}/auth/register`, adminUser, {
        headers: { "X-API-Key": API_KEY },
      });
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(403);
      expect(error.response.data.message).toMatch(/Invalid or missing invitation code/i);
    }
  });

  // 4. Register Admin with Missing Code
  test("Scenario 4: Register Admin with Missing Code", async () => {
    const adminUser = {
      username: `missing_code_${Date.now()}`,
      email: `missing_code_${Date.now()}@test.com`,
      password: "Password123!",
      role: "admin",
    };

    try {
      await axios.post(`${BASE_URL}/auth/register`, adminUser, {
        headers: { "X-API-Key": API_KEY },
      });
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(403);
      expect(error.response.data.message).toMatch(/Invalid or missing invitation code/i);
    }
  });

  // 5. Reuse Invitation Code (Should Fail)
  test("Scenario 5: Reuse Invitation Code", async () => {
    const code = await getInvitationCode();

    // Use once
    await axios.post(
      `${BASE_URL}/auth/register`,
      {
        username: `reuse_1_${Date.now()}`,
        email: `reuse_1_${Date.now()}@test.com`,
        password: "Password123!",
        role: "admin",
        invitation_code: code,
      },
      { headers: { "X-API-Key": API_KEY } },
    );

    // Try using again
    try {
      await axios.post(
        `${BASE_URL}/auth/register`,
        {
          username: `reuse_2_${Date.now()}`,
          email: `reuse_2_${Date.now()}@test.com`,
          password: "Password123!",
          role: "admin",
          invitation_code: code,
        },
        { headers: { "X-API-Key": API_KEY } },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(403); // Code deleted after use
    }
  });
});
