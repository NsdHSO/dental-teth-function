use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct BillingResponse {
    pub id: i64,
    pub patient_id: i64,
    pub appointment_id: Option<i64>,
    pub amount_cents: i64,
    pub currency: String,
    pub status: String,
    pub description: Option<String>,
    pub paid_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateBillingRequest {
    pub appointment_id: Option<i64>,
    pub amount_cents: i64,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBillingRequest {
    pub appointment_id: Option<i64>,
    pub amount_cents: Option<i64>,
    pub currency: Option<String>,
    pub status: Option<String>,
    pub description: Option<String>,
    pub paid_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct ListBillingsQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
    pub status: Option<String>,
}

fn default_page() -> i64 { 1 }
fn default_limit() -> i64 { 20 }

#[derive(Debug, Serialize)]
pub struct ListBillingsResponse {
    pub data: Vec<BillingResponse>,
    pub pagination: crate::internal::Pagination,
}
