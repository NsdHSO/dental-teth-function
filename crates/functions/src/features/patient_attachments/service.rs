use http_response::{CustomError, HttpCodeW};
use models::dto::patient_attachment::{
    ActiveModel as AttachmentActiveModel, Column, Entity as Attachment, Model as AttachmentModel,
};
use models::internal::Pagination;
use models::internal::patient_attachment::{
    AttachmentResponse, CreateAttachmentRequest, ListAttachmentsResponse,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, Set,
};

use crate::features::patients::service::PatientService;

pub struct PatientAttachmentService;

impl PatientAttachmentService {
    pub async fn create<C: ConnectionTrait>(
        db: &C,
        patient_id: i64,
        request: CreateAttachmentRequest,
    ) -> Result<AttachmentResponse, CustomError> {
        let _ = PatientService::get_by_id(db, patient_id).await?;
        let am = build_attachment_active_model(patient_id, request);
        let inserted = am.insert(db).await?;
        Ok(to_attachment_response(inserted))
    }

    pub async fn list<C: ConnectionTrait>(
        db: &C,
        patient_id: i64,
        page: i64,
        limit: i64,
    ) -> Result<ListAttachmentsResponse, CustomError> {
        let _ = PatientService::get_by_id(db, patient_id).await?;
        let (page, limit) = clamp_attachment_page(page, limit);
        let paginator = Attachment::find()
            .filter(Column::PatientId.eq(patient_id))
            .paginate(db, limit as u64);
        let total = paginator.num_items().await?;
        let data = paginator.fetch_page((page - 1) as u64).await?;
        Ok(build_attachment_list_response(data, page, limit, total))
    }

    pub async fn get_by_id<C: ConnectionTrait>(
        db: &C,
        attachment_id: i64,
    ) -> Result<AttachmentResponse, CustomError> {
        let row = find_attachment(db, attachment_id).await?;
        Ok(to_attachment_response(row))
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

fn build_attachment_active_model(patient_id: i64, req: CreateAttachmentRequest) -> AttachmentActiveModel {
    let now = chrono::Utc::now().naive_utc();
    AttachmentActiveModel {
        patient_id: Set(patient_id), kind: Set(req.kind),
        storage_url: Set(req.storage_url), mime_type: Set(req.mime_type),
        size_bytes: Set(req.size_bytes), original_filename: Set(req.original_filename),
        uploaded_by: Set(req.uploaded_by), created_at: Set(now),
        ..Default::default()
    }
}

fn clamp_attachment_page(page: i64, limit: i64) -> (i64, i64) {
    let p = if page < 1 { 1 } else { page };
    let l = if (1..=200).contains(&limit) { limit } else { 50 };
    (p, l)
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

fn build_attachment_list_response(
    data: Vec<AttachmentModel>,
    page: i64,
    limit: i64,
    total: u64,
) -> ListAttachmentsResponse {
    let total_pages = (total as f64 / limit as f64).ceil() as i64;
    ListAttachmentsResponse {
        data: data.into_iter().map(to_attachment_response).collect(),
        pagination: Pagination { page, limit, total: total as i64, total_pages },
    }
}

fn to_attachment_response(row: AttachmentModel) -> AttachmentResponse {
    AttachmentResponse {
        id: row.id,
        patient_id: row.patient_id,
        kind: row.kind,
        storage_url: row.storage_url,
        mime_type: row.mime_type,
        size_bytes: row.size_bytes,
        original_filename: row.original_filename,
        uploaded_by: row.uploaded_by,
        created_at: row.created_at,
    }
}
