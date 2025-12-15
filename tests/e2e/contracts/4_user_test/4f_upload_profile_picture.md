# ENDPOINT: PATCH /api/users/uploads

## Description
Upload a profile picture for the current user.

## Test Scenarios

### 1. Upload without JWT
- **URL**: `http://localhost:5500/api/users/uploads`
- **Method**: `PATCH`
- **Pre-conditions**: None.
- **Request Body**: None (or relevant body).
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Unauthorized"
  }
  ```
  *(Status: 401)*
- **Side Effects**: None.

### 2. Upload without file
- **URL**: `http://localhost:5500/api/users/uploads`
- **Method**: `PATCH`
- **Pre-conditions**: Valid JWT.
- **Request Body**: Empty or missing `file` field.
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Bad Request / Missing file"
  }
  ```
  *(Status: 400)*
- **Side Effects**: None.

### 3. Upload invalid file type
- **URL**: `http://localhost:5500/api/users/uploads`
- **Method**: `PATCH`
- **Pre-conditions**: Valid JWT.
- **Request Body**:
  - Form Data:
    - `file`: [Binary Text Data] (filename="file.txt")
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Invalid file type. Only images allowed."
  }
  ```
  *(Status: 400 or 415)*
- **Side Effects**: None.

### 4. Upload file too large
- **URL**: `http://localhost:5500/api/users/uploads`
- **Method**: `PATCH`
- **Pre-conditions**:
  - Valid JWT.
  - File size > 5MB.
- **Request Body**:
  - Form Data:
    - `file`: [>5MB Image Data]
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Payload Too Large"
  }
  ```
  *(Status: 413 Payload Too Large)*
- **Side Effects**: None.

### 5. Security: Malicious File Extension (RCE)
- **URL**: `http://localhost:5500/api/users/uploads`
- **Method**: `PATCH`
- **Pre-conditions**: Valid JWT.
- **Request Body**:
  - Form Data:
    - `file`: [Binary content] (filename="exploit.php")
- **Expected Response**:
  ```json
  {
    "status": false,
    "message": "Invalid file extension"
  }
  ```
  *(Status: 400/415)*
- **Side Effects**: None.

### 6. Security: Double Extension / MIME Type Bypass
- **URL**: `http://localhost:5500/api/users/uploads`
- **Method**: `PATCH`
- **Pre-conditions**: Valid JWT.
- **Request Body**:
  - Form Data:
    - `file`: [PHP Content] (filename="exploit.jpg.php", Content-Type="image/jpeg")
- **Expected Response**:
  - Server should inspect *usage* of file or validate magic bytes (file signature), not just extension/MIME.
  - Status: 400/415 or file saved but not executable.
- **Side Effects**: None or file rejected.

### 7. Security: Path Traversal in Filename
- **URL**: `http://localhost:5500/api/users/uploads`
- **Method**: `PATCH`
- **Pre-conditions**: Valid JWT.
- **Request Body**:
  - Form Data:
    - `file`: [Image Data] (filename="../../etc/passwd")
- **Expected Response**:
  - Server MUST sanitize filename (e.g., generate a UUID for storage).
  - Status: 200 (if renamed) or 400.
- **Side Effects**: None or sanitized save.

### 8. Security: Image Tragic / Malformed Image (DoS)
- **URL**: `http://localhost:5500/api/users/uploads`
- **Method**: `PATCH`
- **Pre-conditions**: Valid JWT.
- **Request Body**:
  - Form Data:
    - `file`: [Corrupted/DoS Image Binary]
- **Expected Response**:
  - 400 Bad Request or 422 Unprocessable Entity.
  - If 422:
    ```json
    {
      "status": false,
      "message": "Validation Error",
      "details": [{ "field": "file", "message": "Malformed image data" }]
    }
    ```
- **Side Effects**: None.

### 9. Upload valid profile picture
- **URL**: `http://localhost:5500/api/users/uploads`
- **Method**: `PATCH`
- **Pre-conditions**:
  - Valid JWT.
- **Request Body**:
  - Form Data:
    - `file`: [Binary Image Data] (filename="profile.jpg")
- **Expected Response**:
  ```json
  {
    "status": true,
    "message": "Profile picture uploaded successfully",
    "data": { "id": "uuid" }
  }
  ```
  *(Status: 200)*
- **Side Effects**:
  - File saved to storage.
  - User details `profile_picture_url` updated.
