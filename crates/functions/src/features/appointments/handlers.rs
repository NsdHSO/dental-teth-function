use actix_web::{web, HttpResponse, Result};
use http_response::{create_response, HttpCodeW};
use models::internal::appointment::{
    CreateAppointmentRequest, UpdateAppointmentRequest,
};
use serde::Deserialize;

use super::service::AppointmentService;

#[derive(Debug, Deserialize)]
pub struct ListAppointmentsQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
    pub date: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub dentist_id: Option<i64>,
    pub patient_id: Option<i64>,
    pub status: Option<String>,
}

fn default_page() -> i64 { 1 }
fn default_limit() -> i64 { 20 }

pub async fn create_appointment(
    db: web::Data<sea_orm::DatabaseConnection>,
    request: web::Json<CreateAppointmentRequest>,
) -> Result<HttpResponse> {
    let row = AppointmentService::create(db.get_ref(), request.into_inner()).await?;
    Ok(HttpResponse::Created().json(create_response(row, HttpCodeW::Created)))
}

pub async fn list_appointments(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<ListAppointmentsQuery>,
) -> Result<HttpResponse> {
    let q = query.into_inner();
    let response = AppointmentService::list(
        db.get_ref(),
        q.page,
        q.limit,
        q.date,
        q.from,
        q.to,
        q.dentist_id,
        q.patient_id,
        q.status,
    )
    .await?;
    Ok(HttpResponse::Ok().json(create_response(response, HttpCodeW::OK)))
}

pub async fn get_appointment(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
) -> Result<HttpResponse> {
    let row = AppointmentService::get_by_id(db.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(create_response(row, HttpCodeW::OK)))
}

pub async fn update_appointment(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    request: web::Json<UpdateAppointmentRequest>,
) -> Result<HttpResponse> {
    let row =
        AppointmentService::update(db.get_ref(), path.into_inner(), request.into_inner()).await?;
    Ok(HttpResponse::Ok().json(create_response(row, HttpCodeW::OK)))
}

pub async fn delete_appointment(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
) -> Result<HttpResponse> {
    AppointmentService::delete(db.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::NoContent().json(create_response((), HttpCodeW::NoContent)))
}
