use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct AppointmentResponse {
    pub id: String,
    pub date: String,
    pub time: String,
    pub dentist: String,
    pub reason: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAppointmentRequest {
    pub patient_name: String,
    pub patient_phone: Option<String>,
    pub patient_email: Option<String>,
    pub dentist_id: i64,
    pub appointment_date: String,
    pub appointment_time: String,
    pub duration: Option<i32>,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAppointmentRequest {
    pub patient_name: Option<String>,
    pub patient_phone: Option<String>,
    pub patient_email: Option<String>,
    pub dentist_id: Option<i64>,
    pub appointment_date: Option<String>,
    pub appointment_time: Option<String>,
    pub duration: Option<i32>,
    pub status: Option<String>,
    pub reason: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListAppointmentsQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
    pub date: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub dentist_id: Option<i64>,
    pub status: Option<String>,
}

fn default_page() -> i64 {
    1
}

fn default_limit() -> i64 {
    20
}

#[derive(Debug, Serialize)]
pub struct ListAppointmentsResponse {
    pub data: Vec<AppointmentResponse>,
    pub pagination: Pagination,
}

#[derive(Debug, Serialize)]
pub struct Pagination {
    pub page: i64,
    pub limit: i64,
    pub total: i64,
    pub total_pages: i64,
}