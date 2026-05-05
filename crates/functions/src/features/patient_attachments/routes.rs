use actix_web::web;

use super::handlers::{
    create_attachment, delete_attachment, get_attachment, list_attachments,
};

pub fn configure_patient_attachments(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/patients/{patient_id}/attachments",
        web::get().to(list_attachments),
    )
    .route(
        "/patients/{patient_id}/attachments",
        web::post().to(create_attachment),
    )
    .route(
        "/patients/{patient_id}/attachments/{attachment_id}",
        web::get().to(get_attachment),
    )
    .route(
        "/patients/{patient_id}/attachments/{attachment_id}",
        web::delete().to(delete_attachment),
    );
}
