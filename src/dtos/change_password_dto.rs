use serde::{Deserialize, Serialize};

/// Request DTO for changing user password.
///
/// Contains the old password for verification and the new password with confirmation.
#[derive(Debug, Deserialize, Serialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
    pub confirm_new_password: String,
}
