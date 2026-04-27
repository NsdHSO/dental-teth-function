use http_response::{CustomError, HttpCodeW};
use models::dto::appointment::{
    ActiveModel as AppointmentActiveModel, Column as AppointmentColumn, Entity as Appointment,
    Model as AppointmentModel,
};
use models::dto::dentist::Entity as Dentist;
use models::dto::user::Entity as User;
use models::internal::appointment::{
    AppointmentResponse, CreateAppointmentRequest, ListAppointmentsResponse,
    Pagination as AppointmentPagination, UpdateAppointmentRequest,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

pub struct AppointmentService;

impl AppointmentService {
    pub async fn create(
        db: &DatabaseConnection,
        request: CreateAppointmentRequest,
    ) -> Result<AppointmentResponse, CustomError> {
        let dentist = Dentist::find_by_id(request.dentist_id)
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(HttpCodeW::NotFound, "Dentist not found".to_string())
            })?;

        let user = User::find_by_id(dentist.user_id)
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(HttpCodeW::NotFound, "Dentist user not found".to_string())
            })?;

        let appointment_date = chrono::NaiveDate::parse_from_str(&request.appointment_date, "%Y-%m-%d")
            .map_err(|_| {
                CustomError::new(
                    HttpCodeW::BadRequest,
                    "Invalid date format. Use YYYY-MM-DD".to_string(),
                )
            })?;

        let appointment_time = chrono::NaiveTime::parse_from_str(&request.appointment_time, "%H:%M")
            .map_err(|_| {
                CustomError::new(
                    HttpCodeW::BadRequest,
                    "Invalid time format. Use HH:MM".to_string(),
                )
            })?;

        let now = chrono::Utc::now().naive_utc();
        let new_appointment = AppointmentActiveModel {
            patient_name: Set(request.patient_name),
            patient_phone: Set(request.patient_phone),
            patient_email: Set(request.patient_email),
            dentist_id: Set(request.dentist_id),
            appointment_date: Set(appointment_date),
            appointment_time: Set(appointment_time),
            duration: Set(request.duration.unwrap_or(30)),
            status: Set("scheduled".to_string()),
            reason: Set(request.reason),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let appointment = new_appointment.insert(db).await?;

        Ok(Self::to_response(appointment, user.auth_user_id))
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: i64,
    ) -> Result<AppointmentResponse, CustomError> {
        let appointment = Appointment::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(HttpCodeW::NotFound, "Appointment not found".to_string())
            })?;

        let dentist = Dentist::find_by_id(appointment.dentist_id)
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(HttpCodeW::NotFound, "Dentist not found".to_string())
            })?;

        let user = User::find_by_id(dentist.user_id)
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(HttpCodeW::NotFound, "Dentist user not found".to_string())
            })?;

        Ok(Self::to_response(appointment, user.auth_user_id))
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i64,
        request: UpdateAppointmentRequest,
    ) -> Result<AppointmentResponse, CustomError> {
        let appointment = Appointment::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(HttpCodeW::NotFound, "Appointment not found".to_string())
            })?;

        let dentist_id = request.dentist_id.unwrap_or(appointment.dentist_id);
        let dentist = Dentist::find_by_id(dentist_id)
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(HttpCodeW::NotFound, "Dentist not found".to_string())
            })?;

        let user = User::find_by_id(dentist.user_id)
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(HttpCodeW::NotFound, "Dentist user not found".to_string())
            })?;

        let now = chrono::Utc::now().naive_utc();
        let mut active_model: AppointmentActiveModel = appointment.into();

        if let Some(patient_name) = request.patient_name {
            active_model.patient_name = Set(patient_name);
        }
        if let Some(patient_phone) = request.patient_phone {
            active_model.patient_phone = Set(Some(patient_phone));
        }
        if let Some(patient_email) = request.patient_email {
            active_model.patient_email = Set(Some(patient_email));
        }
        if let Some(new_dentist_id) = request.dentist_id {
            active_model.dentist_id = Set(new_dentist_id);
        }
        if let Some(date_str) = request.appointment_date {
            let date = chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").map_err(|_| {
                CustomError::new(
                    HttpCodeW::BadRequest,
                    "Invalid date format. Use YYYY-MM-DD".to_string(),
                )
            })?;
            active_model.appointment_date = Set(date);
        }
        if let Some(time_str) = request.appointment_time {
            let time = chrono::NaiveTime::parse_from_str(&time_str, "%H:%M").map_err(|_| {
                CustomError::new(
                    HttpCodeW::BadRequest,
                    "Invalid time format. Use HH:MM".to_string(),
                )
            })?;
            active_model.appointment_time = Set(time);
        }
        if let Some(duration) = request.duration {
            active_model.duration = Set(duration);
        }
        if let Some(status) = request.status {
            active_model.status = Set(status);
        }
        if let Some(reason) = request.reason {
            active_model.reason = Set(Some(reason));
        }
        if let Some(notes) = request.notes {
            active_model.notes = Set(Some(notes));
        }
        active_model.updated_at = Set(now);

        let updated = active_model.update(db).await?;

        Ok(Self::to_response(updated, user.auth_user_id))
    }

    pub async fn delete(db: &DatabaseConnection, id: i64) -> Result<(), CustomError> {
        let appointment = Appointment::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(HttpCodeW::NotFound, "Appointment not found".to_string())
            })?;

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
        status: Option<String>,
    ) -> Result<ListAppointmentsResponse, CustomError> {
        let page = if page < 1 { 1 } else { page };
        let limit = if limit < 1 || limit > 100 { 20 } else { limit };

        let mut query = Appointment::find();

        if let Some(date_filter) = date {
            let parsed_date = chrono::NaiveDate::parse_from_str(&date_filter, "%Y-%m-%d")
                .map_err(|_| {
                    CustomError::new(
                        HttpCodeW::BadRequest,
                        "Invalid date format. Use YYYY-MM-DD".to_string(),
                    )
                })?;
            query = query.filter(AppointmentColumn::AppointmentDate.eq(parsed_date));
        }

        if let Some(from_date) = from {
            let parsed = chrono::NaiveDate::parse_from_str(&from_date, "%Y-%m-%d")
                .map_err(|_| {
                    CustomError::new(
                        HttpCodeW::BadRequest,
                        "Invalid from date format. Use YYYY-MM-DD".to_string(),
                    )
                })?;
            query = query.filter(AppointmentColumn::AppointmentDate.gte(parsed));
        }

        if let Some(to_date) = to {
            let parsed = chrono::NaiveDate::parse_from_str(&to_date, "%Y-%m-%d")
                .map_err(|_| {
                    CustomError::new(
                        HttpCodeW::BadRequest,
                        "Invalid to date format. Use YYYY-MM-DD".to_string(),
                    )
                })?;
            query = query.filter(AppointmentColumn::AppointmentDate.lte(parsed));
        }

        if let Some(d_id) = dentist_id {
            query = query.filter(AppointmentColumn::DentistId.eq(d_id));
        }

        if let Some(status_filter) = status {
            query = query.filter(AppointmentColumn::Status.eq(status_filter));
        }

        let total = query.clone().count(db).await?;
        let appointments = query
            .order_by(AppointmentColumn::AppointmentDate, sea_orm::Order::Desc)
            .order_by(AppointmentColumn::AppointmentTime, sea_orm::Order::Desc)
            .paginate(db, limit as u64)
            .fetch_page((page - 1) as u64)
            .await?;

        let mut results = Vec::new();
        for appointment in appointments {
            let dentist = Dentist::find_by_id(appointment.dentist_id)
                .one(db)
                .await?;
            let dentist_name = if let Some(d) = dentist {
                let user = User::find_by_id(d.user_id).one(db).await?;
                user.map(|u| u.auth_user_id).unwrap_or_else(|| "Unknown".to_string())
            } else {
                "Unknown".to_string()
            };
            results.push(Self::to_response(appointment, dentist_name));
        }

        let total_pages = (total as f64 / limit as f64).ceil() as i64;

        Ok(ListAppointmentsResponse {
            data: results,
            pagination: AppointmentPagination {
                page,
                limit,
                total: total as i64,
                total_pages,
            },
        })
    }

    fn to_response(appointment: AppointmentModel, dentist_name: String) -> AppointmentResponse {
        AppointmentResponse {
            id: appointment.id.to_string(),
            date: appointment.appointment_date.format("%Y-%m-%d").to_string(),
            time: appointment.appointment_time.format("%H:%M").to_string(),
            dentist: dentist_name,
            reason: appointment.reason,
            created_at: Some(appointment.created_at.format("%Y-%m-%d %H:%M:%S").to_string()),
            updated_at: Some(appointment.updated_at.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}