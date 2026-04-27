use actix_web::{web, HttpResponse, Result};
use http_response::{create_response, HttpCodeW};
use models::internal::{CreateAppointmentRequest, ListAppointmentsQuery, UpdateAppointmentRequest};

use super::service::AppointmentService;

pub async fn create_appointment(
    db: web::Data<sea_orm::DatabaseConnection>,
    request: web::Json<CreateAppointmentRequest>,
) -> Result<HttpResponse> {
    let appointment = AppointmentService::create(&db, request.into_inner()).await?;
    let resp = create_response(appointment, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(resp))
}

pub async fn get_appointment(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
) -> Result<HttpResponse> {
    let appointment_id = path.into_inner();
    let appointment = AppointmentService::get_by_id(&db, appointment_id).await?;
    let resp = create_response(appointment, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn update_appointment(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    request: web::Json<UpdateAppointmentRequest>,
) -> Result<HttpResponse> {
    let appointment_id = path.into_inner();
    let appointment = AppointmentService::update(&db, appointment_id, request.into_inner()).await?;
    let resp = create_response(appointment, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn delete_appointment(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
) -> Result<HttpResponse> {
    let appointment_id = path.into_inner();
    AppointmentService::delete(&db, appointment_id).await?;
    let resp = create_response((), HttpCodeW::NoContent);
    Ok(HttpResponse::NoContent().json(resp))
}

pub async fn list_appointments(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<ListAppointmentsQuery>,
) -> Result<HttpResponse> {
    let response = AppointmentService::list(
        &db,
        query.page,
        query.limit,
        query.date.clone(),
        query.from.clone(),
        query.to.clone(),
        query.dentist_id,
        query.status.clone(),
    )
    .await?;
    let resp = create_response(response, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}