use actix_web::{web, HttpResponse, Result};
use http_response::{create_response, HttpCodeW};
use models::internal::patient_attachment::{CreateAttachmentRequest, ListAttachmentsQuery};

use super::service::PatientAttachmentService;

pub async fn create_attachment(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    request: web::Json<CreateAttachmentRequest>,
) -> Result<HttpResponse> {
    let row = PatientAttachmentService::create(
        db.get_ref(),
        path.into_inner(),
        request.into_inner(),
    )
    .await?;
    let resp = create_response(row, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(resp))
}

pub async fn list_attachments(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    query: web::Query<ListAttachmentsQuery>,
) -> Result<HttpResponse> {
    let response = PatientAttachmentService::list(
        db.get_ref(),
        path.into_inner(),
        query.page,
        query.limit,
    )
    .await?;
    let resp = create_response(response, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn get_attachment(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64, i64)>,
) -> Result<HttpResponse> {
    let (_patient_id, attachment_id) = path.into_inner();
    let row = PatientAttachmentService::get_by_id(db.get_ref(), attachment_id).await?;
    let resp = create_response(row, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn delete_attachment(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64, i64)>,
) -> Result<HttpResponse> {
    let (_patient_id, attachment_id) = path.into_inner();
    PatientAttachmentService::delete(db.get_ref(), attachment_id).await?;
    let resp = create_response((), HttpCodeW::NoContent);
    Ok(HttpResponse::NoContent().json(resp))
}
