use actix_web::{web, HttpResponse, Result};
use auth_integration::Subject;
use http_response::{create_response, HttpCodeW};
use models::internal::ListUsersQuery;

use super::service::UserService;

/// GET /v1/me
/// Get the authenticated church user (derived from JWT)
pub async fn get_me(
    db: web::Data<sea_orm::DatabaseConnection>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user = UserService::get_user_by_auth_id(&db, &subject.sub).await?;
    let resp = create_response(user, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

/// POST /v1/users/link
/// Link the authenticated user from auth server to church system
pub async fn link_user(
    db: web::Data<sea_orm::DatabaseConnection>,
    subject: Subject,
) -> Result<HttpResponse> {
    let response = UserService::link_user(&db, &subject.sub).await?;

    if response.message.contains("already") {
        let resp = create_response(response, HttpCodeW::OK);
        Ok(HttpResponse::Ok().json(resp))
    } else {
        let resp = create_response(response, HttpCodeW::Created);
        Ok(HttpResponse::Created().json(resp))
    }
}

/// GET /v1/users/:id
/// Get a specific church user by ID
pub async fn get_user(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    _subject: Subject,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();
    let user = UserService::get_user_by_id(&db, user_id).await?;

    let resp = create_response(user, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

/// GET /v1/users
/// List all church users (paginated)
pub async fn list_users(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<ListUsersQuery>,
    _subject: Subject,
) -> Result<HttpResponse> {
    let response = UserService::list_users(&db, query.page, query.limit).await?;

    let resp = create_response(response, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}
