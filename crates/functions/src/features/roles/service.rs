use http_response::{CustomError, HttpCodeW};
use models::dto::{role, Role};
use models::internal::{CreateRoleRequest, RoleResponse, UpdateRoleRequest};
use role::ActiveModel;
use role::Column::Name;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};
use serde_json::json;

pub struct RoleService;

impl RoleService {
    /// Create a new role
    pub async fn create_role(
        db: &DatabaseConnection,
        request: CreateRoleRequest,
    ) -> Result<RoleResponse, CustomError> {
        // Check if role name already exists
        let existing = Role::find().filter(Name.eq(&request.name)).one(db).await?;

        if existing.is_some() {
            return Err(CustomError::new(
                HttpCodeW::Conflict,
                format!("Role with name '{}' already exists", request.name),
            ));
        }

        let now = chrono::Utc::now().naive_utc();
        let new_role = ActiveModel {
            name: Set(request.name),
            description: Set(request.description),
            level: Set(request.level),
            permissions: Set(request.permissions),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let role = new_role.insert(db).await?;

        Ok(role.into())
    }

    /// Get a role by ID
    pub async fn get_role_by_id(
        db: &DatabaseConnection,
        role_id: i64,
    ) -> Result<RoleResponse, CustomError> {
        let role = Role::find_by_id(role_id)
            .one(db)
            .await?
            .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "Role not found".to_string()))?;

        Ok(role.into())
    }

    /// Get a role by name
    pub async fn get_role_by_name(
        db: &DatabaseConnection,
        name: &str,
    ) -> Result<RoleResponse, CustomError> {
        let role = Role::find()
            .filter(Name.eq(name))
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(HttpCodeW::NotFound, format!("Role '{}' not found", name))
            })?;

        Ok(role.into())
    }

    /// List all roles with pagination
    pub async fn list_roles(
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

        let roles = Role::find()
            .paginate(db, limit as u64)
            .fetch_page((page - 1) as u64)
            .await?;

        let total = Role::find().count(db).await?;

        Ok(json!({
            "data": roles,
            "pagination": {
                "page": page,
                "limit": limit,
                "total": total,
                "total_pages": (total as f64 / limit as f64).ceil() as i64
            }
        }))
    }

    /// Update a role
    pub async fn update_role(
        db: &DatabaseConnection,
        role_id: i64,
        request: UpdateRoleRequest,
    ) -> Result<RoleResponse, CustomError> {
        let existing_role = Role::find_by_id(role_id)
            .one(db)
            .await?
            .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "Role not found".to_string()))?;

        let mut active_role: ActiveModel = existing_role.into();

        if let Some(name) = request.name {
            // Check if new name conflicts with another role
            let conflict = Role::find()
                .filter(Name.eq(&name))
                .filter(role::Column::Id.ne(role_id))
                .one(db)
                .await?;

            if conflict.is_some() {
                return Err(CustomError::new(
                    HttpCodeW::Conflict,
                    format!("Role with name '{}' already exists", name),
                ));
            }

            active_role.name = Set(name);
        }

        if let Some(description) = request.description {
            active_role.description = Set(Some(description));
        }

        if let Some(level) = request.level {
            active_role.level = Set(level);
        }

        if let Some(permissions) = request.permissions {
            active_role.permissions = Set(Some(permissions));
        }

        active_role.updated_at = Set(chrono::Utc::now().naive_utc());

        let updated = active_role.update(db).await?;

        Ok(updated.into())
    }

    /// Delete a role
    pub async fn delete_role(db: &DatabaseConnection, role_id: i64) -> Result<(), CustomError> {
        let role = Role::find_by_id(role_id)
            .one(db)
            .await?
            .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "Role not found".to_string()))?;

        let active_role: ActiveModel = role.into();
        active_role.delete(db).await?;

        Ok(())
    }
}
