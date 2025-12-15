# ENDPOINT: DELETE /api/tenants/{id}

## Description
Soft delete a tenant (set `is_active` to false or separate deleted flag).

## Test Scenarios

### 1. Delete without JWT
- **URL**: `http://localhost:5500/api/tenants/<tenant_id>`
- **Method**: `DELETE`
- **Pre-conditions**: None.
- **Request Body**: None.
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Unauthorized"
  }
  ```
  *(Status: 401)*
- **Side Effects**: None.

### 2. Delete non-existent tenant
- **URL**: `http://localhost:5500/api/tenants/<fake_id>`
- **Method**: `DELETE`
- **Pre-conditions**: None.
- **Request Body**: None.
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Tenant not found"
  }
  ```
  *(Status: 404)*
- **Side Effects**: None.

### 3. Delete already deleted tenant
- **URL**: `http://localhost:5500/api/tenants/<deleted_id>`
- **Method**: `DELETE`
- **Pre-conditions**: Tenant already deleted.
- **Request Body**: None.
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Tenant not found"
  }
  ```
  *(Status: 404)*
- **Side Effects**: None.

### 4. Soft delete tenant
- **URL**: `http://localhost:5500/api/tenants/<tenant_id>`
- **Method**: `DELETE`
- **Pre-conditions**:
  - Tenant exists.
  - Valid JWT.
- **Request Body**: None.
- **Expected Response**:
  ```json
  {
    "status": true,
    "message": "Tenant deleted successfully"
  }
  ```
  *(Status: 200 or 204 No Content)*
- **Side Effects**:
  - `deleted_at` set or `is_active` becomes false.
  - Tenant removed from GET /tenants list.

### 5. Verify deletion (Get by ID)
- **URL**: `http://localhost:5500/api/tenants/<tenant_id>`
- **Method**: `GET`
- **Pre-conditions**: Tenant was just deleted.
- **Request Body**: None.
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Tenant not found"
  }
  ```
  *(Status: 404)*
- **Side Effects**: None.

### 6. Re-create tenant (Data Restoration)
- **URL**: `http://localhost:5500/api/tenants`
- **Method**: `POST`
- **Pre-conditions**: Tenant was deleted.
- **Request Body**:
  ```json
  {
    "name": "<original_name>",
    "description": "Restored Tenant"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": true,
    "message": "Tenant created successfully",
    "data": { "tenant_id": "uuid" }
  }
  ```
  *(Status: 201)*
- **Side Effects**:
  - Tenant recreated (new ID) to allow subsequent tests to pass.
