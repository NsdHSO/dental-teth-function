use http_response::{CustomError, HttpCodeW};
use models::dto::appointment::{
    ActiveModel as AppointmentActiveModel, Column as AppointmentColumn, Entity as Appointment,
    Model as AppointmentModel,
};
use models::internal::appointment::{
    AppointmentPatientSummary, AppointmentResponse, CreateAppointmentRequest,
    ListAppointmentsResponse, UpdateAppointmentRequest,
};
use models::internal::Pagination as AppointmentPagination;
use models::internal::patient::PatientResponse;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};

use crate::features::dentists::service::DentistService;
use crate::features::patients::service::PatientService;

pub struct AppointmentService;

impl AppointmentService {
    pub async fn create(
        db: &DatabaseConnection,
        req: CreateAppointmentRequest,
    ) -> Result<AppointmentResponse, CustomError> {
        let date = parse_date(&req.appointment_date)?;
        let time = parse_time(&req.appointment_time)?;
        Ok(db
            .transaction::<_, AppointmentResponse, CustomError>(|txn| {
                Box::pin(Self::create_in_txn(txn, req, date, time))
            })
            .await?)
    }

    async fn create_in_txn<C: ConnectionTrait>(
        txn: &C,
        req: CreateAppointmentRequest,
        date: chrono::NaiveDate,
        time: chrono::NaiveTime,
    ) -> Result<AppointmentResponse, CustomError> {
        PatientService::get_by_id(txn, req.patient_id).await?;
        let dentist = DentistService::get_by_id(txn, req.dentist_id).await?;
        let am = build_appointment_active_model(&req, date, time);
        let appointment = am.insert(txn).await?;
        Self::hydrate(txn, appointment, dentist.name).await
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<AppointmentResponse, CustomError> {
        let appointment = find_appointment(db, id).await?;
        let dentist = DentistService::get_by_id(db, appointment.dentist_id).await?;
        Self::hydrate(db, appointment, dentist.name).await
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i64,
        req: UpdateAppointmentRequest,
    ) -> Result<AppointmentResponse, CustomError> {
        Ok(db
            .transaction::<_, AppointmentResponse, CustomError>(|txn| {
                Box::pin(Self::update_in_txn(txn, id, req))
            })
            .await?)
    }

    async fn update_in_txn<C: ConnectionTrait>(
        txn: &C,
        id: i64,
        req: UpdateAppointmentRequest,
    ) -> Result<AppointmentResponse, CustomError> {
        let appointment = find_appointment(txn, id).await?;
        if let Some(pid) = req.patient_id {
            PatientService::get_by_id(txn, pid).await?;
        }
        let dentist_id = req.dentist_id.unwrap_or(appointment.dentist_id);
        let dentist = DentistService::get_by_id(txn, dentist_id).await?;
        let mut am: AppointmentActiveModel = appointment.into();
        apply_appointment_patch(&mut am, &req)?;
        am.updated_at = Set(chrono::Utc::now().naive_utc());
        let updated = am.update(txn).await?;
        Self::hydrate(txn, updated, dentist.name).await
    }

    pub async fn delete(db: &DatabaseConnection, id: i64) -> Result<(), CustomError> {
        let appointment = find_appointment(db, id).await?;
        let active_model: AppointmentActiveModel = appointment.into();
        active_model.delete(db).await?;
        Ok(())
    }

    pub async fn list(
        db: &DatabaseConnection,
        page: i64,
        limit: i64,
        date: Option<String>,
        from: Option<String>,
        to: Option<String>,
        dentist_id: Option<i64>,
        patient_id: Option<i64>,
        status: Option<String>,
    ) -> Result<ListAppointmentsResponse, CustomError> {
        let (page, limit) = clamp_page(page, limit);
        let query =
            apply_appointment_filters(Appointment::find(), date, from, to, dentist_id, patient_id, status)?;
        let total = query.clone().count(db).await?;
        let appointments = query
            .order_by(AppointmentColumn::AppointmentDate, sea_orm::Order::Desc)
            .order_by(AppointmentColumn::AppointmentTime, sea_orm::Order::Desc)
            .paginate(db, limit as u64)
            .fetch_page((page - 1) as u64)
            .await?;
        let data = hydrate_all(db, appointments).await?;
        Ok(ListAppointmentsResponse {
            data,
            pagination: AppointmentPagination {
                page,
                limit,
                total: total as i64,
                total_pages: (total as f64 / limit as f64).ceil() as i64,
            },
        })
    }

    async fn hydrate<C: ConnectionTrait>(
        db: &C,
        appointment: AppointmentModel,
        dentist_name: String,
    ) -> Result<AppointmentResponse, CustomError> {
        let patient = PatientService::get_by_id(db, appointment.patient_id).await?;
        Ok(build_appointment_response(&appointment, dentist_name, patient))
    }
}

fn parse_date(s: &str) -> Result<chrono::NaiveDate, CustomError> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| CustomError::new(HttpCodeW::BadRequest, "Invalid date format. Use YYYY-MM-DD".into()))
}

fn parse_time(s: &str) -> Result<chrono::NaiveTime, CustomError> {
    chrono::NaiveTime::parse_from_str(s, "%H:%M")
        .map_err(|_| CustomError::new(HttpCodeW::BadRequest, "Invalid time format. Use HH:MM".into()))
}

fn build_appointment_active_model(
    req: &CreateAppointmentRequest,
    date: chrono::NaiveDate,
    time: chrono::NaiveTime,
) -> AppointmentActiveModel {
    let now = chrono::Utc::now().naive_utc();
    AppointmentActiveModel {
        patient_id: Set(req.patient_id), dentist_id: Set(req.dentist_id),
        appointment_date: Set(date), appointment_time: Set(time),
        duration: Set(req.duration.unwrap_or(30)), status: Set("scheduled".to_string()),
        reason: Set(req.reason.clone()), created_at: Set(now), updated_at: Set(now),
        ..Default::default()
    }
}

fn apply_appointment_patch(
    am: &mut AppointmentActiveModel,
    req: &UpdateAppointmentRequest,
) -> Result<(), CustomError> {
    if let Some(pid) = req.patient_id { am.patient_id = Set(pid); }
    if let Some(did) = req.dentist_id { am.dentist_id = Set(did); }
    if let Some(d) = &req.appointment_date { am.appointment_date = Set(parse_date(d)?); }
    if let Some(t) = &req.appointment_time { am.appointment_time = Set(parse_time(t)?); }
    if let Some(dur) = req.duration { am.duration = Set(dur); }
    if let Some(s) = &req.status { am.status = Set(s.clone()); }
    if let Some(r) = &req.reason { am.reason = Set(Some(r.clone())); }
    if let Some(n) = &req.notes { am.notes = Set(Some(n.clone())); }
    Ok(())
}

fn apply_appointment_filters(
    mut query: sea_orm::Select<Appointment>,
    date: Option<String>,
    from: Option<String>,
    to: Option<String>,
    dentist_id: Option<i64>,
    patient_id: Option<i64>,
    status: Option<String>,
) -> Result<sea_orm::Select<Appointment>, CustomError> {
    if let Some(d) = date { query = query.filter(AppointmentColumn::AppointmentDate.eq(parse_date(&d)?)); }
    if let Some(f) = from { query = query.filter(AppointmentColumn::AppointmentDate.gte(parse_date(&f)?)); }
    if let Some(t) = to { query = query.filter(AppointmentColumn::AppointmentDate.lte(parse_date(&t)?)); }
    if let Some(did) = dentist_id { query = query.filter(AppointmentColumn::DentistId.eq(did)); }
    if let Some(pid) = patient_id { query = query.filter(AppointmentColumn::PatientId.eq(pid)); }
    if let Some(s) = status { query = query.filter(AppointmentColumn::Status.eq(s)); }
    Ok(query)
}

fn clamp_page(page: i64, limit: i64) -> (i64, i64) {
    let p = if page < 1 { 1 } else { page };
    let l = if limit < 1 || limit > 100 { 20 } else { limit };
    (p, l)
}

async fn find_appointment<C: ConnectionTrait>(
    db: &C,
    id: i64,
) -> Result<AppointmentModel, CustomError> {
    Appointment::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "Appointment not found".to_string()))
}

async fn dentist_display_name_or_unknown<C: ConnectionTrait>(db: &C, dentist_id: i64) -> String {
    DentistService::get_by_id(db, dentist_id)
        .await
        .map(|d| d.name)
        .unwrap_or_else(|_| "Unknown".to_string())
}

async fn hydrate_all<C: ConnectionTrait>(
    db: &C,
    appointments: Vec<AppointmentModel>,
) -> Result<Vec<AppointmentResponse>, CustomError> {
    let mut results = Vec::with_capacity(appointments.len());
    for a in appointments {
        let name = dentist_display_name_or_unknown(db, a.dentist_id).await;
        results.push(AppointmentService::hydrate(db, a, name).await?);
    }
    Ok(results)
}

fn build_appointment_response(
    a: &AppointmentModel,
    dentist_name: String,
    patient: PatientResponse,
) -> AppointmentResponse {
    let fmt = |d: chrono::NaiveDateTime| d.format("%Y-%m-%d %H:%M:%S").to_string();
    AppointmentResponse {
        id: a.id.to_string(), date: a.appointment_date.format("%Y-%m-%d").to_string(),
        time: a.appointment_time.format("%H:%M").to_string(), dentist: dentist_name,
        patient: AppointmentPatientSummary { id: patient.id, full_name: patient.full_name, phone: patient.phone, email: patient.email },
        reason: a.reason.clone(),
        created_at: Some(fmt(a.created_at)), updated_at: Some(fmt(a.updated_at)),
    }
}
