use serde::Serialize;

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

// Provide a simple alias for cases where details/result are None
impl<T> ErrorResponseDTO<T>
where
    T: Serialize,
{
    pub fn new(message: &'static str) -> Self {
        Self {
            success: false,
            message,
            details: None,
            result: None,
        }
    }
}

