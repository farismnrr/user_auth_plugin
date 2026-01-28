use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use std::sync::Arc;

use crate::domains::common::errors::AppError;
use crate::domains::mqtt::dtos::mqtt_dto::{
    CheckMqttUserRequest, CreateMqttUserRequest, MqttAclRequest,
};
use crate::domains::mqtt::usecases::mqtt_usecase::{MqttAclResult, MqttAuthResult, MqttUseCase};

pub struct MqttController;

impl MqttController {
    pub async fn create(
        usecase: web::Data<Arc<MqttUseCase>>,
        req: web::Json<CreateMqttUserRequest>,
    ) -> Result<impl Responder, AppError> {
        let result = usecase.create_user(req.into_inner()).await?;

        Ok(HttpResponse::Created().json(json!({
            "status": true,
            "message": "MQTT User created successfully",
            "data": result
        })))
    }

    pub async fn check(
        usecase: web::Data<Arc<MqttUseCase>>,
        req: web::Json<CheckMqttUserRequest>,
    ) -> Result<impl Responder, AppError> {
        match usecase.check_user(req.into_inner()).await {
            Ok(auth_result) => match auth_result {
                MqttAuthResult::Allow(is_superuser) => Ok(HttpResponse::Ok().json(json!({
                    "status": true,
                    "message": "Authentication successful",
                    "data": { "is_superuser": is_superuser },
                    "result": "allow"
                }))),
                MqttAuthResult::Deny => Ok(HttpResponse::Ok().json(json!({
                    "status": true,
                    "message": "Invalid information",
                    "result": "deny"
                }))),
                MqttAuthResult::Ignore => Ok(HttpResponse::Ok().json(json!({
                    "status": true,
                    "message": "User not found",
                    "result": "ignore"
                }))),
            },
            Err(e) => match e {
                AppError::ValidationError(msg, details) => Ok(HttpResponse::UnprocessableEntity()
                    .json(json!({
                        "status": false,
                        "message": msg,
                        "details": details,
                        "result": "ignore"
                    }))),
                _ => Err(e),
            },
        }
    }

    pub async fn acl(
        usecase: web::Data<Arc<MqttUseCase>>,
        req: web::Json<MqttAclRequest>,
    ) -> Result<impl Responder, AppError> {
        match usecase.check_acl(req.into_inner()).await {
            Ok(acl_result) => match acl_result {
                MqttAclResult::Allow(msg) => Ok(HttpResponse::Ok().json(json!({
                    "status": true,
                    "message": msg,
                    "result": "allow"
                }))),
                MqttAclResult::Deny => Ok(HttpResponse::Ok().json(json!({
                    "status": true,
                    "message": "Permission denied",
                    "result": "deny"
                }))),
                MqttAclResult::Ignore => Ok(HttpResponse::Ok().json(json!({
                    "status": true,
                    "message": "Ignored",
                    "result": "ignore"
                }))),
            },
            Err(e) => match e {
                AppError::ValidationError(msg, details) => Ok(HttpResponse::UnprocessableEntity()
                    .json(json!({
                        "status": false,
                        "message": msg,
                        "details": details,
                        "result": "ignore"
                    }))),
                _ => Err(e),
            },
        }
    }

    pub async fn list(usecase: web::Data<Arc<MqttUseCase>>) -> Result<impl Responder, AppError> {
        let users = usecase.get_all_users().await?;
        Ok(HttpResponse::Ok().json(json!({
            "status": true,
            "message": "User MQTT list retrieved successfully",
            "data": {
                "mqtt": users
            }
        })))
    }

    pub async fn delete(
        usecase: web::Data<Arc<MqttUseCase>>,
        path: web::Path<String>,
    ) -> Result<impl Responder, AppError> {
        let username = path.into_inner();
        usecase.delete_user(&username).await?;

        Ok(HttpResponse::Ok().json(json!({
            "status": true,
            "message": "MQTT User deleted successfully"
        })))
    }
}
