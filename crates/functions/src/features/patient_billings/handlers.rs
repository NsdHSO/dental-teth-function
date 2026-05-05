use actix_web::{web, HttpResponse, Result};
use http_response::{create_response, HttpCodeW};
use models::internal::patient_billing::{
    CreateBillingRequest, ListBillingsQuery, UpdateBillingRequest,
};

use super::service::PatientBillingService;

pub async fn create_billing(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    request: web::Json<CreateBillingRequest>,
) -> Result<HttpResponse> {
    let row = PatientBillingService::create(
        db.get_ref(),
        path.into_inner(),
        request.into_inner(),
    )
    .await?;
    let resp = create_response(row, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(resp))
}

pub async fn list_billings(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    query: web::Query<ListBillingsQuery>,
) -> Result<HttpResponse> {
    let response = PatientBillingService::list(
        db.get_ref(),
        path.into_inner(),
        query.page,
        query.limit,
        query.status.clone(),
    )
    .await?;
    let resp = create_response(response, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn get_billing(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64, i64)>,
) -> Result<HttpResponse> {
    let (_patient_id, billing_id) = path.into_inner();
    let row = PatientBillingService::get_by_id(db.get_ref(), billing_id).await?;
    let resp = create_response(row, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn update_billing(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64, i64)>,
    request: web::Json<UpdateBillingRequest>,
) -> Result<HttpResponse> {
    let (_patient_id, billing_id) = path.into_inner();
    let row = PatientBillingService::update(db.get_ref(), billing_id, request.into_inner()).await?;
    let resp = create_response(row, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn delete_billing(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64, i64)>,
) -> Result<HttpResponse> {
    let (_patient_id, billing_id) = path.into_inner();
    PatientBillingService::delete(db.get_ref(), billing_id).await?;
    let resp = create_response((), HttpCodeW::NoContent);
    Ok(HttpResponse::NoContent().json(resp))
}

pub async fn mark_paid(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<(i64, i64)>,
) -> Result<HttpResponse> {
    let (_patient_id, billing_id) = path.into_inner();
    let row = PatientBillingService::mark_paid(db.get_ref(), billing_id).await?;
    let resp = create_response(row, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}
