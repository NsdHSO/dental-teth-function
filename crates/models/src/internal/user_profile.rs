use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct UserProfileResponse {
    pub id: i64,
    pub user_sub: String,
    pub schema_version: String,
    pub attributes: serde_json::Value,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserProfileRequest {
    pub attributes: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserProfileRequest {
    pub attributes: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct UserProfileListResponse {
    pub data: Vec<UserProfileResponse>,
    pub pagination: Pagination,
}

#[derive(Debug, Serialize)]
pub struct Pagination {
    pub page: i64,
    pub limit: i64,
    pub total: i64,
    pub total_pages: i64,
}