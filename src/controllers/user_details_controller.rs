use crate::dtos::user_details_dto::UpdateUserDetailsRequest;
use crate::dtos::response_dto::{SuccessResponseDTO, IdResponse};
use crate::errors::AppError;
use crate::usecases::user_details_usecase::UserDetailsUseCase;
use crate::usecases::auth_usecase::AuthUseCase;
use crate::validators::user_details_validator;
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
    
    // Validate input
    user_details_validator::validate_full_name(&body.full_name)?;
    user_details_validator::validate_phone_number(&body.phone_number)?;
    user_details_validator::validate_address(&body.address)?;
    
    let user_details = usecase.update_user_details(user_id, body.into_inner()).await?;

    Ok(HttpResponse::Ok().json(SuccessResponseDTO::new(
        "User details updated successfully",
        IdResponse { id: user_details.id },
    )))
}

/// Upload current user's profile picture (from JWT)
pub async fn upload_profile_picture(
    usecase: web::Data<Arc<UserDetailsUseCase>>,
    payload: Multipart,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let user_id = AuthUseCase::extract_user_id_from_request(&req)?;
    let user_details = usecase.update_profile_picture(user_id, payload).await?;

    Ok(HttpResponse::Ok().json(SuccessResponseDTO::new(
        "Profile picture uploaded successfully",
        IdResponse { id: user_details.id },
    )))
}
