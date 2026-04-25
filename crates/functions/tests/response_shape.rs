use actix_web::{test, web, App, HttpResponse};
use functions::configure_health;
use http_response::{CustomError, HttpCodeW};

#[actix_rt::test]
async fn health_returns_response_object() {
    let app = test::init_service(App::new().configure(configure_health)).await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(
        body.get("message").is_some(),
        "missing message field: {body:?}"
    );
    assert!(body.get("code").is_some(), "missing code field: {body:?}");
}

async fn boom() -> Result<HttpResponse, CustomError> {
    Err(CustomError::new(HttpCodeW::NotFound, "nope".to_string()))
}

#[actix_rt::test]
async fn errors_are_response_object() {
    let app = test::init_service(App::new().route("/boom", web::get().to(boom))).await;

    let req = test::TestRequest::get().uri("/boom").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 404);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["code"], serde_json::json!("NotFound"));
    assert_eq!(body["message"], serde_json::json!("nope"));
}
