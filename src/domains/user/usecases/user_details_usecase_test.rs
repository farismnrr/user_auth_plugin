
#[cfg(test)]
mod tests {
    use crate::domains::user::usecases::user_details_usecase::UserDetailsUseCase;
    use crate::domains::user::repositories::user_details_repository::UserDetailsRepositoryTrait;
    use crate::domains::user::entities::user_details::Model as UserDetails;
    use crate::domains::user::dtos::user_details_dto::UpdateUserDetailsRequest;
    use crate::domains::common::errors::AppError;
    use async_trait::async_trait;
    use std::sync::Arc;
    use mockall::mock;
    use mockall::predicate::*;
    use uuid::Uuid;
    use chrono::{Utc, NaiveDate};

    // Mocking UserDetailsRepositoryTrait
    mock! {
        pub UserDetailsRepository {}
        #[async_trait]
        impl UserDetailsRepositoryTrait for UserDetailsRepository {
            async fn create(&self, user_id: Uuid) -> Result<UserDetails, AppError>;
            async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserDetails>, AppError>;
            async fn update(&self, user_id: Uuid, full_name: Option<String>, phone_number: Option<String>, address: Option<String>, date_of_birth: Option<NaiveDate>) -> Result<UserDetails, AppError>;
            async fn update_profile_picture(&self, user_id: Uuid, profile_picture_url: String) -> Result<UserDetails, AppError>;
        }
    }

    #[tokio::test]
    async fn test_update_user_details_success() {
        let mut mock_repo = MockUserDetailsRepository::new();
        let user_id = Uuid::new_v4();
        let details_id = Uuid::new_v4();

        // Existing details
        let existing_details = UserDetails {
            id: details_id,
            user_id,
            full_name: Some("John Doe".to_string()),
            phone_number: None,
            address: None,
            date_of_birth: None,
            profile_picture_url: None,
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
            deleted_at: None,
        };
        let existing_clone = existing_details.clone();

        // Updated details return
        let updated_details = UserDetails {
            id: details_id,
            user_id,
            full_name: Some("John Smith".to_string()),
            phone_number: Some("+1234567890".to_string()),
            address: None,
            date_of_birth: None,
            profile_picture_url: None,
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
            deleted_at: None,
        };
        let updated_clone = updated_details.clone();

        mock_repo
            .expect_find_by_user_id()
            .with(eq(user_id))
            .times(1)
            .returning(move |_| Ok(Some(existing_clone.clone())));

        mock_repo
            .expect_update()
            .with(
                eq(user_id),
                eq(Some("John Smith".to_string())), 
                eq(Some("+1234567890".to_string())),
                eq(None),
                eq(None)
            )
            .times(1)
            .returning(move |_, _, _, _, _| Ok(updated_clone.clone()));

        let usecase = UserDetailsUseCase::new(Arc::new(mock_repo));

        let req = UpdateUserDetailsRequest {
            first_name: None,
            last_name: Some("Smith".to_string()),
            phone_number: Some("+1234567890".to_string()),
            address: None,
            date_of_birth: None,
        };

        let result = usecase.update_user_details(user_id, req).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.last_name, Some("Smith".to_string()));
        assert_eq!(response.phone_number, Some("+1234567890".to_string()));
    }
}
