use actix_web::{web, HttpResponse, Result};
use http_response::{create_response, HttpCodeW};
use models::internal::autocomplete::AutocompleteQuery;
use models::internal::patient::{
    CreatePatientRequest, ListPatientsQuery, UpdatePatientRequest,
};

use super::service::PatientService;

pub async fn create_patient(
    db: web::Data<sea_orm::DatabaseConnection>,
    request: web::Json<CreatePatientRequest>,
) -> Result<HttpResponse> {
    let patient = PatientService::create(db.get_ref(), request.into_inner()).await?;
    let resp = create_response(patient, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(resp))
}

pub async fn get_patient(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
) -> Result<HttpResponse> {
    let patient = PatientService::get_by_id(db.get_ref(), path.into_inner()).await?;
    let resp = create_response(patient, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn update_patient(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    request: web::Json<UpdatePatientRequest>,
) -> Result<HttpResponse> {
    let patient =
        PatientService::update(db.get_ref(), path.into_inner(), request.into_inner()).await?;
    let resp = create_response(patient, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn delete_patient(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
) -> Result<HttpResponse> {
    PatientService::delete(db.get_ref(), path.into_inner()).await?;
    let resp = create_response((), HttpCodeW::NoContent);
    Ok(HttpResponse::NoContent().json(resp))
}

pub async fn list_patients(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<ListPatientsQuery>,
) -> Result<HttpResponse> {
    let response = PatientService::list(db.get_ref(), query.page, query.limit).await?;
    let resp = create_response(response, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn autocomplete_patients(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<AutocompleteQuery>,
) -> Result<HttpResponse> {
    let items = PatientService::autocomplete(db.get_ref(), &query.q, query.limit).await?;
    let resp = create_response(items, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}
