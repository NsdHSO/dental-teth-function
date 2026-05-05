use http_response::{CustomError, HttpCodeW};
use models::dto::appointment::Entity as Appointment;
use models::dto::appointment_attachment::{
    ActiveModel as AttachmentActiveModel, Column, Entity as Attachment, Model as AttachmentModel,
};
use models::internal::Pagination;
use models::internal::appointment_attachment::{
    AppointmentAttachmentResponse, ListAppointmentAttachmentsResponse,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, Set,
};

pub struct AppointmentAttachmentService;

impl AppointmentAttachmentService {
    pub async fn create<C: ConnectionTrait>(
        db: &C,
        appointment_id: i64,
        filename: Option<String>,
        mime_type: String,
        file_data: Vec<u8>,
        uploaded_by: Option<i64>,
    ) -> Result<AppointmentAttachmentResponse, CustomError> {
        let _ = find_appointment(db, appointment_id).await?;
        let size = file_data.len() as i64;
        let am = build_attachment_active_model(appointment_id, filename, mime_type, file_data, size, uploaded_by);
        let inserted = am.insert(db).await?;
        Ok(to_attachment_response(inserted))
    }

    pub async fn list<C: ConnectionTrait>(
        db: &C,
        appointment_id: i64,
        page: i64,
        limit: i64,
    ) -> Result<ListAppointmentAttachmentsResponse, CustomError> {
        let _ = find_appointment(db, appointment_id).await?;
        let (page, limit) = clamp_page(page, limit);
        let paginator = Attachment::find()
            .filter(Column::AppointmentId.eq(appointment_id))
            .paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let data = paginator.fetch_page((page - 1) as u64).await?;
        Ok(build_list_response(data, page, limit, total))
    }

    pub async fn get_by_id<C: ConnectionTrait>(
        db: &C,
        attachment_id: i64,
    ) -> Result<AppointmentAttachmentResponse, CustomError> {
        let row = find_attachment(db, attachment_id).await?;
        Ok(to_attachment_response(row))
    }

    pub async fn download<C: ConnectionTrait>(
        db: &C,
        attachment_id: i64,
    ) -> Result<(Vec<u8>, String, Option<String>), CustomError> {
        let row = find_attachment(db, attachment_id).await?;
        Ok((row.file_data, row.mime_type, row.filename))
    }

    pub async fn delete<C: ConnectionTrait>(
        db: &C,
        attachment_id: i64,
    ) -> Result<(), CustomError> {
        let row = find_attachment(db, attachment_id).await?;
        let am: AttachmentActiveModel = row.into();
        am.delete(db).await?;
        Ok(())
    }
}

fn build_attachment_active_model(
    appointment_id: i64,
    filename: Option<String>,
    mime_type: String,
    file_data: Vec<u8>,
    size_bytes: i64,
    uploaded_by: Option<i64>,
) -> AttachmentActiveModel {
    let now = chrono::Utc::now().naive_utc();
    AttachmentActiveModel {
        appointment_id: Set(appointment_id),
        filename: Set(filename),
        mime_type: Set(mime_type),
        file_data: Set(file_data),
        size_bytes: Set(size_bytes),
        uploaded_by: Set(uploaded_by),
        created_at: Set(now),
        ..Default::default()
    }
}

fn clamp_page(page: i64, limit: i64) -> (i64, i64) {
    let p = if page < 1 { 1 } else { page };
    let l = if (1..=200).contains(&limit) { limit } else { 50 };
    (p, l)
}

async fn find_appointment<C: ConnectionTrait>(
    db: &C,
    id: i64,
) -> Result<models::dto::appointment::Model, CustomError> {
    Appointment::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "Appointment not found".to_string()))
}

async fn find_attachment<C: ConnectionTrait>(
    db: &C,
    attachment_id: i64,
) -> Result<AttachmentModel, CustomError> {
    Attachment::find_by_id(attachment_id)
        .one(db)
        .await?
        .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "Attachment not found".to_string()))
}

fn build_list_response(
    data: Vec<AttachmentModel>,
    page: i64,
    limit: i64,
    total: u64,
) -> ListAppointmentAttachmentsResponse {
    let total_pages = (total as f64 / limit as f64).ceil() as i64;
    ListAppointmentAttachmentsResponse {
        data: data.into_iter().map(to_attachment_response).collect(),
        pagination: Pagination {
            page,
            limit,
            total: total as i64,
            total_pages,
        },
    }
}

fn to_attachment_response(row: AttachmentModel) -> AppointmentAttachmentResponse {
    AppointmentAttachmentResponse {
        id: row.id,
        appointment_id: row.appointment_id,
        filename: row.filename,
        mime_type: row.mime_type,
        size_bytes: row.size_bytes,
        uploaded_by: row.uploaded_by,
        created_at: row.created_at,
    }
}
