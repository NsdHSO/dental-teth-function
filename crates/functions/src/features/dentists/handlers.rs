use actix_web::{web, HttpResponse, Result};
use http_response::{create_response, HttpCodeW};
use models::internal::autocomplete::AutocompleteQuery;
use models::internal::{CreateDentistRequest, ListDentistsQuery, UpdateDentistRequest};

use super::service::DentistService;

pub async fn create_dentist(
    db: web::Data<sea_orm::DatabaseConnection>,
    request: web::Json<CreateDentistRequest>,
) -> Result<HttpResponse> {
    let dentist = DentistService::create(db.get_ref(), request.into_inner()).await?;
    let resp = create_response(dentist, HttpCodeW::Created);
    Ok(HttpResponse::Created().json(resp))
}

pub async fn get_dentist(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
) -> Result<HttpResponse> {
    let dentist = DentistService::get_by_id(db.get_ref(), path.into_inner()).await?;
    let resp = create_response(dentist, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn update_dentist(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
    request: web::Json<UpdateDentistRequest>,
) -> Result<HttpResponse> {
    let dentist =
        DentistService::update(db.get_ref(), path.into_inner(), request.into_inner()).await?;
    let resp = create_response(dentist, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn delete_dentist(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i64>,
) -> Result<HttpResponse> {
    DentistService::delete(db.get_ref(), path.into_inner()).await?;
    let resp = create_response((), HttpCodeW::NoContent);
    Ok(HttpResponse::NoContent().json(resp))
}

pub async fn list_dentists(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<ListDentistsQuery>,
) -> Result<HttpResponse> {
    let response =
        DentistService::list(db.get_ref(), query.page, query.limit, query.specialty.clone())
            .await?;
    let resp = create_response(response, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn autocomplete_dentists(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<AutocompleteQuery>,
) -> Result<HttpResponse> {
    let items = DentistService::autocomplete(db.get_ref(), &query.q, query.limit).await?;
    let resp = create_response(items, HttpCodeW::OK);
    Ok(HttpResponse::Ok().json(resp))
}
