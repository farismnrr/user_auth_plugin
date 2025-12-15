# ENDPOINT: GET /auth/refresh

## Description
Refresh access token using refresh token from cookie.

## Test Scenarios

### 1. Refresh without token cookie
- **URL**: `http://localhost:5500/auth/refresh`
- **Method**: `GET`
- **Pre-conditions**: No cookie in request.
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

### 2. Refresh with invalid token
- **URL**: `http://localhost:5500/auth/refresh`
- **Method**: `GET`
- **Pre-conditions**: Cookie contains invalid token.
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

### 3. Refresh with expired token
- **URL**: `http://localhost:5500/auth/refresh`
- **Method**: `GET`
- **Pre-conditions**: Token expired.
- **Request Body**: None.
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Token expired"
  }
  ```
  *(Status: 401)*
- **Side Effects**: None.

### 4. Edge Case: Refresh token for different Tenant
- **URL**: `http://localhost:5500/auth/refresh`
- **Method**: `GET`
- **Pre-conditions**: Token issued for Tenant A, used on Tenant B.
- **Request Body**: None.
- **Expected Response**: Status 401 Unauthorized.
- **Side Effects**: None.

### 5. Security: Token Reuse Detection
- **URL**: `http://localhost:5500/auth/refresh`
- **Method**: `GET`
- **Pre-conditions**: Token already consumed.
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

### 6. Security: User State Check
- **URL**: `http://localhost:5500/auth/refresh`
- **Method**: `GET`
- **Pre-conditions**: User deleted/banned.
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

### 7. Successful token refresh
- **URL**: `http://localhost:5500/auth/refresh`
- **Method**: `GET`
- **Pre-conditions**:
  - Valid `refresh_token` in cookie.
- **Request Body**: None.
- **Expected Response**:
  ```json
  {
    "status": true,
    "message": "Token refreshed successfully",
    "data": { "access_token": "..." }
  }
  ```
  *(Status: 200)*
- **Side Effects**:
  - New Access Token generated.
  - Refresh token rotated (optional).
