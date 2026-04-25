use actix_web::{web, HttpResponse, Result};
use auth_integration::Subject;
use http_response::{create_response, HttpCodeW};
use models::internal::AssignRoleRequest;

use super::service::UserRoleService;
use crate::features::users::service::UserService;

/// POST /v1/users/:user_id/roles
/// Assign a role to a user (Admin only)
pub async fn assign_role(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    body: web::Json<AssignRoleRequest>,
    subject: Subject,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();

    // assigned_by comes from the authenticated subject
    let me = UserService::get_user_by_auth_id(&db, &subject.sub).await?;
    let assigned_by = me.id;

    let user_role = UserRoleService::assign_role(&db, user_id, body.role_id, assigned_by).await?;

    let resp = create_response(user_role, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(resp))
}

/// DELETE /v1/users/:user_id/roles/:role_id
/// Remove a role from a user (Admin only)
pub async fn remove_role(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64, i64)>,
    _subject: Subject,
) -> Result<HttpResponse> {
    let (user_id, role_id) = path.into_inner();

    UserRoleService::remove_role(&db, user_id, role_id).await?;

    let resp = create_response("Role removed", HttpCodeW::NoContent);
    Ok(HttpResponse::Ok().json(resp))
}

/// GET /v1/users/:user_id/roles
/// Get all roles for a specific user
pub async fn get_user_roles(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    _subject: Subject,
) -> Result<HttpResponse> {
    let user_id = path.into_inner();

    let roles = UserRoleService::get_user_roles(&db, user_id).await?;

    let resp = create_response(roles, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

/// Self-service: POST /v1/me/roles
pub async fn assign_my_role(
    db: web::Data<sea_orm::DatabaseConnection>,
    body: web::Json<AssignRoleRequest>,
    subject: Subject,
) -> Result<HttpResponse> {
    let me = UserService::get_user_by_auth_id(&db, &subject.sub).await?;
    let user_role = UserRoleService::assign_role(&db, me.id, body.role_id, me.id).await?;
    let resp = create_response(user_role, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(resp))
}

/// Self-service: DELETE /v1/me/roles/:role_id
pub async fn remove_my_role(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    subject: Subject,
) -> Result<HttpResponse> {
    let role_id = path.into_inner();
    let me = UserService::get_user_by_auth_id(&db, &subject.sub).await?;
    UserRoleService::remove_role(&db, me.id, role_id).await?;
    let resp = create_response("Role removed", HttpCodeW::NoContent);
    Ok(HttpResponse::Ok().json(resp))
}

/// Self-service: GET /v1/me/roles
pub async fn get_my_roles(
    db: web::Data<sea_orm::DatabaseConnection>,
    subject: Subject,
) -> Result<HttpResponse> {
    let me = UserService::get_user_by_auth_id(&db, &subject.sub).await?;
    let roles = UserRoleService::get_user_roles(&db, me.id).await?;
    let resp = create_response(roles, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

/// GET /v1/roles/:role_id/users
/// Get all users with a specific role
pub async fn get_users_by_role(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    _subject: Subject,
) -> Result<HttpResponse> {
    let role_id = path.into_inner();

    let users = UserRoleService::get_users_by_role(&db, role_id).await?;

    let resp = create_response(users, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}
