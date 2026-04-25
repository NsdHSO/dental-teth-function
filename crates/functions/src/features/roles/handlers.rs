use actix_web::{web, HttpResponse, Result};
use auth_integration::Subject;
use http_response::{create_response, HttpCodeW};
use models::internal::{CreateRoleRequest, ListRolesQuery, UpdateRoleRequest};

use super::service::RoleService;

/// POST /v1/roles
/// Create a new role (Admin only)
pub async fn create_role(
    db: web::Data<sea_orm::DatabaseConnection>,
    body: web::Json<CreateRoleRequest>,
    _subject: Subject,
) -> Result<HttpResponse> {
    let role = RoleService::create_role(&db, body.into_inner()).await?;

    let resp = create_response(role, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(resp))
}

/// GET /v1/roles/:id
/// Get a specific role
pub async fn get_role(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    _subject: Subject,
) -> Result<HttpResponse> {
    let role_id = path.into_inner();
    let role = RoleService::get_role_by_id(&db, role_id).await?;

    let resp = create_response(role, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

/// GET /v1/roles
/// List all roles (paginated)
pub async fn list_roles(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<ListRolesQuery>,
    _subject: Subject,
) -> Result<HttpResponse> {
    let response = RoleService::list_roles(&db, query.page, query.limit).await?;

    let resp = create_response(response, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

/// PUT /v1/roles/:id
/// Update a role (Admin only)
pub async fn update_role(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    body: web::Json<UpdateRoleRequest>,
    _subject: Subject,
) -> Result<HttpResponse> {
    let role_id = path.into_inner();
    let role = RoleService::update_role(&db, role_id, body.into_inner()).await?;

    let resp = create_response(role, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

/// DELETE /v1/roles/:id
/// Delete a role (Admin only)
pub async fn delete_role(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    _subject: Subject,
) -> Result<HttpResponse> {
    let role_id = path.into_inner();
    RoleService::delete_role(&db, role_id).await?;

    // Return a standardized JSON wrapper with a NoContent logical code
    let resp = create_response("Role deleted", HttpCodeW::NoContent);
    Ok(HttpResponse::Ok().json(resp))
}
