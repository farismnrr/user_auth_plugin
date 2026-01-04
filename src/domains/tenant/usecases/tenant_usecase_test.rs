
#[cfg(test)]
mod tests {
    use crate::domains::tenant::usecases::tenant_usecase::TenantUseCase;
    use crate::domains::tenant::repositories::tenant_repository::TenantRepositoryTrait;
    use crate::domains::tenant::entities::tenant::Model as Tenant;
    use crate::domains::tenant::dtos::tenant_dto::CreateTenantRequest;
    use crate::domains::common::errors::AppError;
    use async_trait::async_trait;
    use std::sync::Arc;
    use mockall::mock;
    use mockall::predicate::*;
    use uuid::Uuid;
    use chrono::Utc;

    // Mocking TenantRepositoryTrait
    mock! {
        pub TenantRepository {}
        #[async_trait]
        impl TenantRepositoryTrait for TenantRepository {
            async fn create(&self, tenant: CreateTenantRequest) -> Result<Tenant, AppError>;
            async fn find_by_id(&self, id: Uuid) -> Result<Option<Tenant>, AppError>;
            async fn find_by_name(&self, name: &str) -> Result<Option<Tenant>, AppError>;
            async fn find_by_name_with_deleted(&self, name: &str) -> Result<Option<Tenant>, AppError>;
            async fn find_all(&self) -> Result<Vec<Tenant>, AppError>;
            async fn update(&self, id: Uuid, tenant: crate::domains::tenant::dtos::tenant_dto::UpdateTenantRequest) -> Result<Tenant, AppError>;
            async fn delete(&self, id: Uuid) -> Result<(), AppError>;
            async fn restore(&self, id: Uuid) -> Result<(), AppError>;
        }
    }

    #[tokio::test]
    async fn test_create_tenant_success() {
        let mut mock_repo = MockTenantRepository::new();

        let tenant_id = Uuid::new_v4();
        let name = "New Tenant";
        let description = "Description";

        let tenant = Tenant {
            id: tenant_id,
            name: name.to_string(),
            description: Some(description.to_string()),
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
            deleted_at: None,
        };
        let tenant_clone = tenant.clone();

        // Expect find_by_name_with_deleted -> None
        mock_repo
            .expect_find_by_name_with_deleted()
            .with(eq(name))
            .returning(|_| Ok(None));

        // Expect create
        mock_repo
            .expect_create()
            .times(1)
            .returning(move |_| Ok(tenant_clone.clone()));

        let usecase = TenantUseCase::new(Arc::new(mock_repo));

        let req = CreateTenantRequest {
            name: name.to_string(),
            description: Some(description.to_string()),
        };

        let result = usecase.create_tenant(req).await;

        assert!(result.is_ok());
        let (response, created) = result.unwrap();
        assert_eq!(response.id, tenant_id);
        assert!(created);
    }

    #[tokio::test]
    async fn test_create_tenant_existing() {
        let mut mock_repo = MockTenantRepository::new();

        let tenant_id = Uuid::new_v4();
        let name = "Existing Tenant";

        let tenant = Tenant {
            id: tenant_id,
            name: name.to_string(),
            description: None,
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
            deleted_at: None,
        };
        let tenant_clone = tenant.clone();

        // Expect find_by_name_with_deleted -> Some(tenant)
        mock_repo
            .expect_find_by_name_with_deleted()
            .with(eq(name))
            .returning(move |_| Ok(Some(tenant_clone.clone())));

        // create should NOT be called

        let usecase = TenantUseCase::new(Arc::new(mock_repo));

        let req = CreateTenantRequest {
            name: name.to_string(),
            description: None,
        };

        let result = usecase.create_tenant(req).await;

        assert!(result.is_ok());
        let (response, created) = result.unwrap();
        assert_eq!(response.id, tenant_id);
        assert!(!created);
    }
}
