use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct DentistResponse {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub specialty: Option<String>,
    pub license_number: Option<String>,
    pub photo_url: Option<String>,
    pub bio: Option<String>,
    pub is_available: bool,
    pub consultation_duration: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateDentistRequest {
    pub user_id: i64,
    pub specialty: Option<String>,
    pub license_number: Option<String>,
    pub photo_url: Option<String>,
    pub bio: Option<String>,
    pub is_available: Option<bool>,
    pub consultation_duration: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDentistRequest {
    pub user_id: Option<i64>,
    pub specialty: Option<String>,
    pub license_number: Option<String>,
    pub photo_url: Option<String>,
    pub bio: Option<String>,
    pub is_available: Option<bool>,
    pub consultation_duration: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ListDentistsQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
    pub specialty: Option<String>,
}

fn default_page() -> i64 {
    1
}

fn default_limit() -> i64 {
    20
}

#[derive(Debug, Serialize)]
pub struct ListDentistsResponse {
    pub data: Vec<DentistResponse>,
    pub pagination: Pagination,
}

#[derive(Debug, Serialize)]
pub struct Pagination {
    pub page: i64,
    pub limit: i64,
    pub total: i64,
    pub total_pages: i64,
}