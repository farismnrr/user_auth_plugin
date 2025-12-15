# ENDPOINT: DELETE /api/users

## Description
Delete current user account (Self-deletion).

## Test Scenarios

### 1. Delete without JWT
- **URL**: `http://localhost:5500/api/users`
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

### 2. Delete current user
- **URL**: `http://localhost:5500/api/users`
- **Method**: `DELETE`
- **Pre-conditions**:
  - Valid JWT.
- **Request Body**: None.
- **Expected Response**:
  ```json
  {
    "status": true,
    "message": "User deleted successfully"
  }
  ```
  *(Status: 200)*
- **Side Effects**:
  - User record deleted (soft or hard).
  - Sessions invalidated.

### 3. Verify login after deletion
- **URL**: `http://localhost:5500/auth/login` (Verification Action)
- **Method**: `POST`
- **Pre-conditions**: User deleted.
- **Request Body**:
  ```json
  { "email_or_username": "...", "password": "..." }
  ```
- **Expected Response**: 401 Unauthorized or 404 Not Found.
- **Side Effects**: None.

### 4. Verify access with old token
- **URL**: `http://localhost:5500/api/users` (Verification Action)
- **Method**: `GET`
- **Pre-conditions**: User deleted, old token used.
- **Request Body**: None.
- **Expected Response**: 401 Unauthorized or 404 Not Found.
- **Side Effects**: None.

### 5. Re-create user (Data Restoration)
- **URL**: `http://localhost:5500/auth/register`
- **Method**: `POST`
- **Pre-conditions**: User was deleted.
- **Request Body**:
  ```json
  {
    "username": "<original_username>",
    "email": "<original_email>",
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
  *(Status: 200/201)*
- **Side Effects**:
  - User restored (new ID) to ensure subsequent tests have valid data.
