# ENDPOINT: PUT /api/tenants/{id}

## Description
Update an existing tenant.

## Test Scenarios

### 1. Update without JWT
- **URL**: `http://localhost:5500/api/tenants/<tenant_id>`
- **Method**: `PUT`
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

### 2. Update non-existent tenant
- **URL**: `http://localhost:5500/api/tenants/<fake_id>`
- **Method**: `PUT`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  { "name": "New Name" }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Tenant not found"
  }
  ```
  *(Status: 404)*
- **Side Effects**: None.

### 3. Update tenant with valid data
- **URL**: `http://localhost:5500/api/tenants/<tenant_id>`
- **Method**: `PUT`
- **Pre-conditions**:
  - Tenant exists.
  - Valid JWT.
- **Request Body**:
  ```json
  {
    "name": "Updated Name",
    "description": "Updated Desc",
    "is_active": true
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": true,
    "message": "Tenant updated successfully"
  }
  ```
  *(Status: 200)*
- **Side Effects**:
  - Tenant record updated.
  - `updated_at` timestamp changed.

### 4. Partial Update (Description only)
- **URL**: `http://localhost:5500/api/tenants/<tenant_id>`
- **Method**: `PUT`
- **Pre-conditions**: Tenant exists.
- **Request Body**:
  ```json
  {
    "description": "Only description updated"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": true,
    "message": "Tenant updated successfully"
  }
  ```
  *(Status: 200)*
- **Side Effects**:
  - Tenant description updated.

### 5. Update with duplicate name
- **URL**: `http://localhost:5500/api/tenants/<tenant_id>`
- **Method**: `PUT`
- **Pre-conditions**:
  - Another tenant exists with target name.
- **Request Body**:
  ```json
  {
    "name": "<existing_other_name>"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Tenant name already exists"
  }
  ```
  *(Status: 409)*
- **Side Effects**: None.
