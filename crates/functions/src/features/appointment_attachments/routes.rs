use actix_web::web;

use super::handlers::{
    create_attachment, delete_attachment, download_attachment, get_attachment, list_attachments,
};

pub fn configure_appointment_attachments(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/appointments/{appointment_id}/attachments",
        web::get().to(list_attachments),
    )
    .route(
        "/appointments/{appointment_id}/attachments",
        web::post().to(create_attachment),
    )
    .route(
        "/appointments/{appointment_id}/attachments/{attachment_id}",
        web::get().to(get_attachment),
    )
    .route(
        "/appointments/{appointment_id}/attachments/{attachment_id}/download",
        web::get().to(download_attachment),
    )
    .route(
        "/appointments/{appointment_id}/attachments/{attachment_id}",
        web::delete().to(delete_attachment),
    );
}
