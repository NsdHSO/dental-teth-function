use actix_web::web;

use super::handlers;

/// Configure role routes
pub fn configure_roles(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/roles")
            .route("", web::get().to(handlers::list_roles))
            .route("", web::post().to(handlers::create_role))
            .route("/{id}", web::get().to(handlers::get_role))
            .route("/{id}", web::put().to(handlers::update_role))
            .route("/{id}", web::delete().to(handlers::delete_role)),
    );
}
