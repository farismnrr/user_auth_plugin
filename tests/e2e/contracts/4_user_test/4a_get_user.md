# ENDPOINT: GET /api/users

## Description
Retrieve current authenticated user's profile.

## Test Scenarios

### 1. Get user without JWT
- **URL**: `http://localhost:5500/api/users`
- **Method**: `GET`
- **Pre-conditions**: No header.
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

### 2. Get user with invalid JWT
- **URL**: `http://localhost:5500/api/users`
- **Method**: `GET`
- **Pre-conditions**: Header with invalid token.
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

### 3. Get current user profile
- **URL**: `http://localhost:5500/api/users`
- **Method**: `GET`
- **Pre-conditions**:
  - Valid JWT for User A.
- **Request Body**: None.
- **Expected Response**:
  ```json
  {
    "status": true,
    "message": "User retrieved successfully",
    "data": {
      "user": {
        "id": "<user_a_id>",
        "username": "<user_a_name>",
        "email": "<user_a_email>",
        "role": "user"
        // Password must be EXCLUDED
      }
    }
  }
  ```
  *(Status: 200)*
- **Side Effects**: None.

### 4. Verify password exclusion
- **URL**: `http://localhost:5500/api/users`
- **Method**: `GET`
- **Pre-conditions**: Valid JWT.
- **Request Body**: None.
- **Expected Response**: Response body `data` object must NOT have `password`.
- **Side Effects**: None.
