# ENDPOINT: PUT /auth/reset

## Description
Change authenticated user's password.

## Test Scenarios

### 1. Missing JWT token
- **URL**: `http://localhost:5500/auth/reset`
- **Method**: `PUT`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  { ... }
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

### 2. Wrong old password
- **URL**: `http://localhost:5500/auth/reset`
- **Method**: `PUT`
- **Pre-conditions**: Valid JWT.
- **Request Body**:
  ```json
  {
    "old_password": "WrongPassword",
    "new_password": "...",
    "confirm_new_password": "..."
  }
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

### 3. New passwords don't match
- **URL**: `http://localhost:5500/auth/reset`
- **Method**: `PUT`
- **Pre-conditions**: Valid JWT.
- **Request Body**:
  ```json
  {
    "old_password": "...",
    "new_password": "PassA",
    "confirm_new_password": "PassB"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Passwords do not match",
    "details": [
      {
        "field": "confirm_new_password",
        "message": "Passwords do not match"
      }
    ]
  }
  ```
  *(Status: 400 or 422)*
- **Side Effects**: None.

### 4. Weak new password
- **URL**: `http://localhost:5500/auth/reset`
- **Method**: `PUT`
- **Pre-conditions**: Valid JWT.
- **Request Body**:
  ```json
  {
    "old_password": "...",
    "new_password": "123",
    "confirm_new_password": "123"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Password too weak",
    "details": [
      {
        "field": "new_password",
        "message": "Password too weak"
      }
    ]
  }
  ```
  *(Status: 422)*
- **Side Effects**: None.

### 5. Validation: New password SAME as old password
- **URL**: `http://localhost:5500/auth/reset`
- **Method**: `PUT`
- **Pre-conditions**: Valid JWT.
- **Request Body**:
  ```json
  {
    "old_password": "pass",
    "new_password": "pass",
    "confirm_new_password": "pass"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "New password cannot be the same as old password",
    "details": [
      {
        "field": "new_password",
        "message": "New password cannot be the same as old password"
      }
    ]
  }
  ```
  *(Status: 400 or 422)*
- **Side Effects**: None.

### 6. Successful password change
- **URL**: `http://localhost:5500/auth/reset`
- **Method**: `PUT`
- **Pre-conditions**:
  - User logged in (valid access token).
- **Request Body**:
  ```json
  {
    "old_password": "<correct_old_password>",
    "new_password": "<new_strong_password>",
    "confirm_new_password": "<new_strong_password>"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": true,
    "message": "Password changed successfully"
  }
  ```
  *(Status: 200)*
- **Side Effects**:
  - Password updated in DB.
  - Sessions invalidated.

### 7. Verify login with new password works
- **URL**: `http://localhost:5500/auth/login`
- **Method**: `POST`
- **Pre-conditions**: Password recently changed.
- **Request Body**:
  ```json
  { "email_or_username": "...", "password": "<new_password>" }
  ```
- **Expected Response**: 200 OK.
- **Side Effects**: Session created.

### 8. Verify login with OLD password fails
- **URL**: `http://localhost:5500/auth/login`
- **Method**: `POST`
- **Pre-conditions**: Password recently changed.
- **Request Body**:
  ```json
  { "email_or_username": "...", "password": "<old_password>" }
  ```
- **Expected Response**: 401 Unauthorized.
- **Side Effects**: None.

### 9. Security: Revocation check
- **URL**: `http://localhost:5500/auth/refresh`
- **Method**: `GET`
- **Pre-conditions**: Old refresh token exists.
- **Request Body**: None.
- **Expected Response**: 401 Unauthorized.
- **Side Effects**: None.
