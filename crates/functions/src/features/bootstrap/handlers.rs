use actix_web::{web, HttpResponse, Result};
use auth_integration::Subject;
use http_response::{create_response, HttpCodeW};
use models::internal::{BootstrapCreated, BootstrapRequest, BootstrapResponse};

use crate::features::users::service::UserService;

pub async fn bootstrap(
    db: web::Data<sea_orm::DatabaseConnection>,
    _body: Option<web::Json<BootstrapRequest>>,
    subject: Subject,
) -> Result<HttpResponse> {
    let _created = BootstrapCreated::default();

    let link_res = UserService::link_user(&db, &subject.sub).await?;

    let user = UserService::get_user_by_id(&db, link_res.id).await?;

    let response = BootstrapResponse {
        user,
    };
    let response = create_response(response, HttpCodeW::Created);

    Ok(HttpResponse::Ok().json(response))
}