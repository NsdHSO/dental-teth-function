use actix_web::web;

use super::handlers::{
    autocomplete_dentists, create_dentist, delete_dentist, get_dentist, list_dentists,
    update_dentist,
};

pub fn configure_dentists(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/dentists")
            // Literal segments BEFORE parameterised segments so "/autocomplete"
            // is not parsed as an i64 path id.
            .route("/autocomplete", web::get().to(autocomplete_dentists))
            .route("", web::get().to(list_dentists))
            .route("", web::post().to(create_dentist))
            .route("/{id}", web::get().to(get_dentist))
            .route("/{id}", web::put().to(update_dentist))
            .route("/{id}", web::delete().to(delete_dentist)),
    );
}
