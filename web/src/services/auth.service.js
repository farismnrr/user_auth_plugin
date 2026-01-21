import api from "./api";

class AuthService {
  async login(email_or_username, password, ssoParams = {}) {
    const response = await api.post("/auth/login", {
      email_or_username,
      password,
      ...ssoParams,
    });
    return response.data;
  }

  async register(username, email, password, role = "user", invitation_code = null, ssoParams = {}) {
    const response = await api.post("/auth/register", {
      username,
      email,
      password,
      role,
      invitation_code,
      ...ssoParams,
    });
    return response.data;
  }

  async logout(token) {
    return await api.delete("/auth/logout", {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });
  }

  async refresh() {
    const response = await api.post("/auth/refresh");
    return response.data;
  }

  async verify(token) {
    const response = await api.get("/auth/verify", {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });
    return response.data;
  }
}

export default new AuthService();
