use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{middleware::Logger, web, App, HttpServer};
use auth_integration::JwtAuth;
use chrono::Local;
use config_env::ConfigService;
use database::config::init;
use dotenvy::dotenv;
use env_logger::{Builder, Env};
use functions::{
    configure_bootstrap, configure_roles, configure_user_roles, configure_users,
};
use graphql::{
    build_schema, graphql_handler, graphql_playground, strapi_proxy_handler, StrapiClient,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let cfg = ConfigService::new().await;
    let conn: sea_orm::DatabaseConnection = init(cfg.database_url.clone(), cfg.sqlx_log)
        .await
        .expect("Failed to initialize database connection");
    Builder::from_env(Env::default().default_filter_or("debug"))
        .format(|buf, record| {
            use std::io::Write;
            let timestamp = Local::now().format("%Y-%m-%dT%H:%M:%S%.3f");
            writeln!(
                buf,
                "[{}] {} {} - {}",
                timestamp,
                record.level(),
                record.target(),
                record.args()
            )
        })
        .init();

    let data_base_conn = conn.clone();
    let host = cfg.host.clone();
    let port = cfg.port;

    let strapi_client = StrapiClient::new(format!("{}/graphql", cfg.strapi_api));

    let schema = build_schema(strapi_client.clone());

    let jwt_auth = JwtAuth::new(cfg.auth_base_url.clone());

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req| {
                let origin_str = origin.to_str().unwrap_or("");
                origin_str.starts_with("http://localhost:")
                    || origin_str.starts_with("http://127.0.0.1:")
                    || origin_str == "http://localhost"
                    || origin_str == "http://127.0.0.1"
            })
            .allowed_origin("https://tevet-troc-client.vercel.app")
            .allowed_origin("https://nsdhso.github.io")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::ACCEPT,
                header::AUTHORIZATION,
            ])
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(data_base_conn.clone()))
            .app_data(web::Data::new(schema.clone()))
            .app_data(web::Data::new(strapi_client.clone()))
            .app_data(web::Data::new(cfg.clone()))
            .wrap(Logger::default())
            .wrap(jwt_auth.clone())
            .service(
                web::scope("/v1")
                    .configure(configure_users)
                    .configure(configure_roles)
                    .configure(configure_user_roles)
                    .configure(configure_bootstrap),
            )
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql_handler))
                    .route(web::get().to(graphql_playground)),
            )
            .service(web::resource("/strapi-proxy").route(web::post().to(strapi_proxy_handler)))
            .wrap(cors)
    })
    .bind(format!("{host}:{port}"))
    .unwrap_or_else(|_| panic!("Failed to bind to {host}:{port}"));

    println!("Server running at http://{}:{}", host, port);
    server.run().await
}