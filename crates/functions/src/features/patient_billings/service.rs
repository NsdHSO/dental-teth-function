use http_response::{CustomError, HttpCodeW};
use models::dto::patient_billing::{
    ActiveModel as BillingActiveModel, Column, Entity as Billing, Model as BillingModel,
};
use models::internal::Pagination;
use models::internal::patient_billing::{
    BillingResponse, CreateBillingRequest, ListBillingsResponse, UpdateBillingRequest,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, Set,
};

use crate::features::patients::service::PatientService;

pub struct PatientBillingService;

impl PatientBillingService {
    pub async fn create<C: ConnectionTrait>(
        db: &C,
        patient_id: i64,
        request: CreateBillingRequest,
    ) -> Result<BillingResponse, CustomError> {
        let _ = PatientService::get_by_id(db, patient_id).await?;
        validate_amount(request.amount_cents)?;
        let am = build_billing_active_model(patient_id, request);
        let inserted = am.insert(db).await?;
        Ok(to_billing_response(inserted))
    }

    pub async fn list<C: ConnectionTrait>(
        db: &C,
        patient_id: i64,
        page: i64,
        limit: i64,
        status: Option<String>,
    ) -> Result<ListBillingsResponse, CustomError> {
        let _ = PatientService::get_by_id(db, patient_id).await?;
        let (page, limit) = clamp_billing_page(page, limit);
        let (data, total) = paginate_billings_filtered(db, patient_id, status, page, limit).await?;
        Ok(build_billing_list_response(data, page, limit, total))
    }

    pub async fn get_by_id<C: ConnectionTrait>(
        db: &C,
        billing_id: i64,
    ) -> Result<BillingResponse, CustomError> {
        let row = find_billing(db, billing_id).await?;
        Ok(to_billing_response(row))
    }

    pub async fn update<C: ConnectionTrait>(
        db: &C,
        billing_id: i64,
        request: UpdateBillingRequest,
    ) -> Result<BillingResponse, CustomError> {
        let row = find_billing(db, billing_id).await?;
        let am = apply_billing_patch(row, &request);
        let updated = am.update(db).await?;
        Ok(to_billing_response(updated))
    }

    pub async fn delete<C: ConnectionTrait>(
        db: &C,
        billing_id: i64,
    ) -> Result<(), CustomError> {
        let row = find_billing(db, billing_id).await?;
        let am: BillingActiveModel = row.into();
        am.delete(db).await?;
        Ok(())
    }

    pub async fn mark_paid<C: ConnectionTrait>(
        db: &C,
        billing_id: i64,
    ) -> Result<BillingResponse, CustomError> {
        let row = find_billing(db, billing_id).await?;
        if row.status == "paid" { return Ok(to_billing_response(row)); }
        let am = build_paid_patch(row);
        let updated = am.update(db).await?;
        Ok(to_billing_response(updated))
    }
}

fn validate_amount(amount: i64) -> Result<(), CustomError> {
    if amount < 0 {
        return Err(CustomError::new(
            HttpCodeW::BadRequest,
            "amount_cents must be >= 0".to_string(),
        ));
    }
    Ok(())
}

fn build_billing_active_model(patient_id: i64, req: CreateBillingRequest) -> BillingActiveModel {
    let now = chrono::Utc::now().naive_utc();
    BillingActiveModel {
        patient_id: Set(patient_id),
        appointment_id: Set(req.appointment_id),
        amount_cents: Set(req.amount_cents),
        currency: Set(req.currency.unwrap_or_else(|| "RON".to_string())),
        status: Set(req.status.unwrap_or_else(|| "draft".to_string())),
        description: Set(req.description),
        paid_at: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    }
}

fn clamp_billing_page(page: i64, limit: i64) -> (i64, i64) {
    let p = if page < 1 { 1 } else { page };
    let l = if (1..=100).contains(&limit) { limit } else { 20 };
    (p, l)
}

async fn paginate_billings_filtered<C: ConnectionTrait>(
    db: &C,
    patient_id: i64,
    status: Option<String>,
    page: i64,
    limit: i64,
) -> Result<(Vec<BillingModel>, u64), CustomError> {
    let mut q = Billing::find().filter(Column::PatientId.eq(patient_id));
    if let Some(s) = status { q = q.filter(Column::Status.eq(s)); }
    let paginator = q.paginate(db, limit as u64);
    let total = paginator.num_items().await?;
    let data = paginator.fetch_page((page - 1) as u64).await?;
    Ok((data, total))
}

async fn find_billing<C: ConnectionTrait>(
    db: &C,
    billing_id: i64,
) -> Result<BillingModel, CustomError> {
    Billing::find_by_id(billing_id)
        .one(db)
        .await?
        .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "Billing not found".to_string()))
}

fn apply_billing_patch(row: BillingModel, req: &UpdateBillingRequest) -> BillingActiveModel {
    let mut am: BillingActiveModel = row.into();
    if let Some(v) = req.appointment_id { am.appointment_id = Set(Some(v)); }
    if let Some(v) = req.amount_cents { am.amount_cents = Set(v); }
    if let Some(v) = req.currency.clone() { am.currency = Set(v); }
    if let Some(v) = req.status.clone() { am.status = Set(v); }
    if let Some(v) = req.description.clone() { am.description = Set(Some(v)); }
    if let Some(v) = req.paid_at { am.paid_at = Set(Some(v)); }
    am.updated_at = Set(chrono::Utc::now().naive_utc());
    am
}

fn build_paid_patch(row: BillingModel) -> BillingActiveModel {
    let now = chrono::Utc::now().naive_utc();
    let mut am: BillingActiveModel = row.into();
    am.status = Set("paid".to_string());
    am.paid_at = Set(Some(now));
    am.updated_at = Set(now);
    am
}

fn build_billing_list_response(
    data: Vec<BillingModel>,
    page: i64,
    limit: i64,
    total: u64,
) -> ListBillingsResponse {
    let total_pages = (total as f64 / limit as f64).ceil() as i64;
    ListBillingsResponse {
        data: data.into_iter().map(to_billing_response).collect(),
        pagination: Pagination { page, limit, total: total as i64, total_pages },
    }
}

fn to_billing_response(row: BillingModel) -> BillingResponse {
    BillingResponse {
        id: row.id, patient_id: row.patient_id, appointment_id: row.appointment_id,
        amount_cents: row.amount_cents, currency: row.currency, status: row.status,
        description: row.description, paid_at: row.paid_at,
        created_at: row.created_at, updated_at: row.updated_at,
    }
}
