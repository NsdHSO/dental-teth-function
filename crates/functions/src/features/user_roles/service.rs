use crate::features::roles::service::RoleService;
use crate::features::users::service::UserService;
use http_response::{CustomError, HttpCodeW};
use models::dto::{user_role, UserRole};
use models::internal::UserRoleResponse;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use user_role::{ActiveModel, Column, Model};
use Column::{IsActive, RoleId, UserId};

pub struct UserRoleService;

impl UserRoleService {
    /// Helper to build UserRoleResponse from user_role model and role name
    fn build_response(user_role: Model, role_name: String) -> UserRoleResponse {
        UserRoleResponse {
            id: user_role.id,
            user_id: user_role.user_id,
            role_id: user_role.role_id,
            role_name,
            assigned_date: user_role.assigned_date,
            assigned_by: user_role.assigned_by,
            is_active: user_role.is_active,
            created_at: user_role.created_at,
            updated_at: user_role.updated_at,
        }
    }

    /// Assign a role to a user
    pub async fn assign_role(
        db: &DatabaseConnection,
        user_id: i64,
        role_id: i64,
        assigned_by_user_id: i64,
    ) -> Result<UserRoleResponse, CustomError> {
        // Check if user exists via UserService
        UserService::get_user_by_id(db, user_id).await?;

        // Check if role exists via RoleService
        let role = RoleService::get_role_by_id(db, role_id).await?;

        // Check if user already has this role
        let existing = UserRole::find()
            .filter(UserId.eq(user_id))
            .filter(RoleId.eq(role_id))
            .filter(IsActive.eq(true))
            .one(db)
            .await?;

        if existing.is_some() {
            return Err(CustomError::new(
                HttpCodeW::Conflict,
                format!("User already has the '{}' role", role.name),
            ));
        }

        // Create user_role assignment
        let now = chrono::Utc::now().naive_utc();
        let new_user_role = ActiveModel {
            user_id: Set(user_id),
            role_id: Set(role_id),
            assigned_date: Set(chrono::Utc::now().date_naive()),
            assigned_by: Set(Some(assigned_by_user_id)),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let user_role = new_user_role.insert(db).await?;

        Ok(Self::build_response(user_role, role.name))
    }

    /// Remove a role from a user (set is_active = false)
    pub async fn remove_role(
        db: &DatabaseConnection,
        user_id: i64,
        role_id: i64,
    ) -> Result<(), CustomError> {
        // Find the user_role assignment
        let user_role = UserRole::find()
            .filter(UserId.eq(user_id))
            .filter(RoleId.eq(role_id))
            .filter(IsActive.eq(true))
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(
                    HttpCodeW::NotFound,
                    "User does not have this role".to_string(),
                )
            })?;

        // Deactivate the role assignment
        let mut active_user_role: ActiveModel = user_role.into();
        active_user_role.is_active = Set(false);
        active_user_role.updated_at = Set(chrono::Utc::now().naive_utc());
        active_user_role.update(db).await?;

        Ok(())
    }

    /// Get all roles for a specific user
    pub async fn get_user_roles(
        db: &DatabaseConnection,
        user_id: i64,
    ) -> Result<Vec<UserRoleResponse>, CustomError> {
        // Check if user exists via UserService
        UserService::get_user_by_id(db, user_id).await?;

        // Get all active user_role records for this user
        let user_roles = UserRole::find()
            .filter(UserId.eq(user_id))
            .filter(IsActive.eq(true))
            .all(db)
            .await?;

        // For each user_role, get role details via RoleService
        let mut responses = Vec::new();
        for user_role in user_roles {
            if let Ok(role) = RoleService::get_role_by_id(db, user_role.role_id).await {
                responses.push(Self::build_response(user_role, role.name));
            }
        }

        Ok(responses)
    }

    /// Get all users with a specific role
    pub async fn get_users_by_role(
        db: &DatabaseConnection,
        role_id: i64,
    ) -> Result<Vec<UserRoleResponse>, CustomError> {
        // Check if role exists via RoleService
        let role = RoleService::get_role_by_id(db, role_id).await?;

        // Get all active user_role assignments for this role (only accessing own table)
        let user_roles = UserRole::find()
            .filter(RoleId.eq(role_id))
            .filter(IsActive.eq(true))
            .all(db)
            .await?;

        let responses: Vec<UserRoleResponse> = user_roles
            .into_iter()
            .map(|user_role| Self::build_response(user_role, role.name.clone()))
            .collect();

        Ok(responses)
    }
}
