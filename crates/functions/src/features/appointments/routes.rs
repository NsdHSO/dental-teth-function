use actix_web::web;

use super::handlers::{
    create_appointment, delete_appointment, get_appointment, list_appointments, update_appointment,
};

pub fn configure_appointments(cfg: &mut web::ServiceConfig) {
    cfg.route("/appointments", web::get().to(list_appointments))
        .route("/appointments", web::post().to(create_appointment))
        .route("/appointments/{id}", web::get().to(get_appointment))
        .route("/appointments/{id}", web::put().to(update_appointment))
        .route("/appointments/{id}", web::delete().to(delete_appointment));
}
