use serde::Serialize;
use uuid::Uuid;

/// Standard error response structure for API endpoints.
///
/// This DTO provides a consistent error response format across all API endpoints.
/// It includes a success flag (always false), a message, and optional details or result data.
#[derive(Serialize)]
pub struct ErrorResponseDTO<T>
where
    T: Serialize,
{
    pub success: bool,
    pub message: &'static str,
    pub details: Option<T>,
    pub result: Option<T>,
}

#[allow(dead_code)]
impl<T> ErrorResponseDTO<T>
where
    T: Serialize,
{
    /// Creates a new error response with only a message.
    pub fn new(message: &'static str) -> Self {
        Self {
            success: false,
            message,
            details: None,
            result: None,
        }
    }
}

/// Standard success response structure for API endpoints.
///
/// This DTO provides a consistent success response format across all API endpoints.
/// It includes a success flag (always true), a message, and optional data payload.
#[derive(Serialize)]
pub struct SuccessResponseDTO<T>
where
    T: Serialize,
{
    pub success: bool,
    pub message: &'static str,
    pub data: Option<T>,
}

#[allow(dead_code)]
impl<T> SuccessResponseDTO<T>
where
    T: Serialize,
{
    /// Creates a new success response with a message and data payload.
    pub fn new(message: &'static str, data: T) -> Self {
        Self {
            success: true,
            message,
            data: Some(data),
        }
    }
}

impl SuccessResponseDTO<()> {
    /// Creates a success response without data payload.
    ///
    /// This is typically used for operations like DELETE that don't return data.
    #[allow(dead_code)]
    pub fn no_data(message: &'static str) -> Self {
        Self {
            success: true,
            message,
            data: None,
        }
    }
}

/// Response containing only an ID.
///
/// Used for create/update operations to minimize response payload.
#[derive(Serialize)]
pub struct IdResponse {
    pub id: Uuid,
}
