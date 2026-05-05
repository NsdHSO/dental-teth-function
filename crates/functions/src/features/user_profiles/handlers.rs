use actix_web::{web, HttpResponse, Result};
use auth_integration::Subject;
use http_response::{create_response, HttpCodeW};
use models::internal::user_profile::{CreateUserProfileRequest, UpdateUserProfileRequest};

use super::service::UserProfileService;
use crate::features::users::service::UserService;

pub async fn get_my_profile(
    db: web::Data<sea_orm::DatabaseConnection>,
    subject: Subject,
) -> Result<HttpResponse> {
    let profile = UserProfileService::get_by_user_sub(db.get_ref(), &subject.sub).await?;
    let resp = create_response(profile, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn create_my_profile(
    db: web::Data<sea_orm::DatabaseConnection>,
    subject: Subject,
    request: web::Json<CreateUserProfileRequest>,
) -> Result<HttpResponse> {
    let user = UserService::get_user_by_auth_id(db.get_ref(), &subject.sub).await?;
    let profile =
        UserProfileService::create(db.get_ref(), &subject.sub, user.id, request.into_inner())
            .await?;
    let resp = create_response(profile, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(resp))
}

pub async fn update_my_profile(
    db: web::Data<sea_orm::DatabaseConnection>,
    subject: Subject,
    request: web::Json<UpdateUserProfileRequest>,
) -> Result<HttpResponse> {
    let profile =
        UserProfileService::update(db.get_ref(), &subject.sub, request.into_inner()).await?;
    let resp = create_response(profile, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn upsert_my_profile(
    db: web::Data<sea_orm::DatabaseConnection>,
    subject: Subject,
    request: web::Json<CreateUserProfileRequest>,
) -> Result<HttpResponse> {
    let user = UserService::get_user_by_auth_id(db.get_ref(), &subject.sub).await?;
    let payload = request.into_inner();
    let profile = UserProfileService::upsert_identity(
        db.get_ref(),
        &subject.sub,
        user.id,
        payload.full_name,
        payload.phone,
        payload.email,
        payload.cnp,
    )
    .await?;
    let resp = create_response(profile, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}
