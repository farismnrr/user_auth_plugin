# ENDPOINT: POST /api/tenants

## Description
Create a new tenant using tenant secret key (bootstrapping).

## Test Scenarios

### 1. Create tenant without authentication (401 Unauthorized)
- **URL**: `http://localhost:5500/api/tenants`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  { "name": "<name>" }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Unauthorized"
  }
  ```
- **Side Effects**: None.

### 2. Create tenant with invalid secret key (401 Unauthorized)
- **URL**: `http://localhost:5500/api/tenants`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  { "name": "<name>" }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Unauthorized"
  }
  ```
- **Side Effects**: None.

### 3. Invalid Content-Type (415 Unsupported Media Type)
- **URL**: `http://localhost:5500/api/tenants`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  `name=Tenant`
- **Expected Response**:
  Status: 415 Unsupported Media Type
- **Side Effects**: None.

### 4. Malformed Request: Empty body or invalid JSON (400 Bad Request)
- **URL**: `http://localhost:5500/api/tenants`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  {}
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Bad Request"
  }
  ```
- **Side Effects**: None.

### 5. Create tenant with empty name (422)
- **URL**: `http://localhost:5500/api/tenants`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  { "name": "" }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Name cannot be empty",
    "details": [
      {
        "field": "name",
        "message": "Name cannot be empty"
      }
    ]
  }
  ```
- **Side Effects**: None.

### 6. Validation: Name too long (> 255 chars) (422)
- **URL**: `http://localhost:5500/api/tenants`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  { "name": "<256_chars_string>" }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Name too long",
    "details": [
      {
        "field": "name",
        "message": "Name too long"
      }
    ]
  }
  ```
- **Side Effects**: None.

### 7. Validation: Name too short / empty string (422)
- **URL**: `http://localhost:5500/api/tenants`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  { "name": "" }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Name cannot be empty",
    "details": [
      {
        "field": "name",
        "message": "Name cannot be empty"
      }
    ]
  }
  ```
- **Side Effects**: None.

### 8. Validation: Invalid characters in name (422/Sanitized)
- **URL**: `http://localhost:5500/api/tenants`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  { "name": "Tenant😳" }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Invalid characters in name",
    "details": [
      {
        "field": "name",
        "message": "Invalid characters in name"
      }
    ]
  }
  ```
  *(Or 201 if sanitized)*
- **Side Effects**: None (or sanitized creation).

### 9. Validation: SQL Injection payload in name (422/Sanitized)
- **URL**: `http://localhost:5500/api/tenants`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  { "name": "Tenant'; DROP TABLE tenants; --" }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Invalid characters in name",
    "details": [
      {
        "field": "name",
        "message": "Invalid characters in name"
      }
    ]
  }
  ```
  *(Or 201 if handled safely)*
- **Side Effects**: None.

### 10. Create tenant with tenant secret key (201 Created)
- **URL**: `http://localhost:5500/api/tenants`
- **Method**: `POST`
- **Pre-conditions**:
  - `TENANT_SECRET_KEY` env var configured.
- **Request Body**:
  ```json
  {
    "name": "<unique_name>",
    "description": "Test Tenant"
  }
  ```
- **Expected Response**:
  {
    "status": true,
    "message": "Tenant created successfully",
    "data": { 
      "tenant_id": "uuid",
      "api_key": "string"
    }
  }
  ```
- **Side Effects**:
  - New tenant inserted into DB.

### 11. Create tenant with duplicate name using secret key (200 OK - Idempotent)
- **URL**: `http://localhost:5500/api/tenants`
- **Method**: `POST`
- **Pre-conditions**:
  - Tenant with name already exists.
- **Request Body**:
  ```json
  {
    "name": "<existing_name>",
    "description": "Duplicate"
  }
  ```
- **Expected Response**:
  {
    "status": true,
    "message": "Tenant already exists",
    "data": { 
      "tenant_id": "uuid",
      "api_key": "string"
    }
  }
  ```
- **Side Effects**: None (Idempotent return).
