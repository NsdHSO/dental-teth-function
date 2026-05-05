use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Result};
use futures_util::TryStreamExt;
use http_response::{create_response, HttpCodeW};
use models::internal::appointment_attachment::ListAppointmentAttachmentsQuery;

use super::service::AppointmentAttachmentService;

pub async fn create_attachment(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    mut payload: Multipart,
) -> Result<HttpResponse> {
    let appointment_id = path.into_inner();
    let mut filename = None;
    let mut mime_type = "application/octet-stream".to_string();
    let mut file_data = Vec::new();

    while let Some(mut field) = payload.try_next().await? {
        if let Some(cd) = field.content_disposition() {
            if cd.get_name() == Some("file") {
                filename = cd.get_filename().map(|s| s.to_string());
                if let Some(ct) = field.content_type() {
                    mime_type = ct.to_string();
                }
                while let Some(chunk) = field.try_next().await? {
                    file_data.extend_from_slice(&chunk);
                }
            }
        }
    }

    if file_data.is_empty() {
        return Ok(HttpResponse::BadRequest().json(create_response(
            "No file provided".to_string(),
            HttpCodeW::BadRequest,
        )));
    }

    let row = AppointmentAttachmentService::create(
        db.get_ref(),
        appointment_id,
        filename,
        mime_type,
        file_data,
        None,
    )
    .await?;

    let resp = create_response(row, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(resp))
}

pub async fn list_attachments(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    query: web::Query<ListAppointmentAttachmentsQuery>,
) -> Result<HttpResponse> {
    let response = AppointmentAttachmentService::list(
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
    let (_appointment_id, attachment_id) = path.into_inner();
    let row = AppointmentAttachmentService::get_by_id(db.get_ref(), attachment_id).await?;
    let resp = create_response(row, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn download_attachment(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64, i64)>,
) -> Result<HttpResponse> {
    let (_appointment_id, attachment_id) = path.into_inner();
    let (file_data, mime_type, filename) =
        AppointmentAttachmentService::download(db.get_ref(), attachment_id).await?;

    let mut response = HttpResponse::Ok();
    response.content_type(mime_type);
    if let Some(name) = filename {
        response.insert_header((
            actix_web::http::header::CONTENT_DISPOSITION,
            format!(r#"attachment; filename="{}""#, name),
        ));
    }
    Ok(response.body(file_data))
}

pub async fn delete_attachment(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64, i64)>,
) -> Result<HttpResponse> {
    let (_appointment_id, attachment_id) = path.into_inner();
    AppointmentAttachmentService::delete(db.get_ref(), attachment_id).await?;
    let resp = create_response((), HttpCodeW::NoContent);
    Ok(HttpResponse::NoContent().json(resp))
}
