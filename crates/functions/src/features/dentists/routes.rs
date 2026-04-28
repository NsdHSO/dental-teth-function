use actix_web::web;

use super::handlers::{create_dentist, delete_dentist, get_dentist, list_dentists, update_dentist};

pub fn configure_dentists(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/dentists")
            .route("", web::get().to(list_dentists))
            .route("", web::post().to(create_dentist))
            .route("/{id}", web::get().to(get_dentist))
            .route("/{id}", web::put().to(update_dentist))
            .route("/{id}", web::delete().to(delete_dentist)),
    );
}