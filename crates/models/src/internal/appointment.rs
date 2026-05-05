use serde::{Deserialize, Serialize};
use super::Pagination;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppointmentPatientSummary {
    pub id: i64,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppointmentResponse {
    pub id: String,
    pub date: String,
    pub time: String,
    pub dentist: String,
    pub patient: AppointmentPatientSummary,
    pub reason: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateAppointmentRequest {
    pub patient_id: i64,
    pub dentist_id: i64,
    pub appointment_date: String,
    pub appointment_time: String,
    pub duration: Option<i32>,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateAppointmentRequest {
    pub patient_id: Option<i64>,
    pub dentist_id: Option<i64>,
    pub appointment_date: Option<String>,
    pub appointment_time: Option<String>,
    pub duration: Option<i32>,
    pub status: Option<String>,
    pub reason: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListAppointmentsResponse {
    pub data: Vec<AppointmentResponse>,
    pub pagination: Pagination,
}
