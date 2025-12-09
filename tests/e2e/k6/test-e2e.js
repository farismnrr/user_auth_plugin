import { htmlReport } from "https://raw.githubusercontent.com/benc-uk/k6-reporter/main/dist/bundle.js";
import { group } from "k6";
import { options as configOptions } from "./config.js";

// Auth Tests
import register from "./auth/register.js";
import login from "./auth/login.js";
import logout from "./auth/logout.js";
import refresh from "./auth/refresh.js";
import verify from "./auth/verify.js";

// User Tests
import usersGet from "./users/get.js";
import usersGetAll from "./users/get_all.js";
import usersUpdate from "./users/update.js";
import usersDelete from "./users/delete.js";

// User Details Tests
import userDetailsUpdate from "./user_details/update.js";
import userDetailsUpload from "./user_details/upload.js";

export const options = configOptions;

export default function () {
    group("Auth - Register", () => {
        register();
    });
    group("Auth - Login", () => {
        login();
    });
    group("Auth - Logout", () => {
        logout();
    });
    group("Auth - Refresh", () => {
        refresh();
    });
    group("Auth - Verify", () => {
        verify();
    });

    group("Users - Get Me", () => {
        usersGet();
    });
    group("Users - Get All", () => {
        usersGetAll();
    });
    group("Users - Update", () => {
        usersUpdate();
    });
    group("Users - Delete", () => {
        usersDelete();
    });

    group("User Details - Update", () => {
        userDetailsUpdate();
    });
    group("User Details - Upload", () => {
        userDetailsUpload();
    });
}

export function handleSummary(data) {
    return {
        "/scripts/coverage/test-e2e.html": htmlReport(data),
    };
}
