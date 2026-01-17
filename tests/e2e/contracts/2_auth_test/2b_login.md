# ENDPOINT: POST /auth/login

## Description
Authenticate user and receive access token.

## Test Scenarios

### 1. Missing API key
- **URL**: `http://localhost:5500/auth/login`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  {
    "email_or_username": "...",
    "password": "..."
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

### 2. Account Security: Login to Banned/Soft-Deleted Account
- **URL**: `http://localhost:5500/auth/login`
- **Method**: `POST`
- **Pre-conditions**: User is banned/deleted.
- **Request Body**:
  ```json
  {
    "email_or_username": "<banned_user>",
    "password": "<password>"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Forbidden"
  }
  ```
  *(Status: 403)*
- **Side Effects**: None.

### 3. Missing credentials
- **URL**: `http://localhost:5500/auth/login`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  {
    "email_or_username": "user"
    // Missing password
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

### 4. Invalid email format
- **URL**: `http://localhost:5500/auth/login`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  {
    "email_or_username": "invalid-email-format",
    "password": "..."
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "username or email or password invalid"
  }
  ```
  *(Status: 401)*
- **Side Effects**: None.

### 5. Wrong password
- **URL**: `http://localhost:5500/auth/login`
- **Method**: `POST`
- **Pre-conditions**:
  - User exists.
- **Request Body**:
  ```json
  {
    "email_or_username": "<email>",
    "password": "WrongPassword"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "username or email or password invalid"
  }
  ```
  *(Status: 401)*
- **Side Effects**: None.

### 6. Invalid Input: Leading/trailing spaces in credentials
- **URL**: `http://localhost:5500/auth/login`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  {
    "email_or_username": "  user  ",
    "password": "..."
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "username or email or password invalid"
  }
  ```
  *(Status: 401)*
- **Side Effects**: None.

### 7. Non-existent user
- **URL**: `http://localhost:5500/auth/login`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  {
    "email_or_username": "non-existent",
    "password": "..."
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "username or email or password invalid"
  }
  ```
  *(Status: 401)*
- **Side Effects**: None.

### 8. Security: Brute force protection check
- **URL**: `http://localhost:5500/auth/login`
- **Method**: `POST`
- **Pre-conditions**: Repeated failed attempts.
- **Request Body**:
  ```json
  {
    "email_or_username": "<user>",
    "password": "<password>"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Too Many Requests"
  }
  ```
  *(Status: 429)*
- **Side Effects**: None.

### 9. Successful login with email
- **URL**: `http://localhost:5500/auth/login`
- **Method**: `POST`
- **Pre-conditions**:
  - User exists.
- **Request Body**:
  ```json
  {
    "email_or_username": "<email>",
    "password": "<correct_password>"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": true,
    "message": "Login successful",
    "data": { "access_token": "..." }
  }
  ```
  *(Status: 200)*
- **Side Effects**:
  - Refresh token cookie set.
  - Session created.

### 11. Validation: Invalid SSO State (Special Chars)
- **URL**: `http://localhost:5500/auth/login`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  {
    "email_or_username": "user",
    "password": "password",
    "state": "invalid-state!"
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

### 12. Validation: SSO Nonce Too Long
- **URL**: `http://localhost:5500/auth/login`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  {
    "email_or_username": "user",
    "password": "password",
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

### 10. Successful login with username
- **URL**: `http://localhost:5500/auth/login`
- **Method**: `POST`
- **Pre-conditions**:
  - User exists.
- **Request Body**:
  ```json
  {
    "email_or_username": "<username>",
    "password": "<correct_password>"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": true,
    "message": "Login successful",
    "data": { "access_token": "..." }
  }
  ```
  *(Status: 200)*
- **Side Effects**:
  - Refresh token cookie set.
  - Session created.

### 13. Validation: Redirect URI not in allowed origins
- **URL**: `http://localhost:5500/auth/login`
- **Method**: `POST`
- **Pre-conditions**: None.
- **Request Body**:
  ```json
  {
    "email_or_username": "user",
    "password": "password",
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
### 14. Login Role Mismatch
- **URL**: `http://localhost:5500/auth/login`
- **Method**: `POST`
- **Pre-conditions**:
  - User exists with role "user" (or any role distinct from requested role).
- **Request Body**:
  ```json
  {
    "email_or_username": "<user_email>",
    "password": "<correct_password>",
    "role": "admin"
  }
  ```
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "User not found"
  }
  ```
  *(Status: 404)*
- **Side Effects**: None.
