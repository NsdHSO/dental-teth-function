use serde::{Deserialize, Serialize};

// ==================== LINK USER ====================

/// Request to link an auth server user to church system
/// No fields needed - uses JWT sub automatically
#[derive(Debug, Deserialize)]
pub struct LinkUserRequest {
    // Empty - we get the sub from JWT token
}

/// Response when linking a user
#[derive(Debug, Serialize)]
pub struct LinkUserResponse {
    pub id: i64,
    pub auth_user_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub message: String,
}

// ==================== GET USER ====================

/// Response for user data (church system only)
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub auth_user_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

// ==================== LIST USERS ====================

/// Query parameters for listing users
#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_page() -> i64 {
    1
}

fn default_limit() -> i64 {
    20
}
