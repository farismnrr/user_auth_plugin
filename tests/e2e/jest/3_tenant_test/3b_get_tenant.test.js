const axios = require("axios");
const { BASE_URL, API_KEY, TENANT_SECRET_KEY } = require("../config");

describe("GET /api/tenants/:id - Get Tenant by ID", () => {
  let authToken = "";
  let validTenantId = "";
  const testUser = {
    username: `gettenant_${Date.now()}`,
    email: `gettenant_${Date.now()}@example.com`,
    password: "StrongPassword123!",
    role: "user",
  };

  beforeAll(async () => {
    try {
      // 1. Create Tenant to get
      const tRes = await axios.post(
        `${BASE_URL}/api/tenants`,
        {
          name: "Tenant_Get_" + Date.now(),
          description: "Test Tenant for Get",
        },
        { headers: { "X-Tenant-Secret-Key": TENANT_SECRET_KEY } },
      );
      validTenantId = tRes.data.data?.tenant_id || tRes.data.result?.tenant_id;

      // 2. Setup user for JWT
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

  // 1. Get tenant without JWT
  test("Scenario 1: Get tenant without JWT", async () => {
    try {
      await axios.get(`${BASE_URL}/api/tenants/${validTenantId}`, {
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

  // 2. Get non-existent tenant
  test("Scenario 2: Get non-existent tenant", async () => {
    const fakeId = "00000000-0000-0000-0000-000000000000"; // Valid UUID format but likely non-existent
    try {
      await axios.get(`${BASE_URL}/api/tenants/${fakeId}`, {
        headers: {
          "X-API-Key": API_KEY,
          Authorization: `Bearer ${authToken}`,
        },
      });
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(404);
      if (error.response.data && error.response.data !== "") {
        expect(error.response.data).toEqual(
          expect.objectContaining({
            status: false,
            message: "Tenant not found",
          }),
        );
      }
    }
  });

  // 3. Get tenant with invalid ID format
  test("Scenario 3: Get tenant with invalid ID format", async () => {
    try {
      await axios.get(`${BASE_URL}/api/tenants/not-a-uuid`, {
        headers: {
          "X-API-Key": API_KEY,
          Authorization: `Bearer ${authToken}`,
        },
      });
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(400);
      if (error.response.data && error.response.data !== "") {
        expect(error.response.data).toEqual(
          expect.objectContaining({
            status: false,
            message: "Bad Request",
          }),
        );
      }
    }
  });

  // 4. Get existing tenant
  test("Scenario 4: Get existing tenant", async () => {
    const response = await axios.get(`${BASE_URL}/api/tenants/${validTenantId}`, {
      headers: {
        "X-API-Key": API_KEY,
        Authorization: `Bearer ${authToken}`,
      },
    });

    expect(response.status).toBe(200);
    expect(response.data.status).toBe(true);
    expect(response.data.message).toBe("Tenant retrieved successfully");
    expect(response.data.data).toHaveProperty("tenant");
    expect(response.data.data.tenant).toHaveProperty("id", validTenantId);
    expect(response.data.data.tenant).toHaveProperty("name");
    expect(response.data.data.tenant).toHaveProperty("is_active", true);
  });
});
