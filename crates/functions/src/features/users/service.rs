use http_response::{CustomError, HttpCodeW};
use models::dto::user::Model as UserModel;
use models::internal::{LinkUserResponse, UserResponse};
use models::{Role, User, UserActiveModel};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, Set,
};
use serde_json::json;
use uuid::Uuid;

pub struct UserService;

impl UserService {
    /// Link a user from auth server to church system.
    pub async fn link_user<C: ConnectionTrait>(
        db: &C,
        auth_user_id: &str,
    ) -> Result<LinkUserResponse, CustomError> {
        if let Some(user) = find_by_auth_id(db, auth_user_id).await? {
            return Ok(linked_response(user, "User already linked"));
        }
        let user = insert_auth_user(db, auth_user_id).await?;
        let _ = assign_role_by_name(db, user.id, "Member").await;
        Ok(linked_response(
            user,
            "User linked successfully",
        ))
    }

    /// Raw entity lookup by primary key. Other services MUST use this
    /// instead of touching `User::find_by_id` directly.
    pub async fn get_by_id<C: ConnectionTrait>(
        db: &C,
        user_id: i64,
    ) -> Result<UserModel, CustomError> {
        User::find_by_id(user_id)
            .one(db)
            .await?
            .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "User not found".to_string()))
    }

    pub async fn get_user_by_id<C: ConnectionTrait>(
        db: &C,
        user_id: i64,
    ) -> Result<UserResponse, CustomError> {
        let user = Self::get_by_id(db, user_id).await?;
        Ok(to_user_response(user))
    }

    pub async fn get_user_by_auth_id<C: ConnectionTrait>(
        db: &C,
        auth_user_id: &str,
    ) -> Result<UserResponse, CustomError> {
        let user = find_by_auth_id(db, auth_user_id).await?.ok_or_else(|| {
            CustomError::new(
                HttpCodeW::NotFound,
                "User not linked in church system".to_string(),
            )
        })?;
        Ok(to_user_response(user))
    }

    /// Create a synthetic user for a walk-in patient. Returns the inserted
    /// row (caller is responsible for creating user_profile + patient rows
    /// inside the same transaction).
    pub async fn find_or_create_for_patient_seed<C: ConnectionTrait>(
        db: &C,
    ) -> Result<UserModel, CustomError> {
        let sub = synthetic_patient_sub();
        let user = insert_auth_user(db, &sub).await?;
        assign_role_by_name(db, user.id, "Patient").await?;
        Ok(user)
    }

    pub async fn list_users<C: ConnectionTrait>(
        db: &C,
        page: i64,
        limit: i64,
    ) -> Result<serde_json::Value, CustomError> {
        let (page, limit) = clamp_page(page, limit);
        let users = User::find()
            .paginate(db, limit as u64)
            .fetch_page((page - 1) as u64)
            .await?;
        let total = User::find().count(db).await?;
        Ok(list_response(users, page, limit, total))
    }
}

fn synthetic_patient_sub() -> String {
    format!("local:patient:{}", Uuid::new_v4())
}

fn to_user_response(user: UserModel) -> UserResponse {
    UserResponse {
        id: user.id,
        auth_user_id: user.auth_user_id,
        created_at: user.created_at,
        updated_at: user.updated_at,
    }
}

fn linked_response(user: UserModel, message: &str) -> LinkUserResponse {
    LinkUserResponse {
        id: user.id,
        auth_user_id: user.auth_user_id,
        created_at: user.created_at,
        updated_at: user.updated_at,
        message: message.to_string(),
    }
}

fn clamp_page(page: i64, limit: i64) -> (i64, i64) {
    let p = if page < 1 { 1 } else { page };
    let l = if (1..=100).contains(&limit) { limit } else { 20 };
    (p, l)
}

fn list_response(users: Vec<UserModel>, page: i64, limit: i64, total: u64) -> serde_json::Value {
    json!({
        "data": users,
        "pagination": {
            "page": page,
            "limit": limit,
            "total": total,
            "total_pages": (total as f64 / limit as f64).ceil() as i64,
        }
    })
}

async fn find_by_auth_id<C: ConnectionTrait>(
    db: &C,
    auth_user_id: &str,
) -> Result<Option<UserModel>, CustomError> {
    use models::dto::user::Column;
    Ok(User::find()
        .filter(Column::AuthUserId.eq(auth_user_id))
        .one(db)
        .await?)
}

async fn insert_auth_user<C: ConnectionTrait>(
    db: &C,
    auth_user_id: &str,
) -> Result<UserModel, CustomError> {
    let now = chrono::Utc::now().naive_utc();
    let am = UserActiveModel {
        auth_user_id: Set(auth_user_id.to_string()),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };
    Ok(am.insert(db).await?)
}

async fn assign_role_by_name<C: ConnectionTrait>(
    db: &C,
    user_id: i64,
    role_name: &str,
) -> Result<(), CustomError> {
    let role_id = find_role_id_by_name(db, role_name).await?;
    insert_user_role(db, user_id, role_id).await
}

async fn find_role_id_by_name<C: ConnectionTrait>(
    db: &C,
    role_name: &str,
) -> Result<i64, CustomError> {
    use models::dto::role::Column;
    let role = Role::find()
        .filter(Column::Name.eq(role_name))
        .one(db)
        .await?
        .ok_or_else(|| {
            CustomError::new(HttpCodeW::NotFound, format!("Role '{}' not found", role_name))
        })?;
    Ok(role.id)
}

async fn insert_user_role<C: ConnectionTrait>(
    db: &C,
    user_id: i64,
    role_id: i64,
) -> Result<(), CustomError> {
    let am = build_user_role_am(user_id, role_id);
    am.insert(db).await.map(|_| ()).map_err(Into::into)
}

fn build_user_role_am(user_id: i64, role_id: i64) -> models::dto::user_role::ActiveModel {
    let now = chrono::Utc::now().naive_utc();
    let today = chrono::Utc::now().date_naive();
    models::dto::user_role::ActiveModel {
        user_id: Set(user_id), role_id: Set(role_id),
        assigned_date: Set(today), assigned_by: Set(Some(user_id)),
        is_active: Set(true),
        created_at: Set(now), updated_at: Set(now),
        ..Default::default()
    }
}
