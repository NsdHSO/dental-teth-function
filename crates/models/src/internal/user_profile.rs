use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct UserProfileResponse {
    pub id: i64,
    pub user_sub: String,
    pub user_id: i64,
    pub schema_version: String,
    pub attributes: serde_json::Value,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub cnp: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Default)]
pub struct CreateUserProfileRequest {
    pub attributes: Option<serde_json::Value>,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub cnp: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct UpdateUserProfileRequest {
    pub attributes: Option<serde_json::Value>,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub cnp: Option<String>,
}
