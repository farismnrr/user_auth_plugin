use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Request DTO for creating a new tenant.
///
/// This DTO is used when creating a new tenant via the API.
#[derive(Debug, Deserialize)]
pub struct CreateTenantRequest {
    /// Tenant name (unique, required)
    pub name: String,
    
    /// Tenant description (optional)
    pub description: Option<String>,
}

/// Request DTO for updating an existing tenant.
///
/// All fields are optional to allow partial updates.
#[derive(Debug, Deserialize)]
pub struct UpdateTenantRequest {
    /// Updated tenant name
    pub name: Option<String>,
    
    /// Updated tenant description
    pub description: Option<String>,
}

/// Response DTO for tenant data.
///
/// This DTO is returned when retrieving tenant information.
#[derive(Debug, Serialize)]
pub struct TenantResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<crate::domains::tenant::entities::tenant::Model> for TenantResponse {
    fn from(tenant: crate::domains::tenant::entities::tenant::Model) -> Self {
        Self {
            id: tenant.id,
            name: tenant.name,
            description: tenant.description,
            is_active: tenant.deleted_at.is_none(),
            deleted_at: tenant.deleted_at,
            created_at: tenant.created_at,
            updated_at: tenant.updated_at,
        }
    }
}
