# ENDPOINT: GET /api/tenants

## Description
Retrieve all active tenants.

## Test Scenarios

### 1. Get tenants without JWT
- **URL**: `http://localhost:5500/api/tenants`
- **Method**: `GET`
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

### 2. Get tenants with invalid JWT
- **URL**: `http://localhost:5500/api/tenants`
- **Method**: `GET`
- **Pre-conditions**: Invalid JWT.
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

### 3. Get all tenants (active only)
- **URL**: `http://localhost:5500/api/tenants`
- **Method**: `GET`
- **Pre-conditions**:
  - Valid JWT.
- **Request Body**: None.
- **Expected Response**:
  ```json
  {
    "status": true,
    "message": "Tenants retrieved successfully",
    "data": {
      "tenants": [
        {
          "id": "uuid",
          "name": "string",
          "description": "string",
          "is_active": true
        }
      ],
      "pagination": {
        "page": 1,
        "limit": 10,
        "total": 1,
        "total_pages": 1
      }
    }
  }
  ```
  *(Status: 200)*
- **Side Effects**: None.

### 4. Verify inactive tenants are excluded
- **URL**: `http://localhost:5500/api/tenants`
- **Method**: `GET`
- **Pre-conditions**:
  - Tenant exists with `is_active = false`.
- **Request Body**: None.
- **Expected Response**:
  - List should NOT contain the inactive tenant.
  *(Status: 200)*
- **Side Effects**: None.
