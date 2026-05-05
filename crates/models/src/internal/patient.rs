use serde::{Deserialize, Serialize};
use super::Pagination;

#[derive(Debug, Serialize)]
pub struct PatientResponse {
    pub id: i64,
    pub user_id: i64,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub cnp: Option<String>,
    pub medical_notes: Option<String>,
    pub allergies: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreatePatientRequest {
    pub full_name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub cnp: Option<String>,
    pub medical_notes: Option<String>,
    pub allergies: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePatientRequest {
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub cnp: Option<String>,
    pub medical_notes: Option<String>,
    pub allergies: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListPatientsQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_page() -> i64 { 1 }
fn default_limit() -> i64 { 20 }

#[derive(Debug, Serialize)]
pub struct ListPatientsResponse {
    pub data: Vec<PatientResponse>,
    pub pagination: Pagination,
}

