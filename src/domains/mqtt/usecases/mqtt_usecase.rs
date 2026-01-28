use std::sync::Arc;

use crate::domains::common::errors::AppError;
use crate::domains::common::utils::password;
use crate::domains::mqtt::dtos::mqtt_dto::{
    CheckMqttUserRequest, CreateMqttUserRequest, MqttAclRequest, MqttUserResponse,
};
use crate::domains::mqtt::repositories::mqtt_repository::MqttRepositoryTrait;

pub struct MqttUseCase {
    repo: Arc<dyn MqttRepositoryTrait>,
}

pub enum MqttAuthResult {
    Allow(bool), // is_superuser
    Deny,
    Ignore,
}

pub enum MqttAclResult {
    Allow(String),
    Deny,
    Ignore,
}

impl MqttUseCase {
    pub fn new(repo: Arc<dyn MqttRepositoryTrait>) -> Self {
        Self { repo }
    }

    pub async fn create_user(
        &self,
        req: CreateMqttUserRequest,
    ) -> Result<MqttUserResponse, AppError> {
        // Validation
        let username = req.username.ok_or_else(|| {
            AppError::ValidationError(
                "Validation error".to_string(),
                Some(vec![crate::domains::common::errors::ValidationDetail {
                    field: "username".to_string(),
                    message: "Username is required".to_string(),
                }]),
            )
        })?;

        let password = req.password.ok_or_else(|| {
            AppError::ValidationError(
                "Validation error".to_string(),
                Some(vec![crate::domains::common::errors::ValidationDetail {
                    field: "password".to_string(),
                    message: "Password is required".to_string(),
                }]),
            )
        })?;

        let is_superuser = req.is_superuser.ok_or_else(|| {
            AppError::ValidationError(
                "Validation error".to_string(),
                Some(vec![crate::domains::common::errors::ValidationDetail {
                    field: "is_superuser".to_string(),
                    message: "is_superuser is required".to_string(),
                }]),
            )
        })?;

        // Username Regex Validation
        // ^[a-zA-Z0-9_\-]+$
        let username_regex = regex::Regex::new(r"^[a-zA-Z0-9_\-]+$").unwrap();
        if !username_regex.is_match(&username) {
            return Err(AppError::ValidationError(
                "Validation error".to_string(),
                Some(vec![crate::domains::common::errors::ValidationDetail {
                    field: "username".to_string(),
                    message: "Invalid username format".to_string(),
                }]),
            ));
        }

        // Validate password strength (reuse common validator if available, or simple check)
        if password.len() < 8 {
            return Err(AppError::ValidationError(
                "Validation error".to_string(),
                Some(vec![crate::domains::common::errors::ValidationDetail {
                    field: "password".to_string(),
                    message: "Password too weak".to_string(),
                }]),
            ));
        }

        let hashed = password::hash_password(&password)
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        let validated_req = crate::domains::mqtt::dtos::mqtt_dto::CreateMqttUserRequest {
            username: Some(username.clone()),
            password: Some(password.clone()),
            is_superuser: Some(is_superuser),
        };

        // Note: The repo expects CreateMqttUserRequest which now has Option fields.
        // We should probably update repo signature or pass raw values, but repo uses req.username directly.
        // Let's assume we pass the original req structure (but validated).
        // Actually, repo code: req.username.clone() -> Option<String>.
        // Repo needs to be updated too if it expects String!
        // Wait, repo `create` signature takes `CreateMqttUserRequest`.
        // So I need to update repo implementation to handle Option or unwrap safely (since we validated here).
        // BUT, repo shouldn't do validation. UseCase does.
        // So I should pass the unwrapped values or update repo to expect specific args?
        // Changing DTO everywhere is cleaner but repo needs update.

        let user = self.repo.create(validated_req, hashed).await?;

        Ok(MqttUserResponse {
            username: user.username,
            is_superuser: user.is_superuser,
            is_deleted: false,
        })
    }

    pub async fn check_user(&self, req: CheckMqttUserRequest) -> Result<MqttAuthResult, AppError> {
        let username = req.username.ok_or_else(|| {
            AppError::ValidationError(
                "Validation error".to_string(),
                Some(vec![crate::domains::common::errors::ValidationDetail {
                    field: "username".to_string(),
                    message: "Username is required".to_string(),
                }]),
            )
        })?;

        let password = req.password.ok_or_else(|| {
            // Contract expects validation error details for missing password
            // Note: Use "ignore" result if not validation error?
            // Contract 5b test 5 says: 422, result=ignore.
            // Controller maps MqttAuthResult. But if we return AppError::ValidationError, controller returns 422.
            // We need to ensure response body has "result": "ignore" for 422?
            // The json_error_handler returns standard error response.
            // It doesn't know about "result": "ignore".
            // We might need to handle this in controller or UseCase returns specific Result?
            // Let's implement UseCase returning ValidationError.
            AppError::ValidationError(
                "Validation error".to_string(),
                Some(vec![crate::domains::common::errors::ValidationDetail {
                    field: "password".to_string(),
                    message: "Password is required".to_string(),
                }]),
            )
        })?;

        let user_opt = self.repo.find_by_username(&username).await?;

        match user_opt {
            Some(user) => {
                let valid = password::verify_password(&password, &user.password)
                    .map_err(|e| AppError::InternalError(e.to_string()))?;

                if valid {
                    Ok(MqttAuthResult::Allow(user.is_superuser))
                } else {
                    Ok(MqttAuthResult::Deny)
                }
            }
            Option::None => Ok(MqttAuthResult::Ignore),
        }
    }

    pub async fn check_acl(&self, req: MqttAclRequest) -> Result<MqttAclResult, AppError> {
        let username = req.username.ok_or_else(|| {
            AppError::ValidationError(
                "Validation error".to_string(),
                Some(vec![crate::domains::common::errors::ValidationDetail {
                    field: "username".to_string(),
                    message: "Username is required".to_string(),
                }]),
            )
        })?;

        let topic = req.topic.ok_or_else(|| {
            AppError::ValidationError(
                "Validation error".to_string(),
                Some(vec![crate::domains::common::errors::ValidationDetail {
                    field: "topic".to_string(),
                    message: "Topic is required".to_string(),
                }]),
            )
        })?;

        // access is also required theoretically
        let _access = req.access.ok_or_else(|| {
            AppError::ValidationError(
                "Validation error".to_string(),
                Some(vec![crate::domains::common::errors::ValidationDetail {
                    field: "access".to_string(),
                    message: "Access is required".to_string(),
                }]),
            )
        })?;

        let user_opt = self.repo.find_by_username(&username).await?;

        // If user doesn't exist, we generally ignore or deny. EMQX usually expects ignore so existing rules can apply,
        // but if it's our authenticated user, we should know them.
        // Contract: "User Not Found" isn't explicitly tested in ACL but implied logic.
        // Existing logic in EMQX-Auth-Service:
        // is_superuser -> allow
        // topic starts with "users/{username}/" -> allow
        // else -> deny

        match user_opt {
            Some(user) => {
                if user.is_superuser {
                    return Ok(MqttAclResult::Allow("Superuser authorized".to_string()));
                }

                if topic.starts_with(&format!("users/{}/", user.username)) {
                    Ok(MqttAclResult::Allow("Authorization successful".to_string()))
                } else {
                    Ok(MqttAclResult::Deny)
                }
            }
            Option::None => Ok(MqttAclResult::Ignore), // Or Deny, but Ignore lets other plugins decide
        }
    }

    pub async fn get_all_users(&self) -> Result<Vec<MqttUserResponse>, AppError> {
        let users = self.repo.find_all().await?;
        Ok(users
            .into_iter()
            .map(|u| MqttUserResponse {
                username: u.username,
                is_superuser: u.is_superuser,
                is_deleted: u.deleted_at.is_some(),
            })
            .collect())
    }

    pub async fn delete_user(&self, username: &str) -> Result<(), AppError> {
        self.repo.delete(username).await
    }
}
