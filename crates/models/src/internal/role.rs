use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleResponse {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub level: i32,
    pub permissions: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<crate::dto::role::Model> for RoleResponse {
    fn from(role: crate::dto::role::Model) -> Self {
        Self {
            id: role.id,
            name: role.name,
            description: role.description,
            level: role.level,
            permissions: role.permissions,
            created_at: role.created_at,
            updated_at: role.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub description: Option<String>,
    pub level: i32,
    pub permissions: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoleRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub level: Option<i32>,
    pub permissions: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListRolesQuery {
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
