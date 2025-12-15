use crate::dtos::user_details_dto::UpdateUserDetailsRequest;
use crate::dtos::response_dto::SuccessResponseDTO;
use serde_json::json;
use crate::errors::AppError;
use crate::usecases::user_details_usecase::UserDetailsUseCase;
use crate::usecases::auth_usecase::AuthUseCase;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_multipart::Multipart;
use std::sync::Arc;

/// Update current user's details (from JWT)
pub async fn update_user_details(
    usecase: web::Data<Arc<UserDetailsUseCase>>,
    body: web::Json<UpdateUserDetailsRequest>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let user_id = AuthUseCase::extract_user_id_from_request(&req)?;
    
    // Validation is handled in usecase
    usecase.update_user_details(user_id, body.into_inner()).await?;

    Ok(HttpResponse::Ok().json(SuccessResponseDTO::no_data(
        "User details updated successfully",
    )))
}

/// Upload current user's profile picture (from JWT)
pub async fn upload_profile_picture(
    usecase: web::Data<Arc<UserDetailsUseCase>>,
    payload: Multipart,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let user_id = AuthUseCase::extract_user_id_from_request(&req)?;
    let updated_details = usecase.update_profile_picture(user_id, payload).await?;
    Ok(HttpResponse::Ok().json(SuccessResponseDTO::new(
        "Profile picture uploaded successfully",
        json!({ "id": updated_details.id }),
    )))
}

/// Get current user's details (from JWT)
pub async fn get_user_details(
    usecase: web::Data<Arc<UserDetailsUseCase>>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let user_id = AuthUseCase::extract_user_id_from_request(&req)?;
    let user_details = usecase.get_user_details(user_id).await?;

    Ok(HttpResponse::Ok().json(SuccessResponseDTO::new(
        "User details retrieved successfully",
        json!({ "user_details": user_details }),
    )))
}
