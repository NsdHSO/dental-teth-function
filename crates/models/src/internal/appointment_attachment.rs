use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct AppointmentAttachmentResponse {
    pub id: i64,
    pub appointment_id: i64,
    pub filename: Option<String>,
    pub mime_type: String,
    pub size_bytes: i64,
    pub uploaded_by: Option<i64>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct ListAppointmentAttachmentsQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_page() -> i64 {
    1
}
fn default_limit() -> i64 {
    50
}

#[derive(Debug, Serialize)]
pub struct ListAppointmentAttachmentsResponse {
    pub data: Vec<AppointmentAttachmentResponse>,
    pub pagination: crate::internal::Pagination,
}
