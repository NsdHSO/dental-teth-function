use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoleResponse {
    pub id: i64,
    pub user_id: i64,
    pub role_id: i64,
    pub role_name: String,
    pub assigned_date: chrono::NaiveDate,
    pub assigned_by: Option<i64>,
    pub is_active: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct AssignRoleRequest {
    pub role_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct RemoveRoleRequest {
    pub role_id: i64,
}
