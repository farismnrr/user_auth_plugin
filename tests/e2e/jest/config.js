const path = require('path');
require('dotenv').config({ path: path.resolve(__dirname, '../../../.env') });

module.exports = {
    BASE_URL: process.env.TEST_BASE_URL || 'http://localhost:5500',
    API_KEY: process.env.TEST_API_KEY || process.env.API_KEY,
    TENANT_SECRET_KEY: process.env.TEST_TENANT_SECRET_KEY || process.env.TENANT_SECRET_KEY
};
