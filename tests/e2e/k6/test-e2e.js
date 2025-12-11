/**
 * Main E2E Test Suite
 * 
 * Runs all E2E tests and generates HTML report
 */

import { htmlReport } from "https://raw.githubusercontent.com/benc-uk/k6-reporter/main/dist/bundle.js";
import { textSummary } from "https://jslib.k6.io/k6-summary/0.0.1/index.js";

// Import all test scenarios
import authRegisterTest from './auth/register.js';
import authLoginTest from './auth/login.js';
import authLogoutTest from './auth/logout.js';
import authRefreshTest from './auth/refresh.js';
import authVerifyTest from './auth/verify.js';

import usersGetTest from './users/get.js';
import usersGetAllTest from './users/get_all.js';
import usersUpdateTest from './users/update.js';
import usersSoftDeleteTest from './users/soft_delete.js';

import userDetailsUpdateTest from './user_details/update.js';
import userDetailsUploadTest from './user_details/upload.js';
import userDetailsSoftDeleteTest from './user_details/soft_delete.js';

import tenantsCreateTest from './tenants/create.js';
import tenantsGetTest from './tenants/get.js';
import tenantsUpdateTest from './tenants/update.js';
import tenantsSoftDeleteTest from './tenants/soft_delete.js';

export const options = {
    scenarios: {
        // Auth tests
        auth_register: { executor: 'shared-iterations', exec: 'authRegister', iterations: 1, maxDuration: '30s' },
        auth_login: { executor: 'shared-iterations', exec: 'authLogin', iterations: 1, maxDuration: '30s', startTime: '1s' },
        auth_logout: { executor: 'shared-iterations', exec: 'authLogout', iterations: 1, maxDuration: '30s', startTime: '2s' },
        auth_refresh: { executor: 'shared-iterations', exec: 'authRefresh', iterations: 1, maxDuration: '30s', startTime: '3s' },
        auth_verify: { executor: 'shared-iterations', exec: 'authVerify', iterations: 1, maxDuration: '30s', startTime: '4s' },

        // User tests
        users_get: { executor: 'shared-iterations', exec: 'usersGet', iterations: 1, maxDuration: '30s', startTime: '5s' },
        users_get_all: { executor: 'shared-iterations', exec: 'usersGetAll', iterations: 1, maxDuration: '30s', startTime: '6s' },
        users_update: { executor: 'shared-iterations', exec: 'usersUpdate', iterations: 1, maxDuration: '30s', startTime: '7s' },
        users_soft_delete: { executor: 'shared-iterations', exec: 'usersSoftDelete', iterations: 1, maxDuration: '30s', startTime: '8s' },

        // User details tests
        user_details_update: { executor: 'shared-iterations', exec: 'userDetailsUpdate', iterations: 1, maxDuration: '30s', startTime: '9s' },
        user_details_upload: { executor: 'shared-iterations', exec: 'userDetailsUpload', iterations: 1, maxDuration: '30s', startTime: '10s' },
        user_details_soft_delete: { executor: 'shared-iterations', exec: 'userDetailsSoftDelete', iterations: 1, maxDuration: '30s', startTime: '11s' },

        // Tenant tests
        tenants_create: { executor: 'shared-iterations', exec: 'tenantsCreate', iterations: 1, maxDuration: '30s', startTime: '12s' },
        tenants_get: { executor: 'shared-iterations', exec: 'tenantsGet', iterations: 1, maxDuration: '30s', startTime: '13s' },
        tenants_update: { executor: 'shared-iterations', exec: 'tenantsUpdate', iterations: 1, maxDuration: '30s', startTime: '14s' },
        tenants_soft_delete: { executor: 'shared-iterations', exec: 'tenantsSoftDelete', iterations: 1, maxDuration: '30s', startTime: '15s' },
    },
};

// Export test functions
export function authRegister() { authRegisterTest.default(); }
export function authLogin() { authLoginTest.default(); }
export function authLogout() { authLogoutTest.default(); }
export function authRefresh() { authRefreshTest.default(); }
export function authVerify() { authVerifyTest.default(); }

export function usersGet() { usersGetTest.default(); }
export function usersGetAll() { usersGetAllTest.default(); }
export function usersUpdate() { usersUpdateTest.default(); }
export function usersSoftDelete() { usersSoftDeleteTest.default(); }

export function userDetailsUpdate() { userDetailsUpdateTest.default(); }
export function userDetailsUpload() { userDetailsUploadTest.default(); }
export function userDetailsSoftDelete() { userDetailsSoftDeleteTest.default(); }

export function tenantsCreate() { tenantsCreateTest.default(); }
export function tenantsGet() { tenantsGetTest.default(); }
export function tenantsUpdate() { tenantsUpdateTest.default(); }
export function tenantsSoftDelete() { tenantsSoftDeleteTest.default(); }

export function handleSummary(data) {
    return {
        "coverage/test-e2e.html": htmlReport(data),
        stdout: textSummary(data, { indent: " ", enableColors: true }),
    };
}
