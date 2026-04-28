use actix_web::web;

use super::handlers::{create_my_profile, get_my_profile, update_my_profile, upsert_my_profile};

pub fn configure_user_profiles(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user-profiles")
            .route("/me", web::get().to(get_my_profile))
            .route("/me", web::post().to(create_my_profile))
            .route("/me", web::put().to(update_my_profile))
            .route("/me/upsert", web::post().to(upsert_my_profile)),
    );
}