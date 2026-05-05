use actix_web::web;

use super::handlers::{
    autocomplete_patients, create_patient, delete_patient, get_patient, list_patients,
    update_patient,
};

pub fn configure_patients(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/patients")
            // Literal route registered BEFORE parameterised /{id}.
            .route("/autocomplete", web::get().to(autocomplete_patients))
            .route("", web::get().to(list_patients))
            .route("", web::post().to(create_patient))
            .route("/{id}", web::get().to(get_patient))
            .route("/{id}", web::put().to(update_patient))
            .route("/{id}", web::delete().to(delete_patient)),
    );
}
