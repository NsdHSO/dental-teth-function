use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct AttachmentResponse {
    pub id: i64,
    pub patient_id: i64,
    pub kind: String,
    pub storage_url: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub original_filename: Option<String>,
    pub uploaded_by: Option<i64>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateAttachmentRequest {
    pub kind: String,
    pub storage_url: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub original_filename: Option<String>,
    pub uploaded_by: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ListAttachmentsQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_page() -> i64 { 1 }
fn default_limit() -> i64 { 50 }

#[derive(Debug, Serialize)]
pub struct ListAttachmentsResponse {
    pub data: Vec<AttachmentResponse>,
    pub pagination: crate::internal::Pagination,
}
