use actix_web::{web, HttpResponse, Result};
use auth_integration::Subject;
use http_response::{create_response, HttpCodeW};
use models::internal::user_profile::{CreateUserProfileRequest, UpdateUserProfileRequest};

use super::service::UserProfileService;

pub async fn get_my_profile(
    db: web::Data<sea_orm::DatabaseConnection>,
    subject: Subject,
) -> Result<HttpResponse> {
    let profile = UserProfileService::get_by_user_sub(&db, &subject.sub).await?;
    let resp = create_response(profile, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn create_my_profile(
    db: web::Data<sea_orm::DatabaseConnection>,
    subject: Subject,
    request: web::Json<CreateUserProfileRequest>,
) -> Result<HttpResponse> {
    let profile = UserProfileService::create(&db, &subject.sub, request.into_inner()).await?;
    let resp = create_response(profile, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(resp))
}

pub async fn update_my_profile(
    db: web::Data<sea_orm::DatabaseConnection>,
    subject: Subject,
    request: web::Json<UpdateUserProfileRequest>,
) -> Result<HttpResponse> {
    let profile = UserProfileService::update(&db, &subject.sub, request.into_inner()).await?;
    let resp = create_response(profile, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn upsert_my_profile(
    db: web::Data<sea_orm::DatabaseConnection>,
    subject: Subject,
    request: web::Json<CreateUserProfileRequest>,
) -> Result<HttpResponse> {
    let profile = UserProfileService::upsert(&db, &subject.sub, request.into_inner()).await?;
    let resp = create_response(profile, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}