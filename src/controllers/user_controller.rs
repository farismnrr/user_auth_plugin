use crate::dtos::user_dto::UpdateUserRequest;
use crate::dtos::response_dto::{SuccessResponseDTO, IdResponse};
use crate::errors::AppError;
use crate::usecases::user_usecase::UserUseCase;
use crate::usecases::auth_usecase::AuthUseCase;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use std::sync::Arc;

/// Get current user (from JWT)
pub async fn get_user(
    usecase: web::Data<Arc<UserUseCase>>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let user_id = AuthUseCase::extract_user_id_from_request(&req)?;
    let user = usecase.get_user(user_id).await?;

    Ok(HttpResponse::Ok().json(SuccessResponseDTO::new("User retrieved successfully", user)))
}

/// Get all users
pub async fn get_all_users(
    usecase: web::Data<Arc<UserUseCase>>,
) -> Result<impl Responder, AppError> {
    let users = usecase.get_all_users().await?;

    Ok(HttpResponse::Ok().json(SuccessResponseDTO::new("Users retrieved successfully", users)))
}

/// Update current user (from JWT)
pub async fn update_user(
    usecase: web::Data<Arc<UserUseCase>>,
    body: web::Json<UpdateUserRequest>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let user_id = AuthUseCase::extract_user_id_from_request(&req)?;
    let updated_user = usecase.update_user(user_id, body.into_inner()).await?;

    Ok(HttpResponse::Ok().json(SuccessResponseDTO::new(
        "User updated successfully",
        IdResponse { id: updated_user.id },
    )))
}

/// Delete current user (from JWT)
pub async fn delete_user(
    usecase: web::Data<Arc<UserUseCase>>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let user_id = AuthUseCase::extract_user_id_from_request(&req)?;
    usecase.delete_user(user_id).await?;

    Ok(HttpResponse::Ok().json(SuccessResponseDTO::new(
        "User deleted successfully",
        IdResponse { id: user_id },
    )))
}
