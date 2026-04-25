use actix_web::web;

use super::handlers;

/// Configure user_roles routes
/// - Self-service via /me/roles (JWT subject)
/// - Admin via /users/{user_id}/roles
/// - Query users by role via /roles/{role_id}/users
pub fn configure_user_roles(cfg: &mut web::ServiceConfig) {
    cfg
        // Self-service
        .service(
            web::resource("/me/roles")
                .route(web::get().to(handlers::get_my_roles))
                .route(web::post().to(handlers::assign_my_role)),
        )
        .service(
            web::resource("/me/roles/{role_id}").route(web::delete().to(handlers::remove_my_role)),
        )
        // Admin endpoints
        .service(
            web::resource("/users/{user_id}/roles")
                .route(web::get().to(handlers::get_user_roles))
                .route(web::post().to(handlers::assign_role)),
        )
        .service(
            web::resource("/users/{user_id}/roles/{role_id}")
                .route(web::delete().to(handlers::remove_role)),
        )
        // Query users by role
        .service(
            web::resource("/roles/{role_id}/users")
                .route(web::get().to(handlers::get_users_by_role)),
        );
}
