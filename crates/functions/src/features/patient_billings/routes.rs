use actix_web::web;

use super::handlers::{
    create_billing, delete_billing, get_billing, list_billings, mark_paid, update_billing,
};

pub fn configure_patient_billings(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/patients/{patient_id}/billings",
        web::get().to(list_billings),
    )
    .route(
        "/patients/{patient_id}/billings",
        web::post().to(create_billing),
    )
    .route(
        "/patients/{patient_id}/billings/{billing_id}",
        web::get().to(get_billing),
    )
    .route(
        "/patients/{patient_id}/billings/{billing_id}",
        web::put().to(update_billing),
    )
    .route(
        "/patients/{patient_id}/billings/{billing_id}",
        web::delete().to(delete_billing),
    )
    .route(
        "/patients/{patient_id}/billings/{billing_id}/mark-paid",
        web::post().to(mark_paid),
    );
}
