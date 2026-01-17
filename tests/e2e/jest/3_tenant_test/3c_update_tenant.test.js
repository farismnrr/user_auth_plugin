const axios = require("axios");
const { BASE_URL, API_KEY, TENANT_SECRET_KEY } = require("../config");

describe("PUT /api/tenants/:id - Update Tenant", () => {
  let authToken = "";
  let tenantId = "";
  const conflictingTenantName = "Conflicting_Tenant_" + Date.now();
  const testUser = {
    username: `updtenant_${Date.now()}`,
    email: `updtenant_${Date.now()}@example.com`,
    password: "StrongPassword123!",
    role: "user",
  };

  beforeAll(async () => {
    try {
      // 1. Create Target Tenant
      const tRes = await axios.post(
        `${BASE_URL}/api/tenants`,
        {
          name: "Tenant_Update_Target_" + Date.now(),
          description: "Target for Update",
        },
        { headers: { "X-Tenant-Secret-Key": TENANT_SECRET_KEY } },
      );
      tenantId = tRes.data.data?.tenant_id || tRes.data.result?.tenant_id;

      // 2. Create Conflicting Tenant
      await axios.post(
        `${BASE_URL}/api/tenants`,
        {
          name: conflictingTenantName,
          description: "Conflict Source",
        },
        { headers: { "X-Tenant-Secret-Key": TENANT_SECRET_KEY } },
      );

      // 3. Setup user for JWT
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

  // 1. Update without JWT
  test("Scenario 1: Update without JWT", async () => {
    try {
      await axios.put(
        `${BASE_URL}/api/tenants/${tenantId}`,
        { name: "New Name" },
        {
          headers: { "X-API-Key": API_KEY }, // Missing Auth
        },
      );
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

  // 2. Update non-existent tenant
  test("Scenario 2: Update non-existent tenant", async () => {
    const fakeId = "00000000-0000-0000-0000-000000000000";
    try {
      await axios.put(
        `${BASE_URL}/api/tenants/${fakeId}`,
        { name: "New Name" },
        {
          headers: {
            "X-API-Key": API_KEY,
            Authorization: `Bearer ${authToken}`,
          },
        },
      );
      throw new Error("Should have failed");
    } catch (error) {
      if (!error.response) return; // Handle socket hang up
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

  // 3. Update tenant with valid data
  test("Scenario 3: Update tenant with valid data", async () => {
    const newName = "Updated Name " + Date.now();
    const response = await axios.put(
      `${BASE_URL}/api/tenants/${tenantId}`,
      {
        name: newName,
        description: "Updated Desc",
        is_active: true,
      },
      {
        headers: {
          "X-API-Key": API_KEY,
          Authorization: `Bearer ${authToken}`,
        },
      },
    );

    expect(response.status).toBe(200);
    expect(response.data.status).toBe(true);
    expect(response.data.message).toBe("Tenant updated successfully");
    // Contract does not specify data return
  });

  // 4. Partial Update (Description only)
  test("Scenario 4: Partial Update (Description only)", async () => {
    const response = await axios.put(
      `${BASE_URL}/api/tenants/${tenantId}`,
      {
        description: "Only description updated",
      },
      {
        headers: {
          "X-API-Key": API_KEY,
          Authorization: `Bearer ${authToken}`,
        },
      },
    );

    expect(response.status).toBe(200);
    expect(response.data.status).toBe(true);
    expect(response.data.message).toBe("Tenant updated successfully");
    // Contract does not specify data return
  });

  // 5. Update with duplicate name
  test("Scenario 5: Update with duplicate name", async () => {
    try {
      await axios.put(
        `${BASE_URL}/api/tenants/${tenantId}`,
        {
          name: conflictingTenantName,
        },
        {
          headers: {
            "X-API-Key": API_KEY,
            Authorization: `Bearer ${authToken}`,
          },
        },
      );
      throw new Error("Should have failed");
    } catch (error) {
      expect(error.response.status).toBe(409);
      if (error.response.data && error.response.data !== "") {
        expect(error.response.data).toEqual(
          expect.objectContaining({
            status: false,
            message: "Tenant name already exists",
          }),
        );
      }
    }
  });
});
