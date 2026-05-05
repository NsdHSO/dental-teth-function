use actix_web::{test, web, App};
use functions::{configure_dentists, configure_patients};

#[actix_web::test]
async fn dentist_autocomplete_rejects_short_query() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(stub_db()))
            .service(web::scope("/v1").configure(configure_dentists)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/v1/dentists/autocomplete?q=a")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400, "q < 2 chars must be 400");
}

#[actix_web::test]
async fn dentist_autocomplete_route_is_not_shadowed_by_id_route() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(stub_db()))
            .service(web::scope("/v1").configure(configure_dentists)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/v1/dentists/autocomplete?q=")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(
        resp.status().as_u16(),
        400,
        "empty q must hit autocomplete validator (proves /autocomplete wins over /{{id}})"
    );
}

#[actix_web::test]
async fn patient_autocomplete_rejects_short_query() {
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(stub_db()))
            .service(web::scope("/v1").configure(configure_patients)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/v1/patients/autocomplete?q=a")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400);
}

fn stub_db() -> sea_orm::DatabaseConnection {
    use sea_orm::{DatabaseBackend, MockDatabase};
    MockDatabase::new(DatabaseBackend::Postgres).into_connection()
}
