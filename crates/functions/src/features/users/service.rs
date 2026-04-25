use http_response::{CustomError, HttpCodeW};
use models::internal::{LinkUserResponse, UserResponse};
use models::{User, UserActiveModel};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};
use serde_json::json;

use crate::features::roles::service::RoleService;
use crate::features::user_roles::service::UserRoleService;

pub struct UserService;

impl UserService {
    /// Link a user from auth server to church system
    pub async fn link_user(
        db: &DatabaseConnection,
        auth_user_id: &str,
    ) -> Result<LinkUserResponse, CustomError> {
        // Check if user already exists
        use models::dto::user::Column;
        let existing = User::find()
            .filter(Column::AuthUserId.eq(auth_user_id))
            .one(db)
            .await?;

        match existing {
            Some(user) => {
                // User already linked
                Ok(LinkUserResponse {
                    id: user.id,
                    auth_user_id: user.auth_user_id,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                    message: "User already linked".to_string(),
                })
            }
            None => {
                // Create new church user linked to auth user
                let now = chrono::Utc::now().naive_utc();
                let new_user = UserActiveModel {
                    auth_user_id: Set(auth_user_id.to_string()),
                    created_at: Set(now),
                    updated_at: Set(now),
                    ..Default::default()
                };

                let user = new_user.insert(db).await?;

                // Automatically assign "Member" role to new users via RoleService and UserRoleService
                let member_role_result = RoleService::get_role_by_name(db, "Member").await;

                if let Ok(member_role) = member_role_result {
                    // Assign role through UserRoleService (no assigned_by since it's system-assigned)
                    let _ =
                        UserRoleService::assign_role(db, user.id, member_role.id, user.id).await;
                    // Ignore error if role assignment fails - user is still created
                }

                Ok(LinkUserResponse {
                    id: user.id,
                    auth_user_id: user.auth_user_id,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                    message: "User linked successfully with Member role".to_string(),
                })
            }
        }
    }

    /// Get a user by ID
    pub async fn get_user_by_id(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<UserResponse, CustomError> {
        let user = User::find_by_id(user_id)
            .one(db)
            .await?
            .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "User not found".to_string()))?;

        Ok(UserResponse {
            id: user.id,
            auth_user_id: user.auth_user_id,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    }

    /// Get a user by auth_user_id (from JWT sub)
    pub async fn get_user_by_auth_id(
        db: &DatabaseConnection,
        auth_user_id: &str,
    ) -> Result<UserResponse, CustomError> {
        use models::dto::user::Column;
        let user = User::find()
            .filter(Column::AuthUserId.eq(auth_user_id))
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(
                    HttpCodeW::NotFound,
                    "User not linked in church system".to_string(),
                )
            })?;

        Ok(UserResponse {
            id: user.id,
            auth_user_id: user.auth_user_id,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    }

    /// List users with pagination
    pub async fn list_users(
        db: &DatabaseConnection,
        page: i64,
        limit: i64,
    ) -> Result<serde_json::Value, CustomError> {
        let page = if page < 1 { 1 } else { page };
        let limit = if (1..=100).contains(&limit) {
            20
        } else {
            limit
        };

        let users = User::find()
            .paginate(db, limit as u64)
            .fetch_page((page - 1) as u64)
            .await?;

        let total = User::find().count(db).await?;

        Ok(json!({
            "data": users,
            "pagination": {
                "page": page,
                "limit": limit,
                "total": total,
                "total_pages": (total as f64 / limit as f64).ceil() as i64
            }
        }))
    }
}
