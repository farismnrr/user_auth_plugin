use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Response DTO for user_details data.
///
/// Contains all user_details fields for API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDetailsResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[serde(rename = "phone")]
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub profile_picture_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request DTO for updating user_details text fields.
///
/// All fields are optional. Profile picture is excluded and updated via separate endpoint.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateUserDetailsRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[serde(rename = "phone")]
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
}
