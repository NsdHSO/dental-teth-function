use actix_web::web;

use super::handlers;

/// Configure user routes
pub fn configure_users(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/me").route(web::get().to(handlers::get_me)))
        .service(
            web::scope("/users")
                .route("", web::get().to(handlers::list_users))
                .route("/link", web::post().to(handlers::link_user))
                .route("/{id}", web::get().to(handlers::get_user)),
        );
}
