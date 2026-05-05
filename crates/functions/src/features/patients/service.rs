use http_response::{CustomError, HttpCodeW};
use models::dto::patient::{
    ActiveModel as PatientActiveModel, Entity as Patient, Model as PatientModel,
};
use models::dto::user::Model as UserModel;
use models::dto::user_profile::Model as UserProfileModel;
use models::internal::autocomplete::PatientAutocompleteItem;
use models::internal::patient::{
    CreatePatientRequest, ListPatientsResponse, PatientResponse, UpdatePatientRequest,
};
use models::internal::Pagination;
use sea_orm::DatabaseBackend::Postgres;
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DatabaseConnection, EntityTrait, FromQueryResult,
    PaginatorTrait, Set, Statement, TransactionTrait,
};

use crate::features::user_profiles::service::UserProfileService;
use crate::features::users::service::UserService;

pub struct PatientService;

#[derive(FromQueryResult)]
struct PatientAutocompleteRow {
    patient_id: i64,
    user_id: i64,
    full_name: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    cnp: Option<String>,
    score: f32,
}

impl PatientService {
    pub async fn create(
        db: &DatabaseConnection,
        request: CreatePatientRequest,
    ) -> Result<PatientResponse, CustomError> {
        Ok(db
            .transaction::<_, PatientResponse, CustomError>(|txn| {
                Box::pin(Self::create_in_txn(txn, request))
            })
            .await?)
    }

    async fn create_in_txn<C: ConnectionTrait>(
        txn: &C,
        request: CreatePatientRequest,
    ) -> Result<PatientResponse, CustomError> {
        let user = UserService::find_or_create_for_patient_seed(txn).await?;
        upsert_identity_for(txn, &user, &request).await?;
        let am = build_patient_active_model(&request, user.id);
        let patient = am.insert(txn).await?;
        Self::hydrate(txn, patient).await
    }

    pub async fn get_by_id<C: ConnectionTrait>(
        db: &C,
        id: i64,
    ) -> Result<PatientResponse, CustomError> {
        let patient = find_patient(db, id).await?;
        Self::hydrate(db, patient).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i64,
        request: UpdatePatientRequest,
    ) -> Result<PatientResponse, CustomError> {
        Ok(db
            .transaction::<_, PatientResponse, CustomError>(|txn| {
                Box::pin(Self::update_in_txn(txn, id, request))
            })
            .await?)
    }

    async fn update_in_txn<C: ConnectionTrait>(
        txn: &C,
        id: i64,
        request: UpdatePatientRequest,
    ) -> Result<PatientResponse, CustomError> {
        let patient = find_patient(txn, id).await?;
        let user = UserService::get_by_id(txn, patient.user_id).await?;
        upsert_identity_if_provided(txn, &user, &request).await?;
        let updated = apply_patient_patch(patient, &request).update(txn).await?;
        Self::hydrate(txn, updated).await
    }

    pub async fn delete<C: ConnectionTrait>(db: &C, id: i64) -> Result<(), CustomError> {
        let patient = find_patient(db, id).await?;
        let am: PatientActiveModel = patient.into();
        am.delete(db).await?;
        Ok(())
    }

    pub async fn list<C: ConnectionTrait>(
        db: &C,
        page: i64,
        limit: i64,
    ) -> Result<ListPatientsResponse, CustomError> {
        let (page, limit) = clamp_page(page, limit);
        let paginator = Patient::find().paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let data = paginator.fetch_page((page - 1) as u64).await?;
        let results = Self::hydrate_all(db, data).await?;
        Ok(build_list_response(results, page, limit, total))
    }

    async fn hydrate_all<C: ConnectionTrait>(
        db: &C,
        patients: Vec<PatientModel>,
    ) -> Result<Vec<PatientResponse>, CustomError> {
        let mut results = Vec::with_capacity(patients.len());
        for p in patients {
            results.push(Self::hydrate(db, p).await?);
        }
        Ok(results)
    }

    pub async fn autocomplete<C: ConnectionTrait>(
        db: &C,
        q: &str,
        limit: i32,
    ) -> Result<Vec<PatientAutocompleteItem>, CustomError> {
        validate_autocomplete_query(q)?;
        let limit = limit.clamp(1, 50);
        let rows = run_search_patients(db, q, limit).await?;
        Ok(rows.into_iter().map(row_to_item).collect())
    }

    async fn hydrate<C: ConnectionTrait>(
        db: &C,
        patient: PatientModel,
    ) -> Result<PatientResponse, CustomError> {
        let profile = UserProfileService::get_by_user_id(db, patient.user_id)
            .await
            .ok();
        Ok(build_patient_response(patient, profile))
    }
}

fn build_patient_active_model(req: &CreatePatientRequest, user_id: i64) -> PatientActiveModel {
    let now = chrono::Utc::now().naive_utc();
    PatientActiveModel {
        user_id: Set(user_id),
        medical_notes: Set(req.medical_notes.clone()),
        allergies: Set(req.allergies.clone()),
        created_at: Set(now), updated_at: Set(now),
        ..Default::default()
    }
}

async fn upsert_identity_for<C: ConnectionTrait>(
    txn: &C,
    user: &UserModel,
    req: &CreatePatientRequest,
) -> Result<(), CustomError> {
    UserProfileService::upsert_identity(
        txn, &user.auth_user_id, user.id,
        Some(req.full_name.clone()), req.phone.clone(),
        req.email.clone(), req.cnp.clone(),
    ).await?;
    Ok(())
}

async fn upsert_identity_if_provided<C: ConnectionTrait>(
    txn: &C,
    user: &UserModel,
    req: &UpdatePatientRequest,
) -> Result<(), CustomError> {
    if req.full_name.is_none() && req.phone.is_none() && req.email.is_none() && req.cnp.is_none() {
        return Ok(());
    }
    UserProfileService::upsert_identity(
        txn, &user.auth_user_id, user.id,
        req.full_name.clone(), req.phone.clone(),
        req.email.clone(), req.cnp.clone(),
    ).await?;
    Ok(())
}

fn apply_patient_patch(patient: PatientModel, req: &UpdatePatientRequest) -> PatientActiveModel {
    let mut am: PatientActiveModel = patient.into();
    if let Some(notes) = req.medical_notes.clone() { am.medical_notes = Set(Some(notes)); }
    if let Some(all) = req.allergies.clone() { am.allergies = Set(Some(all)); }
    am.updated_at = Set(chrono::Utc::now().naive_utc());
    am
}

async fn find_patient<C: ConnectionTrait>(
    db: &C,
    id: i64,
) -> Result<PatientModel, CustomError> {
    Patient::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "Patient not found".to_string()))
}

fn clamp_page(page: i64, limit: i64) -> (i64, i64) {
    let p = if page < 1 { 1 } else { page };
    let l = if (1..=100).contains(&limit) { limit } else { 20 };
    (p, l)
}

fn build_list_response(
    data: Vec<PatientResponse>,
    page: i64,
    limit: i64,
    total: u64,
) -> ListPatientsResponse {
    let total_pages = (total as f64 / limit as f64).ceil() as i64;
    ListPatientsResponse {
        data,
        pagination: Pagination { page, limit, total: total as i64, total_pages },
    }
}

fn validate_autocomplete_query(q: &str) -> Result<(), CustomError> {
    if q.trim().is_empty() {
        return Err(CustomError::new(
            HttpCodeW::BadRequest,
            "Query string `q` is required".to_string(),
        ));
    }
    Ok(())
}

async fn run_search_patients<C: ConnectionTrait>(
    db: &C,
    q: &str,
    limit: i32,
) -> Result<Vec<PatientAutocompleteRow>, CustomError> {
    let stmt = Statement::from_sql_and_values(
        Postgres,
        "SELECT * FROM dental.search_patients($1, $2)",
        [q.into(), limit.into()],
    );
    Ok(PatientAutocompleteRow::find_by_statement(stmt).all(db).await?)
}

fn row_to_item(r: PatientAutocompleteRow) -> PatientAutocompleteItem {
    PatientAutocompleteItem {
        patient_id: r.patient_id,
        user_id: r.user_id,
        full_name: r.full_name,
        phone: r.phone,
        email: r.email,
        cnp: r.cnp,
        score: r.score,
    }
}

fn build_patient_response(patient: PatientModel, profile: Option<UserProfileModel>) -> PatientResponse {
    let p = profile.as_ref();
    PatientResponse {
        id: patient.id, user_id: patient.user_id,
        full_name: p.and_then(|x| x.full_name.clone()),
        phone: p.and_then(|x| x.phone.clone()),
        email: p.and_then(|x| x.email.clone()),
        cnp: p.and_then(|x| x.cnp.clone()),
        medical_notes: patient.medical_notes, allergies: patient.allergies,
        created_at: patient.created_at, updated_at: patient.updated_at,
    }
}
