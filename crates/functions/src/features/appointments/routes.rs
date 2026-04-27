use actix_web::web;

use super::handlers::{
    create_appointment, delete_appointment, get_appointment, list_appointments, update_appointment,
};

pub fn configure_appointments(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/appointments")
            .route("", web::get().to(list_appointments))
            .route("", web::post().to(create_appointment))
            .route("/{id}", web::get().to(get_appointment))
            .route("/{id}", web::put().to(update_appointment))
            .route("/{id}", web::delete().to(delete_appointment)),
    );
}