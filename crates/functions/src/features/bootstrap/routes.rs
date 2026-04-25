use actix_web::web;

use super::handlers;

pub fn configure_bootstrap(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/bootstrap").route(web::post().to(handlers::bootstrap)));
}
