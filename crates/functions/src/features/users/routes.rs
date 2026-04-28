use actix_web::web;

use super::handlers;

/// Configure user routes (legacy - use user_profiles instead)
pub fn configure_users(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(handlers::list_users))
            .route("/{id}", web::get().to(handlers::get_user)),
    );
}
