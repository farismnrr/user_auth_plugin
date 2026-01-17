# ENDPOINT: PUT /api/users

## Description
Update current user's core profile information.

## Test Scenarios

### 1. Update without JWT
- **URL**: `http://localhost:5500/api/users`
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

### 2. Update with invalid data (short username)
- **URL**: `http://localhost:5500/api/users`
- **Method**: `PUT`
- **Pre-conditions**: Valid JWT.
- **Request Body**:
  ```json
  { "username": "a" }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Username too short",
    "details": [
      {
        "field": "username",
        "message": "Username too short"
      }
    ]
  }
  ```
  *(Status: 400 or 422)*
- **Side Effects**: None.

### 3. Security: Privilege Escalation Attempt (Role Injection)
- **URL**: `http://localhost:5500/api/users`
- **Method**: `PUT`
- **Pre-conditions**: Valid JWT.
- **Request Body**:
  ```json
  {
    "username": "hacker",
    "role": "admin",
    "is_admin": true,
    "permissions": ["all"]
  }
  ```
- **Expected Response**:
  - The update should succeed for 'username' but **IGNORE** the 'role'/'is_admin' fields.
  *(Status: 200 - but field ignored)*
- **Side Effects**: Role remains unchanged (Verified via GET).

### 4. Security: ID Injection (Resource Hijacking)
- **URL**: `http://localhost:5500/api/users`
- **Method**: `PUT`
- **Pre-conditions**: Valid JWT.
- **Request Body**:
  ```json
  {
    "id": "<admin_id>",
    "username": "hacker_name"
  }
  ```
- **Expected Response**:
  - The API must **IGNORE** the `id` in the body and use the ID from the JWT.
  - The user's OWN record is updated, OR 400 Bad Request.
  *(Status: 200 or 400)*
- **Side Effects**: Target ID record unchanged.

### 5. Security: XSS Injection in Username
- **URL**: `http://localhost:5500/api/users`
- **Method**: `PUT`
- **Pre-conditions**: Valid JWT.
- **Request Body**:
  ```json
  { "username": "<script>alert(1)</script>" }
  ```
- **Expected Response**:
  - Status 422 (Validation Failed) OR 200 with sanitized output.
  - If 422:
    ```json
    {
      "status": false,
      "message": "Invalid characters",
      "details": [{ "field": "username", "message": "Invalid characters" }]
    }
    ```
- **Side Effects**: None or sanitized.

### 6. Update current user (Username/Email)
- **URL**: `http://localhost:5500/api/users`
- **Method**: `PUT`
- **Pre-conditions**:
  - Valid JWT.
- **Request Body**:
  ```json
  {
    "username": "NewUsername",
    "email": "new@email.com"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": true,
    "message": "User updated successfully"
  }
  ```
  *(Status: 200)*
- **Side Effects**:
  - User record updated.

### 7. Update with duplicate username/email
- **URL**: `http://localhost:5500/api/users`
- **Method**: `PUT`
- **Pre-conditions**: Valid JWT, email/username exists.
- **Request Body**: `{ "email": "<existing_email>" }`
- **Expected Response**: 409 Conflict.
- **Side Effects**: None.
