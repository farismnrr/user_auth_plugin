# K6 E2E Tests

Comprehensive blackbox E2E tests for all API endpoints using k6.

## Prerequisites

- k6 installed on your system
- Server running on `http://localhost:5500` (or set `BASE_URL` env var)
- Valid API key configured (or set `API_KEY` env var)

## Test Structure

```
tests/e2e/k6/
├── config.js              # Shared configuration
├── utils.js               # Helper functions
├── auth/                  # Auth endpoint tests
│   ├── register.js        # POST /auth/register
│   ├── login.js           # POST /auth/login
│   ├── logout.js          # POST /auth/logout
│   ├── refresh.js         # POST /auth/refresh
│   └── verify.js          # POST /auth/verify
├── users/                 # User endpoint tests
│   ├── get.js             # GET /users
│   ├── get_all.js         # GET /users/all
│   ├── update.js          # PUT /users
│   └── delete.js          # DELETE /users
└── user_details/          # User details endpoint tests
    ├── update.js          # PUT /users/details
    └── upload.js          # PATCH /users/uploads
```

## Running Tests

### Run All Tests
```bash
make test-k6
```

### Run Specific Test Suites
```bash
# Auth tests only
make test-k6-auth

# User tests only
make test-k6-users

# User details tests only
make test-k6-details
```

### Run Individual Test
```bash
k6 run tests/e2e/k6/auth/register.js
```

### With Custom Configuration
```bash
BASE_URL=http://localhost:3000 API_KEY=your-key k6 run tests/e2e/k6/auth/login.js
```

## Test Coverage

Each test file covers all user behavior permutations:

### Auth Tests (5 files)
- **register.js**: Valid registration, duplicate email/username, invalid email, missing fields, weak password
- **login.js**: Login with email/username, wrong password, non-existent user, missing credentials
- **logout.js**: Successful logout, logout without being logged in
- **refresh.js**: Valid refresh, invalid token, missing token, expired token
- **verify.js**: Valid JWT, invalid JWT, expired JWT, missing auth, deleted user

### User Tests (4 files)
- **get.js**: Get with valid JWT, without JWT, with invalid JWT
- **get_all.js**: Get all with valid JWT, without JWT, array validation
- **update.js**: Valid update, partial update, duplicate email/username, invalid data
- **delete.js**: Successful deletion, without JWT, already deleted user, login after deletion

### User Details Tests (2 files)
- **update.js**: Valid update, partial update, null values, invalid data types, invalid date
- **upload.js**: Valid upload, without JWT, invalid file type, missing file, oversized file

## Documentation

Each test file includes comprehensive documentation at the top:
- Endpoint URL and HTTP method
- Required headers
- Request body format
- Success response format
- Error responses
- Cookies (if applicable)
- Additional notes

## Notes

- Tests create test users in the database
- Consider running `make db-reset` between test runs for clean state
- All tests run with 1 VU (virtual user) and 1 iteration by default
- Response time threshold: 95% of requests < 2s
- Error rate threshold: < 1%
