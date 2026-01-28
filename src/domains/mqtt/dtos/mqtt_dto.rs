use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateMqttUserRequest {
    pub username: Option<String>,
    pub password: Option<String>,
    pub is_superuser: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CheckMqttUserRequest {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MqttAclRequest {
    pub username: Option<String>,
    pub topic: Option<String>,
    pub access: Option<String>, // "publish" or "subscribe"
}

#[derive(Debug, Serialize)]
pub struct MqttUserResponse {
    pub username: String,
    pub is_superuser: bool,
    pub is_deleted: bool,
}

#[derive(Debug, Serialize)]
pub struct MqttCheckResponse {
    pub result: String, // "allow", "deny", "ignore"
    pub is_superuser: bool,
}

#[derive(Debug, Serialize)]
pub struct MqttAclResponse {
    pub result: String, // "allow", "deny", "ignore"
}

// Standard Response wrapper is handled by common response format, but since check/acl have unique structure
// We might need custom structs or rely on the Controller to build them.
// Let's define the inner data structs here.
