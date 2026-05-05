use functions::features::patients::service::PatientService;
use models::dto::user::Entity as User;
use models::internal::patient::CreatePatientRequest;
use sea_orm::{ColumnTrait, Database, EntityTrait, PaginatorTrait, QueryFilter};

#[tokio::test]
async fn patient_create_rolls_back_synthetic_user_when_profile_upsert_fails() {
    let url = std::env::var("DATABASE_URL_TEST")
        .expect("DATABASE_URL_TEST must point at an empty test DB");
    let db = Database::connect(&url).await.expect("connect");

    let bad = CreatePatientRequest {
        full_name: "Ion Popescu".to_string(),
        phone: Some("+40711222333".to_string()),
        email: Some("ion@example.test".to_string()),
        cnp: Some("TOO_SHORT".to_string()),
        medical_notes: None,
        allergies: None,
    };

    let before = User::find().count(&db).await.expect("count before");
    let res = PatientService::create(&db, bad).await;
    assert!(res.is_err(), "bad CNP must fail");

    let after = User::find().count(&db).await.expect("count after");
    assert_eq!(
        before, after,
        "synthetic user row must roll back with the failed profile upsert"
    );

    let orphans = User::find()
        .filter(models::dto::user::Column::AuthUserId.like("local:patient:%"))
        .count(&db)
        .await
        .expect("count orphans");
    assert_eq!(orphans, 0, "no synthetic patient users should exist");
}
