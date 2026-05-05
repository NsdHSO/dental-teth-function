use http_response::{CustomError, HttpCodeW};
use models::dto::dentist::{
    ActiveModel as DentistActiveModel, Column, Entity as Dentist, Model as DentistModel,
};
use models::internal::autocomplete::DentistAutocompleteItem;
use models::internal::dentist::{
    CreateDentistRequest, DentistResponse, ListDentistsResponse, UpdateDentistRequest,
};
use models::internal::Pagination as DentistPagination;
use sea_orm::DatabaseBackend::Postgres;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, FromQueryResult, PaginatorTrait,
    QueryFilter, Set, Statement,
};

use crate::features::user_profiles::service::UserProfileService;
use crate::features::users::service::UserService;

pub struct DentistService;

#[derive(FromQueryResult)]
struct DentistAutocompleteRow {
    dentist_id: i64,
    user_id: i64,
    full_name: Option<String>,
    specialty: Option<String>,
    is_available: bool,
    score: f32,
}

impl DentistService {
    pub async fn create<C: ConnectionTrait>(
        db: &C,
        request: CreateDentistRequest,
    ) -> Result<DentistResponse, CustomError> {
        let _user = UserService::get_by_id(db, request.user_id).await?;
        let am = build_dentist_active_model(&request, request.user_id);
        let dentist = am.insert(db).await?;
        dentist_summary(db, dentist).await
    }

    pub async fn get_by_id<C: ConnectionTrait>(
        db: &C,
        id: i64,
    ) -> Result<DentistResponse, CustomError> {
        let dentist = find_dentist(db, id).await?;
        dentist_summary(db, dentist).await
    }

    pub async fn update<C: ConnectionTrait>(
        db: &C,
        id: i64,
        request: UpdateDentistRequest,
    ) -> Result<DentistResponse, CustomError> {
        let dentist = find_dentist(db, id).await?;
        let mut am: DentistActiveModel = dentist.into();
        apply_dentist_patch(&mut am, request);
        let updated = am.update(db).await?;
        dentist_summary(db, updated).await
    }

    pub async fn delete<C: ConnectionTrait>(db: &C, id: i64) -> Result<(), CustomError> {
        let dentist = find_dentist(db, id).await?;
        let am: DentistActiveModel = dentist.into();
        am.delete(db).await?;
        Ok(())
    }

    pub async fn list<C: ConnectionTrait>(
        db: &C,
        page: i64,
        limit: i64,
        specialty: Option<String>,
    ) -> Result<ListDentistsResponse, CustomError> {
        let (page, limit) = clamp_page(page, limit);
        let (data, total) = paginate_dentists(db, page, limit, specialty).await?;
        let results = hydrate_all(db, data).await?;
        Ok(build_list_response(results, page, limit, total))
    }

    /// Autocomplete via the Postgres function. Logic stays in SQL.
    pub async fn autocomplete<C: ConnectionTrait>(
        db: &C,
        q: &str,
        limit: i32,
    ) -> Result<Vec<DentistAutocompleteItem>, CustomError> {
        validate_autocomplete_query(q)?;
        let limit = limit.clamp(1, 50);
        let rows = run_search_dentists(db, q, limit).await?;
        Ok(rows.into_iter().map(row_to_item).collect())
    }
}

fn build_dentist_active_model(req: &CreateDentistRequest, user_id: i64) -> DentistActiveModel {
    let now = chrono::Utc::now().naive_utc();
    DentistActiveModel {
        user_id: Set(user_id), specialty: Set(req.specialty.clone()),
        license_number: Set(req.license_number.clone()), photo_url: Set(req.photo_url.clone()),
        bio: Set(req.bio.clone()), is_available: Set(req.is_available.unwrap_or(true)),
        consultation_duration: Set(req.consultation_duration.unwrap_or(30)),
        created_at: Set(now), updated_at: Set(now), ..Default::default()
    }
}

fn apply_dentist_patch(am: &mut DentistActiveModel, req: UpdateDentistRequest) {
    if let Some(s) = req.specialty { am.specialty = Set(Some(s)); }
    if let Some(l) = req.license_number { am.license_number = Set(Some(l)); }
    if let Some(p) = req.photo_url { am.photo_url = Set(Some(p)); }
    if let Some(b) = req.bio { am.bio = Set(Some(b)); }
    if let Some(a) = req.is_available { am.is_available = Set(a); }
    if let Some(d) = req.consultation_duration { am.consultation_duration = Set(d); }
    am.updated_at = Set(chrono::Utc::now().naive_utc());
}

async fn dentist_summary<C: ConnectionTrait>(
    db: &C,
    dentist: DentistModel,
) -> Result<DentistResponse, CustomError> {
    let profile = UserProfileService::get_by_user_id(db, dentist.user_id).await.ok();
    Ok(build_dentist_response(dentist, profile.as_ref()))
}

fn build_dentist_response(
    dentist: DentistModel,
    profile: Option<&models::dto::user_profile::Model>,
) -> DentistResponse {
    let name = profile.and_then(|p| p.full_name.clone()).unwrap_or_else(|| "Unknown".to_string());
    DentistResponse {
        id: dentist.id, user_id: dentist.user_id, name,
        email: profile.and_then(|p| p.email.clone()), phone: profile.and_then(|p| p.phone.clone()),
        specialty: dentist.specialty, license_number: dentist.license_number,
        photo_url: dentist.photo_url, bio: dentist.bio, is_available: dentist.is_available,
        consultation_duration: dentist.consultation_duration,
        created_at: dentist.created_at, updated_at: dentist.updated_at,
    }
}

fn validate_autocomplete_query(q: &str) -> Result<(), CustomError> {
    if q.trim().chars().count() < 2 {
        return Err(CustomError::new(
            HttpCodeW::BadRequest,
            "Query string `q` must be at least 2 characters".to_string(),
        ));
    }
    Ok(())
}

fn clamp_page(page: i64, limit: i64) -> (i64, i64) {
    let p = if page < 1 { 1 } else { page };
    let l = if (1..=100).contains(&limit) { limit } else { 20 };
    (p, l)
}

async fn find_dentist<C: ConnectionTrait>(db: &C, id: i64) -> Result<DentistModel, CustomError> {
    Dentist::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "Dentist not found".to_string()))
}

async fn paginate_dentists<C: ConnectionTrait>(
    db: &C,
    page: i64,
    limit: i64,
    specialty: Option<String>,
) -> Result<(Vec<DentistModel>, u64), CustomError> {
    let mut query = Dentist::find();
    if let Some(s) = specialty { query = query.filter(Column::Specialty.eq(s)); }
    let paginator = query.paginate(db, limit as u64);
    let total = paginator.num_items().await?;
    let data = paginator.fetch_page((page - 1) as u64).await?;
    Ok((data, total))
}

async fn hydrate_all<C: ConnectionTrait>(
    db: &C,
    data: Vec<DentistModel>,
) -> Result<Vec<DentistResponse>, CustomError> {
    let mut results = Vec::with_capacity(data.len());
    for d in data { results.push(dentist_summary(db, d).await?); }
    Ok(results)
}

fn build_list_response(
    data: Vec<DentistResponse>,
    page: i64,
    limit: i64,
    total: u64,
) -> ListDentistsResponse {
    let total_pages = (total as f64 / limit as f64).ceil() as i64;
    ListDentistsResponse {
        data,
        pagination: DentistPagination { page, limit, total: total as i64, total_pages },
    }
}

async fn run_search_dentists<C: ConnectionTrait>(
    db: &C,
    q: &str,
    limit: i32,
) -> Result<Vec<DentistAutocompleteRow>, CustomError> {
    let stmt = Statement::from_sql_and_values(
        Postgres,
        "SELECT * FROM dental.search_dentists($1, $2)",
        [q.into(), limit.into()],
    );
    Ok(DentistAutocompleteRow::find_by_statement(stmt).all(db).await?)
}

fn row_to_item(r: DentistAutocompleteRow) -> DentistAutocompleteItem {
    DentistAutocompleteItem {
        dentist_id: r.dentist_id,
        user_id: r.user_id,
        full_name: r.full_name,
        specialty: r.specialty,
        is_available: r.is_available,
        score: r.score,
    }
}
