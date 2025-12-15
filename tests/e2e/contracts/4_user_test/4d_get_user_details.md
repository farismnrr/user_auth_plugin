# ENDPOINT: GET /api/users/details

## Description
Retrieve current user's extended details.

## Test Scenarios

### 1. Get details without JWT
- **URL**: `http://localhost:5500/api/users/details`
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

### 2. Get user details
- **URL**: `http://localhost:5500/api/users/details`
- **Method**: `GET`
- **Pre-conditions**:
  - Valid JWT.
  - User details record exists (or returns empty/null fields if not).
- **Request Body**: None.
- **Expected Response**:
  ```json
  {
    "status": true,
    "message": "User details retrieved successfully",
    "data": {
      "user_details": {
        "first_name": "string",
        "last_name": "string",
        "phone": "string",
        "address": "string",
        "profile_picture_url": "string"
      }
    }
  }
  ```
  *(Status: 200)*
- **Side Effects**: None.

### 3. Get details when empty
- **URL**: `http://localhost:5500/api/users/details`
- **Method**: `GET`
- **Pre-conditions**: User just registered (no details added).
- **Request Body**: None.
- **Expected Response**:
  ```json
  {
    "status": true,
    "data": {
      "user_details": {
        "first_name": null,
        ...
      }
    }
  }
  ```
  *(Status: 200)*
- **Side Effects**: None.
