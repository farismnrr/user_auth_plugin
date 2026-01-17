# ENDPOINT: POST /auth/register

## Description
Register a new user account.

## Test Scenarios

### 1. Missing API key
- **URL**: `http://localhost:5500/auth/register`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  { "username": "...", "email": "...", "password": "...", "role": "user" }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Unauthorized"
  }
  ```
  *(Status: 401)*
- **Side Effects**: None.

### 2. Invalid email format
- **URL**: `http://localhost:5500/auth/register`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  {
    "username": "<unique_username>",
    "email": "not-an-email",
    "password": "StrongPassword123!",
    "role": "user"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Invalid email format"
  }
  ```
  *(Status: 422 Unprocessable Entity)*
- **Side Effects**: None.

### 3. Missing required fields
- **URL**: `http://localhost:5500/auth/register`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  {
    "username": "user",
    "email": "user@mail.com"
    // Missing password
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Missing required fields"
  }
  ```
  *(Status: 400 Bad Request)*
- **Side Effects**: None.

### 4. Weak password
- **URL**: `http://localhost:5500/auth/register`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  {
    "username": "<unique_username>",
    "email": "<unique_email>",
    "password": "123",
    "role": "user"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Password too weak"
  }
  ```
  *(Status: 400 or 422)*
- **Side Effects**: None.

### 5. Validation: Username with invalid chars
- **URL**: `http://localhost:5500/auth/register`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  {
    "username": "User Name",
    "email": "<unique_email>",
    "password": "StrongPassword123!",
    "role": "user"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Invalid characters"
  }
  ```
  *(Status: 422)*
- **Side Effects**: None.

### 6. Validation: Username using reserved words
- **URL**: `http://localhost:5500/auth/register`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  {
    "username": "admin",
    "email": "<unique_email>",
    "password": "StrongPassword123!",
    "role": "user"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Reserved Username"
  }
  ```
  *(Status: 400 or 409)*
- **Side Effects**: None.

### 7. Validation: Invalid Role
- **URL**: `http://localhost:5500/auth/register`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  {
    "username": "<unique_username>",
    "email": "<unique_email>",
    "password": "StrongPassword123!",
    "role": "GOD_MODE"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Bad Request"
  }
  ```
  *(Status: 400)*
- **Side Effects**: None.

### 8. Validation: Password too long
- **URL**: `http://localhost:5500/auth/register`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  {
    "username": "<unique_username>",
    "email": "<unique_email>",
    "password": "<100+ chars>",
    "role": "user"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Password too long",
    "details": [
      {
        "field": "password",
        "message": "Password too long"
      }
    ]
  }
  ```
  *(Status: 422)*
- **Side Effects**: None.

### 9. Successful registration
- **URL**: `http://localhost:5500/auth/register`
- **Method**: `POST`
- **Pre-conditions**:
  - Tenant must exist (`X-API-Key`).
- **Request Body**:
  ```json
  {
    "username": "<unique_username>",
    "email": "<unique_email>",
    "password": "StrongPassword123!",
    "role": "user"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": true,
    "message": "User registered successfully",
    "data": { "user_id": "<uuid>" }
  }
  ```
- **Side Effects**:
  - User record created.
  - Password hashed.

### 10. Duplicate email
- **URL**: `http://localhost:5500/auth/register`
- **Method**: `POST`
- **Pre-conditions**:
  - User with same email exists.
- **Request Body**:
  ```json
  {
    "username": "<unique_username>",
    "email": "<existing_email>",
    "password": "StrongPassword123!",
    "role": "user"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Email already exists"
  }
  ```
  *(Status: 409 Conflict)*
- **Side Effects**: None.

### 11. Duplicate username
- **URL**: `http://localhost:5500/auth/register`
- **Method**: `POST`
- **Pre-conditions**:
  - User with same username exists.
- **Request Body**:
  ```json
  {
    "username": "<existing_username>",
    "email": "<unique_email>",
    "password": "StrongPassword123!",
    "role": "user"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Username already exists"
  }
  ```
  *(Status: 409 Conflict)*
- **Side Effects**: None.

### 12. Edge Case: Email case sensitivity
- **URL**: `http://localhost:5500/auth/register`
- **Method**: `POST`
- **Pre-conditions**: `user@email.com` exists.
- **Request Body**:
  ```json
  {
    "username": "<unique>",
    "email": "User@Email.Com",
    "password": "...",
    "role": "user"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Email already exists"
  }
  ```
  *(Status: 409)*
- **Side Effects**: None.

### 13. Validation: Invalid SSO State (Special Chars)
- **URL**: `http://localhost:5500/auth/register`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  {
    "username": "<unique_username>",
    "email": "<unique_email>",
    "password": "StrongPassword123!",
    "role": "user",
    "state": "invalid_state!"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "State parameter must be alphanumeric"
  }
  ```
  *(Status: 422)*
- **Side Effects**: None.

### 14. Validation: SSO Nonce Too Long
- **URL**: `http://localhost:5500/auth/register`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  {
    "username": "<unique_username>",
    "email": "<unique_email>",
    "password": "StrongPassword123!",
    "role": "user",
    "nonce": "<129 chars>"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Nonce parameter too long (max 128 chars)"
  }
  ```
  *(Status: 422)*
- **Side Effects**: None.

### 15. Validation: Invalid Redirect URI (Injection)
- **URL**: `http://localhost:5500/auth/register`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  {
    "username": "<unique_username>",
    "email": "<unique_email>",
    "password": "StrongPassword123!",
    "role": "user",
    "redirect_uri": "https://example.com/<script>"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Redirect URI contains invalid characters"
  }
  ```
  *(Status: 422)*
- **Side Effects**: None.

### 16. Validation: Redirect URI not in allowed origins
- **URL**: `http://localhost:5500/auth/register`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  {
    "username": "<unique_username>",
    "email": "<unique_email>",
    "password": "StrongPassword123!",
    "role": "user",
    "redirect_uri": "https://evil-site.com/callback"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Redirect URI not in allowed origins"
  }
  ```
  *(Status: 403)*
- **Side Effects**: None.

### 17. User Role Multi-Tenant SSO (Success)
- **Description**: Role 'user' can register in multiple tenants with same credentials (Global SSO).
- **Pre-conditions**: User with role 'user' exists in Tenant A.
- **Request Body** (Tenant B with different API key):
  ```json
  {
    "username": "<existing_user_username>",
    "email": "<existing_user_email>",
    "password": "<CORRECT_PASSWORD>",
    "role": "user"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": true,
    "message": "User registered successfully",
    "data": { "user_id": "<SAME_UUID_AS_TENANT_A>" }
  }
  ```
  *(Status: 201 Created)*
- **Side Effects**: User linked to Tenant B with 'user' role. Shared identity across tenants.
- **Verification**: Check `user_tenants` table has 2 entries for same user_id with different tenant_ids.

### 18. Admin Role Cannot Share Credentials (Conflict)
- **Description**: Role 'admin' (or non-user) CANNOT register in new tenant if credentials already exist.
- **Pre-conditions**: Admin exists in Tenant A.
- **Request Body** (Tenant B with different API key):
  ```json
  {
    "username": "<existing_admin_username>",
    "email": "<existing_admin_email>",
    "password": "<ANY_PASSWORD>",
    "role": "admin"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Email already exists"
  }
  ```
  *(Status: 409 Conflict)*
- **Side Effects**: None. Admin NOT added to Tenant B.
- **Verification**: Check `user_tenants` table still has only 1 entry for this user.

### 19. Cannot Mix User and Admin Roles (Conflict)
- **Description**: Cannot register with 'admin' role if user already has 'user' role in another tenant (or vice versa).
- **Pre-conditions**: User exists with role 'user' in Tenant A.
- **Request Body** (Tenant B with different API key):
  ```json
  {
    "username": "<existing_user_username>",
    "email": "<existing_user_email>",
    "password": "<CORRECT_PASSWORD>",
    "role": "admin"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Cannot register as user - account exists with admin/non-user role"
  }
  ```
  *(Status: 409 Conflict)*
- **Side Effects**: None.
- **Verification**: User remains with 'user' role only.
