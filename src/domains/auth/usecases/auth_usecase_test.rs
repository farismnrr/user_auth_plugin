#[cfg(test)]
mod tests {
    use crate::domains::auth::usecases::auth_usecase::AuthUseCase;
    use crate::domains::common::errors::AppError;
    use crate::domains::common::utils::password;
    use crate::domains::tenant::repositories::user_tenant_repository::{UserTenantInfo as TenantInfo, UserTenantRepositoryTrait};
    use crate::domains::user::dtos::auth_dto::LoginRequest;
    use crate::domains::user::entities::user::Model as User;
    use crate::domains::user::entities::user_activity_log::Model as UserActivityLog;
    use crate::domains::user::entities::user_details::Model as UserDetails;
    use crate::domains::user::entities::user_session::Model as UserSession;
    use crate::domains::user::repositories::user_activity_log_repository::UserActivityLogRepositoryTrait;
    use crate::domains::user::repositories::user_details_repository::UserDetailsRepositoryTrait;
    use crate::domains::user::repositories::user_repository::UserRepositoryTrait;
    use crate::domains::user::repositories::user_session_repository::UserSessionRepositoryTrait;

    use actix_web::test::TestRequest;
    use async_trait::async_trait;
    use chrono::{DateTime, Utc};
    use mockall::mock;
    use mockall::predicate::*;
    use std::sync::{Arc, Mutex};
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
            async fn create(&self, user_id: Uuid) -> Result<UserDetails, AppError>;
            async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserDetails>, AppError>;
            async fn update(&self, user_id: Uuid, full_name: Option<String>, phone_number: Option<String>, address: Option<String>, date_of_birth: Option<chrono::NaiveDate>) -> Result<UserDetails, AppError>;
            async fn update_profile_picture(&self, user_id: Uuid, profile_picture_url: String) -> Result<UserDetails, AppError>;
        }
    }


    // Fake UserTenantRepository
    struct FakeUserTenantRepository {
        role_response: Mutex<Option<String>>,
        all_tenants_response: Mutex<Vec<TenantInfo>>,
        add_user_calls: Mutex<Vec<(Uuid, Uuid, String)>>,
    }

    impl FakeUserTenantRepository {
        fn new() -> Self {
            Self {
                role_response: Mutex::new(None),
                all_tenants_response: Mutex::new(vec![]),
                add_user_calls: Mutex::new(vec![]),
            }
        }

        fn set_role_response(&self, role: Option<String>) {
            *self.role_response.lock().unwrap() = role;
        }

        fn set_all_tenants_response(&self, tenants: Vec<TenantInfo>) {
            *self.all_tenants_response.lock().unwrap() = tenants;
        }
    }

    #[async_trait]
    impl UserTenantRepositoryTrait for FakeUserTenantRepository {
        async fn add_user_to_tenant(&self, user_id: Uuid, tenant_id: Uuid, role: String) -> Result<(), AppError> {
            self.add_user_calls.lock().unwrap().push((user_id, tenant_id, role));
            Ok(())
        }

        async fn get_user_role_in_tenant(&self, _: Uuid, _: Uuid) -> Result<Option<String>, AppError> {
            Ok(self.role_response.lock().unwrap().clone())
        }

        async fn get_all_tenants_for_user(&self, _: Uuid) -> Result<Vec<TenantInfo>, AppError> {
            Ok(self.all_tenants_response.lock().unwrap().clone())
        }
    }

    // Mocking UserSessionRepositoryTrait
    mock! {
        pub UserSessionRepository {}
        #[async_trait]
        impl UserSessionRepositoryTrait for UserSessionRepository {
            async fn create_session(&self, user_id: Uuid, refresh_token_hash: String, user_agent: Option<String>, ip_address: Option<String>, expires_at: DateTime<Utc>) -> Result<UserSession, AppError>;
            async fn find_by_refresh_token_hash(&self, hash: &str) -> Result<Option<UserSession>, AppError>;
            async fn delete_session(&self, id: Uuid) -> Result<(), AppError>;
            async fn delete_all_sessions_for_user(&self, user_id: Uuid) -> Result<(), AppError>;
        }
    }

    // Mocking UserActivityLogRepositoryTrait
    mock! {
        pub UserActivityLogRepository {}
        #[async_trait]
        impl UserActivityLogRepositoryTrait for UserActivityLogRepository {
            async fn log_activity(&self, user_id: Option<Uuid>, activity_type: String, status: String, error_message: Option<String>, ip_address: Option<String>, user_agent: Option<String>) -> Result<UserActivityLog, AppError>;
        }
    }

    // Mocking InvitationCodeRepositoryTrait
    mock! {
        pub InvitationCodeRepository {}
        #[async_trait]
        impl crate::domains::auth::repositories::invitation_code_repository::InvitationCodeRepositoryTrait for InvitationCodeRepository {
            async fn save_code(&self, code: String, ttl: std::time::Duration) -> Result<(), AppError>;
            async fn validate_and_delete_code(&self, code: &str) -> Result<bool, AppError>;
        }
    }

    #[tokio::test]
    async fn test_login_success() {
        // Initialize config for test
        use crate::domains::common::utils::config::Config;
        Config::init_for_test();

        // Setup Mocks
        let mut mock_user_repo = MockUserRepository::new();
        let mock_details_repo = MockUserDetailsRepository::new();
        let mock_tenant_repo = Arc::new(FakeUserTenantRepository::new());
        let mut mock_session_repo = MockUserSessionRepository::new();
        let mut mock_activity_repo = MockUserActivityLogRepository::new();
        let mock_invite_repo = MockInvitationCodeRepository::new();

        // ... (existing test_login_success body) ...
        // I need to update the AuthUseCase::new call

        let user_id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let email = "test@example.com";
        let raw_password = "password123";
        let hashed_password = password::hash_password(raw_password).unwrap();

        let user = User {
            id: user_id,
            username: "testuser".to_string(),
            email: email.to_string(),
            password_hash: hashed_password,
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
            deleted_at: None,
        };
        let user_clone = user.clone();

        mock_user_repo
            .expect_find_by_email_with_deleted()
            .with(eq(email))
            .returning(move |_| Ok(Some(user_clone.clone())));

        mock_tenant_repo.set_role_response(Some("user".to_string()));

        mock_session_repo
            .expect_create_session()
            .returning(|_, _, _, _, _| {
                Ok(UserSession {
                    id: Uuid::new_v4(),
                    user_id: Uuid::new_v4(),
                    refresh_token_hash: "hash".to_string(),
                    user_agent: None,
                    ip_address: None,
                    expires_at: Utc::now().into(),
                    created_at: Utc::now().into(),
                })
            });

        mock_activity_repo
            .expect_log_activity()
            .returning(|_, _, _, _, _, _| {
                Ok(UserActivityLog {
                    id: Uuid::new_v4(),
                    user_id: Some(Uuid::new_v4()),
                    activity_type: "login".to_string(),
                    status: "success".to_string(),
                    error_message: None,
                    ip_address: None,
                    user_agent: None,
                    created_at: Utc::now().into(),
                })
            });

        let usecase = AuthUseCase::new(
            Arc::new(mock_user_repo),
            Arc::new(mock_details_repo),
            mock_tenant_repo.clone(),
            Arc::new(mock_session_repo),
            Arc::new(mock_activity_repo),
            Arc::new(mock_invite_repo),
        );

        let req = LoginRequest {
            email_or_username: email.to_string(),
            password: raw_password.to_string(),
            tenant_id,
            redirect_uri: None,
            state: None,
            nonce: None,
            role: None,
        };
        let http_req = TestRequest::default().to_http_request();

        let result = usecase.login(req, &http_req).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_login_invalid_password() {
        use crate::domains::common::utils::config::Config;
        Config::init_for_test();

        let mut mock_user_repo = MockUserRepository::new();
        let mock_details_repo = MockUserDetailsRepository::new();
        let mock_tenant_repo = Arc::new(FakeUserTenantRepository::new());
        let mock_session_repo = MockUserSessionRepository::new();
        let mut mock_activity_repo = MockUserActivityLogRepository::new();
        let mock_invite_repo = MockInvitationCodeRepository::new();

        let user_id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let email = "test@example.com";
        let raw_password = "password123";
        let hashed_password = password::hash_password(raw_password).unwrap();

        let user = User {
            id: user_id,
            username: "testuser".to_string(),
            email: email.to_string(),
            password_hash: hashed_password,
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
            deleted_at: None,
        };
        let user_clone = user.clone();

        mock_user_repo
            .expect_find_by_email_with_deleted()
            .returning(move |_| Ok(Some(user_clone.clone())));

        mock_activity_repo
            .expect_log_activity()
            .returning(|_, _, _, _, _, _| {
                Ok(UserActivityLog {
                    id: Uuid::new_v4(),
                    user_id: None,
                    activity_type: "".to_string(),
                    status: "".to_string(),
                    error_message: None,
                    ip_address: None,
                    user_agent: None,
                    created_at: Utc::now().into(),
                })
            });

        let usecase = AuthUseCase::new(
            Arc::new(mock_user_repo),
            Arc::new(mock_details_repo),
            mock_tenant_repo.clone(),
            Arc::new(mock_session_repo),
            Arc::new(mock_activity_repo),
            Arc::new(mock_invite_repo),
        );

        let req = LoginRequest {
            email_or_username: email.to_string(),
            password: "wrongpassword".to_string(),
            tenant_id,
            redirect_uri: None,
            state: None,
            nonce: None,
            role: None,
        };
        let http_req = TestRequest::default().to_http_request();

        assert!(usecase.login(req, &http_req).await.is_err());
    }

    #[tokio::test]
    async fn test_register_success() {
        use crate::domains::common::utils::config::Config;
        Config::init_for_test();

        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_details_repo = MockUserDetailsRepository::new();
        let mock_tenant_repo = Arc::new(FakeUserTenantRepository::new());
        let mock_session_repo = MockUserSessionRepository::new();
        let mut mock_activity_repo = MockUserActivityLogRepository::new();
        let mock_invite_repo = MockInvitationCodeRepository::new(); // Not used for "user" role

        let user_id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let email = "newuser@example.com";
        let username = "newuser";
        let password = "password123";

        // Mocks setup...
        mock_user_repo
            .expect_find_by_email_with_deleted()
            .returning(|_| Ok(None));
        mock_user_repo
            .expect_find_by_username()
            .returning(|_| Ok(None));
        mock_user_repo.expect_create().returning(move |_| {
            Ok(User {
                id: user_id,
                username: username.to_string(),
                email: email.to_string(),
                password_hash: "hash".to_string(),
                created_at: Utc::now().into(),
                updated_at: Utc::now().into(),
                deleted_at: None,
            })
        });

        mock_details_repo.expect_create().returning(move |_| {
            Ok(UserDetails {
                id: Uuid::new_v4(),
                user_id,
                full_name: None,
                phone_number: None,
                address: None,
                date_of_birth: None,
                profile_picture_url: None,
                created_at: Utc::now().into(),
                updated_at: Utc::now().into(),
                deleted_at: None,
            })
        });

        mock_tenant_repo.set_role_response(None);

        mock_activity_repo
            .expect_log_activity()
            .returning(|_, _, _, _, _, _| {
                Ok(UserActivityLog {
                    id: Uuid::new_v4(),
                    user_id: Some(Uuid::new_v4()),
                    activity_type: "".to_string(),
                    status: "".to_string(),
                    error_message: None,
                    ip_address: None,
                    user_agent: None,
                    created_at: Utc::now().into(),
                })
            });

        let usecase = AuthUseCase::new(
            Arc::new(mock_user_repo),
            Arc::new(mock_details_repo),
            mock_tenant_repo.clone(),
            Arc::new(mock_session_repo),
            Arc::new(mock_activity_repo),
            Arc::new(mock_invite_repo),
        );

        let req = crate::domains::user::dtos::auth_dto::RegisterRequest {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
            tenant_id,
            role: "user".to_string(),
            state: None,
            nonce: None,
            redirect_uri: None,
            invitation_code: None,
        };
        let http_req = TestRequest::default().to_http_request();

        assert!(usecase.register(req, &http_req).await.is_ok());
    }

    #[tokio::test]
    async fn test_register_admin_success() {
        use crate::domains::common::utils::config::Config;
        Config::init_for_test();

        let mut mock_user_repo = MockUserRepository::new();
        let mut mock_details_repo = MockUserDetailsRepository::new();
        let mock_tenant_repo = Arc::new(FakeUserTenantRepository::new());
        let mock_session_repo = MockUserSessionRepository::new();
        let mut mock_activity_repo = MockUserActivityLogRepository::new();
        let mut mock_invite_repo = MockInvitationCodeRepository::new();

        let user_id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let email = "admin@example.com";
        let username = "admin_user";
        let password = "password123";
        let code = "SECRET123";

        // Expect invitation code validation
        mock_invite_repo
            .expect_validate_and_delete_code()
            .with(eq(code))
            .times(1)
            .returning(|_| Ok(true));

        // Other mocks...
        mock_user_repo
            .expect_find_by_email_with_deleted()
            .returning(|_| Ok(None));
        mock_user_repo
            .expect_find_by_username()
            .returning(|_| Ok(None));
        mock_user_repo.expect_create().returning(move |_| {
            Ok(User {
                id: user_id,
                username: username.to_string(),
                email: email.to_string(),
                password_hash: "hash".to_string(),
                created_at: Utc::now().into(),
                updated_at: Utc::now().into(),
                deleted_at: None,
            })
        });

        mock_details_repo.expect_create().returning(move |_| {
            Ok(UserDetails {
                id: Uuid::new_v4(),
                user_id,
                full_name: None,
                phone_number: None,
                address: None,
                date_of_birth: None,
                profile_picture_url: None,
                created_at: Utc::now().into(),
                updated_at: Utc::now().into(),
                deleted_at: None,
            })
        });

        mock_tenant_repo.set_role_response(None);

        mock_activity_repo
            .expect_log_activity()
            .returning(|_, _, _, _, _, _| {
                Ok(UserActivityLog {
                    id: Uuid::new_v4(),
                    user_id: Some(Uuid::new_v4()),
                    activity_type: "".to_string(),
                    status: "".to_string(),
                    error_message: None,
                    ip_address: None,
                    user_agent: None,
                    created_at: Utc::now().into(),
                })
            });

        let usecase = AuthUseCase::new(
            Arc::new(mock_user_repo),
            Arc::new(mock_details_repo),
            mock_tenant_repo.clone(),
            Arc::new(mock_session_repo),
            Arc::new(mock_activity_repo),
            Arc::new(mock_invite_repo),
        );

        let req = crate::domains::user::dtos::auth_dto::RegisterRequest {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
            tenant_id,
            role: "admin".to_string(),
            state: None,
            nonce: None,
            redirect_uri: None,
            invitation_code: Some(code.to_string()),
        };
        let http_req = TestRequest::default().to_http_request();

        assert!(usecase.register(req, &http_req).await.is_ok());
    }

    #[tokio::test]
    async fn test_register_admin_failure_invalid_code() {
        use crate::domains::common::utils::config::Config;
        Config::init_for_test();

        let mock_user_repo = MockUserRepository::new();
        let mock_details_repo = MockUserDetailsRepository::new();
        let mock_tenant_repo = Arc::new(FakeUserTenantRepository::new());
        let mock_session_repo = MockUserSessionRepository::new();
        let mut mock_activity_repo = MockUserActivityLogRepository::new();
        let mut mock_invite_repo = MockInvitationCodeRepository::new();

        let tenant_id = Uuid::new_v4();
        let code = "INVALID";

        // Expect invitation code validation failure
        mock_invite_repo
            .expect_validate_and_delete_code()
            .with(eq(code))
            .times(1)
            .returning(|_| Ok(false));

        // Expect log failure
        mock_activity_repo
            .expect_log_activity()
            .returning(|_, _, _, _, _, _| {
                Ok(UserActivityLog {
                    id: Uuid::new_v4(),
                    user_id: None,
                    activity_type: "".to_string(),
                    status: "".to_string(),
                    error_message: None,
                    ip_address: None,
                    user_agent: None,
                    created_at: Utc::now().into(),
                })
            });

        let usecase = AuthUseCase::new(
            Arc::new(mock_user_repo),
            Arc::new(mock_details_repo),
            mock_tenant_repo.clone(),
            Arc::new(mock_session_repo),
            Arc::new(mock_activity_repo),
            Arc::new(mock_invite_repo),
        );

        let req = crate::domains::user::dtos::auth_dto::RegisterRequest {
            username: "admin_user".to_string(),
            email: "admin@example.com".to_string(),
            password: "password".to_string(),
            tenant_id,
            role: "admin".to_string(),
            state: None,
            nonce: None,
            redirect_uri: None,
            invitation_code: Some(code.to_string()),
        };
        let http_req = TestRequest::default().to_http_request();

        let result = usecase.register(req, &http_req).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Forbidden(_) => {}
            _ => panic!("Expected Forbidden error"),
        }
    }

    #[tokio::test]
    async fn test_register_fail_conflict_role_mixing() {
        use crate::domains::common::utils::config::Config;
        Config::init_for_test();

        let mut mock_user_repo = MockUserRepository::new();
        let mock_details_repo = MockUserDetailsRepository::new();
        let mock_tenant_repo = Arc::new(FakeUserTenantRepository::new());
        let mock_session_repo = MockUserSessionRepository::new();
        let mut mock_activity_repo = MockUserActivityLogRepository::new();
        let mock_invite_repo = MockInvitationCodeRepository::new();

        let user_id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let email = "admin@example.com";
        
        // Existing user found
        let user = User {
            id: user_id,
            username: "admin_user".to_string(),
            email: email.to_string(),
            password_hash: password::hash_password("password123").unwrap(),
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
            deleted_at: None,
        };
        let user_clone = user.clone();

        mock_user_repo
            .expect_find_by_email_with_deleted()
            .returning(move |_| Ok(Some(user_clone.clone())));

        // Existing role is "admin"
        mock_tenant_repo.set_all_tenants_response(vec![
            crate::domains::tenant::repositories::user_tenant_repository::UserTenantInfo {
                tenant_id: Uuid::new_v4(),
                role: "admin".to_string(),
            }
        ]);

        // Log failure
        mock_activity_repo
            .expect_log_activity()
            .returning(|_, _, _, _, _, _| {
                Ok(UserActivityLog {
                     id: Uuid::new_v4(), user_id: None, activity_type: "".to_string(), status: "".to_string(), 
                     error_message: None, ip_address: None, user_agent: None, created_at: Utc::now().into()
                })
            });

        let usecase = AuthUseCase::new(
            Arc::new(mock_user_repo),
            Arc::new(mock_details_repo),
            mock_tenant_repo.clone(),
            Arc::new(mock_session_repo),
            Arc::new(mock_activity_repo),
            Arc::new(mock_invite_repo),
        );

        let req = crate::domains::user::dtos::auth_dto::RegisterRequest {
            username: "admin_user".to_string(),
            email: email.to_string(),
            password: "password123".to_string(),
            tenant_id,
            role: "user".to_string(), // Trying to register as user
            state: None, nonce: None, redirect_uri: None, invitation_code: None,
        };
        let http_req = TestRequest::default().to_http_request();

        let result = usecase.register(req, &http_req).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Conflict(msg) => assert!(msg.contains("Cannot register as user")),
            _ => panic!("Expected Conflict error"),
        }
    }

    #[tokio::test]
    async fn test_register_success_multitenant_user_sso() {
        use crate::domains::common::utils::config::Config;
        Config::init_for_test();

        let mut mock_user_repo = MockUserRepository::new();
        let mock_details_repo = MockUserDetailsRepository::new();
        let mock_tenant_repo = Arc::new(FakeUserTenantRepository::new());
        let mut mock_session_repo = MockUserSessionRepository::new();
        let mut mock_activity_repo = MockUserActivityLogRepository::new();
        let mock_invite_repo = MockInvitationCodeRepository::new();

        let user_id = Uuid::new_v4();
        let old_tenant_id = Uuid::new_v4();
        let new_tenant_id = Uuid::new_v4();
        let email = "user@example.com";
        let password = "password123";

        // Existing user
        let user = User {
            id: user_id,
            username: "user".to_string(),
            email: email.to_string(),
            password_hash: password::hash_password(password).unwrap(),
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
            deleted_at: None,
        };
        let user_clone = user.clone();

        mock_user_repo
            .expect_find_by_email_with_deleted()
            .returning(move |_| Ok(Some(user_clone.clone())));

        // Existing role is "user" in old tenant
        mock_tenant_repo.set_all_tenants_response(vec![
            crate::domains::tenant::repositories::user_tenant_repository::UserTenantInfo {
                tenant_id: old_tenant_id,
                role: "user".to_string(),
            }
        ]);
        
        // Setup session mock for login (register logs user in)
        mock_session_repo
            .expect_create_session()
            .returning(|_, _, _, _, _| {
                 Ok(UserSession {
                    id: Uuid::new_v4(),
                    user_id: Uuid::new_v4(),
                    refresh_token_hash: "hash".to_string(),
                    user_agent: None,
                    ip_address: None,
                    expires_at: Utc::now().into(),
                    created_at: Utc::now().into(),
                })
            });

        // Log success
        mock_activity_repo
            .expect_log_activity()
            .returning(|_, _, _, _, _, _| {
                Ok(UserActivityLog {
                     id: Uuid::new_v4(), user_id: None, activity_type: "".to_string(), status: "".to_string(), 
                     error_message: None, ip_address: None, user_agent: None, created_at: Utc::now().into()
                })
            });

        let usecase = AuthUseCase::new(
            Arc::new(mock_user_repo),
            Arc::new(mock_details_repo),
            mock_tenant_repo.clone(),
            Arc::new(mock_session_repo),
            Arc::new(mock_activity_repo),
            Arc::new(mock_invite_repo),
        );

        let req = crate::domains::user::dtos::auth_dto::RegisterRequest {
            username: "user".to_string(),
            email: email.to_string(),
            password: password.to_string(),
            tenant_id: new_tenant_id,
            role: "user".to_string(),
            state: None, nonce: None, redirect_uri: None, invitation_code: None,
        };
        let http_req = TestRequest::default().to_http_request();

        assert!(usecase.register(req, &http_req).await.is_ok());
        
        // Verification: Check if add_user calls were recorded
        let calls = mock_tenant_repo.add_user_calls.lock().unwrap();
        assert!(!calls.is_empty());
        assert_eq!(calls[0].0, user_id);
        assert_eq!(calls[0].1, new_tenant_id);
    }

    #[tokio::test]
    async fn test_register_fail_existing_account_non_user() {
        use crate::domains::common::utils::config::Config;
        Config::init_for_test();

        let mut mock_user_repo = MockUserRepository::new();
        let mock_details_repo = MockUserDetailsRepository::new();
        let mock_tenant_repo = Arc::new(FakeUserTenantRepository::new());
        let mock_session_repo = MockUserSessionRepository::new();
        let mut mock_activity_repo = MockUserActivityLogRepository::new();
        let mut mock_invite_repo = MockInvitationCodeRepository::new();
        
        let user_id = Uuid::new_v4();
        let email = "user@example.com";
        let code = "CODE";

        // Expect invitation validation
        mock_invite_repo
            .expect_validate_and_delete_code()
            .with(eq(code))
            .times(1)
            .returning(|_| Ok(true));

        // Existing user
        let user = User {
            id: user_id,
            username: "user".to_string(),
            email: email.to_string(),
            password_hash: "hash".to_string(),
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
            deleted_at: None,
        };
        let user_clone = user.clone();

        mock_user_repo
            .expect_find_by_email_with_deleted()
            .returning(move |_| Ok(Some(user_clone.clone())));

        // Log failure
        mock_activity_repo
            .expect_log_activity()
            .returning(|_, _, _, _, _, _| {
                Ok(UserActivityLog {
                     id: Uuid::new_v4(), user_id: None, activity_type: "".to_string(), status: "".to_string(), 
                     error_message: None, ip_address: None, user_agent: None, created_at: Utc::now().into()
                })
            });

        let usecase = AuthUseCase::new(
            Arc::new(mock_user_repo),
            Arc::new(mock_details_repo),
            mock_tenant_repo.clone(),
            Arc::new(mock_session_repo),
            Arc::new(mock_activity_repo),
            Arc::new(mock_invite_repo),
        );

        let req = crate::domains::user::dtos::auth_dto::RegisterRequest {
            username: "user".to_string(),
            email: email.to_string(),
            password: "password123".to_string(),
            tenant_id: Uuid::new_v4(),
            role: "admin".to_string(), // Trying to register as admin when account exists
            state: None, nonce: None, redirect_uri: None, invitation_code: Some(code.to_string()),
        };
        let http_req = TestRequest::default().to_http_request();

        let result = usecase.register(req, &http_req).await;
        assert!(result.is_err());
         match result.unwrap_err() {
            AppError::Conflict(msg) => assert!(msg.contains("Email already exists")),
            _ => panic!("Expected Conflict error"),
        }
    }

    #[tokio::test]
    async fn test_register_fail_multitenant_wrong_password() {
        use crate::domains::common::utils::config::Config;
        Config::init_for_test();

        let mut mock_user_repo = MockUserRepository::new();
        let mock_details_repo = MockUserDetailsRepository::new();
        let mock_tenant_repo = Arc::new(FakeUserTenantRepository::new());
        let mock_session_repo = MockUserSessionRepository::new();
        let mut mock_activity_repo = MockUserActivityLogRepository::new();
        let mock_invite_repo = MockInvitationCodeRepository::new();
        
        let user_id = Uuid::new_v4();
        let email = "user@example.com";
        let password = "password123";

        // Existing user
        let user = User {
            id: user_id,
            username: "user".to_string(),
            email: email.to_string(),
            password_hash: password::hash_password(password).unwrap(),
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
            deleted_at: None,
        };
        let user_clone = user.clone();

        mock_user_repo
            .expect_find_by_email_with_deleted()
            .returning(move |_| Ok(Some(user_clone.clone())));

         // Existing role is "user" (so role check passes)
        mock_tenant_repo.set_all_tenants_response(vec![
            crate::domains::tenant::repositories::user_tenant_repository::UserTenantInfo {
                tenant_id: Uuid::new_v4(),
                role: "user".to_string(),
            }
        ]);

        // Log failure
        mock_activity_repo
            .expect_log_activity()
            .returning(|_, _, _, _, _, _| {
                Ok(UserActivityLog {
                     id: Uuid::new_v4(), user_id: None, activity_type: "".to_string(), status: "".to_string(), 
                     error_message: None, ip_address: None, user_agent: None, created_at: Utc::now().into()
                })
            });

        let usecase = AuthUseCase::new(
            Arc::new(mock_user_repo),
            Arc::new(mock_details_repo),
            mock_tenant_repo.clone(),
            Arc::new(mock_session_repo),
            Arc::new(mock_activity_repo),
            Arc::new(mock_invite_repo),
        );

        let req = crate::domains::user::dtos::auth_dto::RegisterRequest {
            username: "user".to_string(),
            email: email.to_string(),
            password: "WRONG_PASSWORD".to_string(),
            tenant_id: Uuid::new_v4(),
            role: "user".to_string(),
            state: None, nonce: None, redirect_uri: None, invitation_code: None,
        };
        let http_req = TestRequest::default().to_http_request();

        let result = usecase.register(req, &http_req).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Conflict(msg) => assert!(msg.contains("Email already exists")),
            _ => panic!("Expected Conflict error"),
        }
    }

    #[tokio::test]
    async fn test_login_role_mismatch_returns_not_found() {
        use crate::domains::common::utils::config::Config;
        Config::init_for_test();

        let mut mock_user_repo = MockUserRepository::new();
        let mock_details_repo = MockUserDetailsRepository::new();
        let mock_tenant_repo = Arc::new(FakeUserTenantRepository::new());
        let mock_session_repo = MockUserSessionRepository::new();
        let mut mock_activity_repo = MockUserActivityLogRepository::new();
        let mock_invite_repo = MockInvitationCodeRepository::new();

        let user_id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let email = "test@example.com";
        let raw_password = "password123";
        let hashed_password = password::hash_password(raw_password).unwrap();

        let user = User {
            id: user_id,
            username: "testuser".to_string(),
            email: email.to_string(),
            password_hash: hashed_password,
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
            deleted_at: None,
        };
        let user_clone = user.clone();

        mock_user_repo
            .expect_find_by_email_with_deleted()
            .returning(move |_| Ok(Some(user_clone.clone())));

        // ACTUAL role is "user"
        mock_tenant_repo.set_role_response(Some("user".to_string()));

        mock_activity_repo
            .expect_log_activity()
            .returning(|_, _, _, _, _, _| {
                Ok(UserActivityLog {
                    id: Uuid::new_v4(),
                    user_id: None,
                    activity_type: "".to_string(),
                    status: "".to_string(),
                    error_message: None,
                    ip_address: None,
                    user_agent: None,
                    created_at: Utc::now().into(),
                })
            });

        let usecase = AuthUseCase::new(
            Arc::new(mock_user_repo),
            Arc::new(mock_details_repo),
            mock_tenant_repo.clone(),
            Arc::new(mock_session_repo),
            Arc::new(mock_activity_repo),
            Arc::new(mock_invite_repo),
        );

        // REQUESTED role is "admin"
        let req = LoginRequest {
            email_or_username: email.to_string(),
            password: raw_password.to_string(),
            tenant_id,
            redirect_uri: None,
            state: None,
            nonce: None,
            role: Some("admin".to_string()), // <--- MISMATCH
        };
        let http_req = TestRequest::default().to_http_request();

        let result = usecase.login(req, &http_req).await;
        
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::NotFound(msg) => assert_eq!(msg, "User not found"),
            _ => panic!("Expected NotFound error"),
        }
    }
}
