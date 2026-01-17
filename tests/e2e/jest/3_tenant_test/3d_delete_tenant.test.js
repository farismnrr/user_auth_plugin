const axios = require("axios");
const { BASE_URL, API_KEY, TENANT_SECRET_KEY } = require("../config");

describe("DELETE /api/tenants/:id - Delete Tenant", () => {
  let authToken = "";
  let tenantId = "";
  const tenantName = "Tenant_Delete_" + Date.now();
  const testUser = {
    username: `deltenant_${Date.now()}`,
    email: `deltenant_${Date.now()}@example.com`,
    password: "StrongPassword123!",
    role: "user",
  };

  beforeAll(async () => {
    try {
      // 1. Create Tenant to delete
      const tRes = await axios.post(
        `${BASE_URL}/api/tenants`,
        {
          name: tenantName,
          description: "To be deleted",
        },
        { headers: { "X-Tenant-Secret-Key": TENANT_SECRET_KEY } },
      );
      tenantId = tRes.data.data?.tenant_id || tRes.data.result?.tenant_id;

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

  // 1. Delete without JWT
  test("Scenario 1: Delete without JWT", async () => {
    try {
      await axios.delete(`${BASE_URL}/api/tenants/${tenantId}`, {
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

  // 2. Delete non-existent tenant
  test("Scenario 2: Delete non-existent tenant", async () => {
    const fakeId = "00000000-0000-0000-0000-000000000000";
    try {
      await axios.delete(`${BASE_URL}/api/tenants/${fakeId}`, {
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

  // 3. Delete already deleted tenant
  // Must run AFTER Scenario 4. Order is key.
  // Or we handle it by creating another dummy tenant.
  // User requested strict adherence to file order?
  // Contract order: 1, 2, 3, 4. But 3 relies on condition "Tenant already deleted".
  // I can't delete it before 4 if 4 tests "Soft delete tenant" (meaning it was alive).
  // So 3 should physically run after 4? Or I create a separate tenant for 3.
  // Strategy: Create a separate tenant for Scenario 3.

  test("Scenario 3: Delete already deleted tenant", async () => {
    try {
      // Setup dedicated tenant for this check
      const tempName = "Already_Deleted_" + Date.now();
      const tRes = await axios.post(
        `${BASE_URL}/api/tenants`,
        { name: tempName },
        { headers: { "X-Tenant-Secret-Key": TENANT_SECRET_KEY } },
      );
      const tid = tRes.data.data?.tenant_id || tRes.data.result?.tenant_id;

      // Delete it once
      await axios.delete(`${BASE_URL}/api/tenants/${tid}`, {
        headers: { "X-API-Key": API_KEY, Authorization: `Bearer ${authToken}` },
      });

      // Delete it again (Test Target)
      await axios.delete(`${BASE_URL}/api/tenants/${tid}`, {
        headers: { "X-API-Key": API_KEY, Authorization: `Bearer ${authToken}` },
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

  // 4. Soft delete tenant
  test("Scenario 4: Soft delete tenant", async () => {
    const response = await axios.delete(`${BASE_URL}/api/tenants/${tenantId}`, {
      headers: {
        "X-API-Key": API_KEY,
        Authorization: `Bearer ${authToken}`,
      },
    });

    // 200 or 204
    expect([200, 204]).toContain(response.status);
    if (response.status === 200) {
      expect(response.data.status).toBe(true);
      expect(response.data.message).toBe("Tenant deleted successfully");
    }
  });

  // 5. Verify deletion (Get by ID)
  test("Scenario 5: Verify deletion (Get by ID)", async () => {
    try {
      await axios.get(`${BASE_URL}/api/tenants/${tenantId}`, {
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

  // 6. Re-create tenant (Data Restoration)
  test("Scenario 6: Re-create tenant (Data Restoration)", async () => {
    const response = await axios.post(
      `${BASE_URL}/api/tenants`,
      {
        name: tenantName, // Same name as deleted
        description: "Restored Tenant",
      },
      {
        headers: { "X-Tenant-Secret-Key": TENANT_SECRET_KEY },
      },
    );

    expect([200, 201]).toContain(response.status);
    expect(response.data.status).toBe(true);
    expect(response.data.message).toBe("Tenant created successfully");
    expect(response.data.data).toHaveProperty("tenant_id");
  });
});
