#[cfg(test)]
mod tests {
    use crate::domains::tenant::repositories::user_tenant_repository::UserTenantRepositoryTrait;
    use crate::domains::user::entities::user::Model as User;
    use crate::domains::user::repositories::user_details_repository::UserDetailsRepositoryTrait;
    use crate::domains::user::repositories::user_repository::UserRepositoryTrait;
    use crate::domains::user::usecases::user_usecase::UserUseCase;

    use crate::domains::common::errors::AppError;
    use async_trait::async_trait;
    use chrono::Utc;
    use mockall::mock;
    use mockall::predicate::*;
    use std::sync::Arc;
    use uuid::Uuid;

    // Mocking UserRepositoryTrait
    mock! {
        pub UserRepository {}
        #[async_trait]
        impl UserRepositoryTrait for UserRepository {
            async fn create(&self, user: crate::domains::user::dtos::user_dto::CreateUserRequest) -> Result<User, AppError>;
            async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
            async fn find_by_username(&self, username: &str) -> Result<Option<User>, AppError>;
            async fn find_all(&self) -> Result<Vec<User>, AppError>;
            async fn update(&self, id: Uuid, user: crate::domains::user::dtos::user_dto::UpdateUserRequest) -> Result<User, AppError>;
            async fn delete(&self, id: Uuid) -> Result<(), AppError>;
            async fn find_by_email_with_deleted(&self, email: &str) -> Result<Option<User>, AppError>;
            async fn find_by_username_with_deleted(&self, username: &str) -> Result<Option<User>, AppError>;
            async fn restore(&self, id: Uuid, req: crate::domains::user::dtos::user_dto::CreateUserRequest) -> Result<User, AppError>;
        }
    }

    // Mocking UserDetailsRepositoryTrait
    mock! {
        pub UserDetailsRepository {}
        #[async_trait]
        impl UserDetailsRepositoryTrait for UserDetailsRepository {
            async fn create(&self, user_id: Uuid) -> Result<crate::domains::user::entities::user_details::Model, AppError>;
            async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<crate::domains::user::entities::user_details::Model>, AppError>;
            async fn update(&self, user_id: Uuid, full_name: Option<String>, phone_number: Option<String>, address: Option<String>, date_of_birth: Option<chrono::NaiveDate>) -> Result<crate::domains::user::entities::user_details::Model, AppError>;
            async fn update_profile_picture(&self, user_id: Uuid, profile_picture_url: String) -> Result<crate::domains::user::entities::user_details::Model, AppError>;
        }
    }

    // Mocking UserTenantRepositoryTrait
    mock! {
        pub UserTenantRepository {}
        #[async_trait]
        impl UserTenantRepositoryTrait for UserTenantRepository {
             async fn add_user_to_tenant(&self, user_id: Uuid, tenant_id: Uuid, role: String) -> Result<(), AppError>;
             async fn get_user_role_in_tenant(&self, user_id: Uuid, tenant_id: Uuid) -> Result<Option<String>, AppError>;
             async fn get_all_tenants_for_user(&self, user_id: Uuid) -> Result<Vec<crate::domains::tenant::repositories::user_tenant_repository::UserTenantInfo>, AppError>;
        }
    }

    #[tokio::test]
    async fn test_get_user_success() {
        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_details_repo = MockUserDetailsRepository::new();
        let mut mock_tenant_repo = MockUserTenantRepository::new();

        let user_id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();

        let user = User {
            id: user_id,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hashed".to_string(),
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
            deleted_at: None,
        };

        let user_clone = user.clone();

        // Expect find_by_id to be called once and return the user
        mock_user_repo
            .expect_find_by_id()
            .with(eq(user_id))
            .times(1)
            .returning(move |_| Ok(Some(user_clone.clone())));

        // Expect find_by_user_id to be called and return None (no details for now)
        mock_details_repo
            .expect_find_by_user_id()
            .with(eq(user_id))
            .returning(|_| Ok(None));

        // Expect get_user_role_in_tenant to be called
        mock_tenant_repo
            .expect_get_user_role_in_tenant()
            .with(eq(user_id), eq(tenant_id))
            .returning(|_, _| Ok(Some("admin".to_string())));

        let usecase = UserUseCase::new(
            Arc::new(mock_user_repo),
            Arc::new(mock_details_repo),
            Arc::new(mock_tenant_repo),
        );

        let result = usecase.get_user(user_id, tenant_id).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.id, user_id);
        assert_eq!(response.role, "admin");
    }

    #[tokio::test]
    async fn test_get_user_not_found() {
        let mut mock_user_repo = MockUserRepository::new();
        let mock_details_repo = MockUserDetailsRepository::new();
        let mock_tenant_repo = MockUserTenantRepository::new();

        let user_id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();

        mock_user_repo
            .expect_find_by_id()
            .with(eq(user_id))
            .times(1)
            .returning(|_| Ok(None));

        let usecase = UserUseCase::new(
            Arc::new(mock_user_repo),
            Arc::new(mock_details_repo),
            Arc::new(mock_tenant_repo),
        );

        let result = usecase.get_user(user_id, tenant_id).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::NotFound(msg) => assert!(msg.contains("not found")),
            _ => panic!("Expected NotFound error"),
        }
    }
}
