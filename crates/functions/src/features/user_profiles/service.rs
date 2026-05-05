use http_response::{CustomError, HttpCodeW};
use models::dto::user_profile::{
    ActiveModel as UserProfileActiveModel, Column as UserProfileColumn, Entity as UserProfile,
    Model as UserProfileModel,
};
use models::internal::user_profile::{
    CreateUserProfileRequest, UpdateUserProfileRequest, UserProfileResponse,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set};

pub struct UserProfileService;

impl UserProfileService {
    /// Raw entity lookup. Other services use this when they need identity
    /// fields (full_name, phone, email, cnp).
    pub async fn get_by_user_id<C: ConnectionTrait>(
        db: &C,
        user_id: i64,
    ) -> Result<UserProfileModel, CustomError> {
        UserProfile::find()
            .filter(UserProfileColumn::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| not_found())
    }

    pub async fn get_by_user_sub<C: ConnectionTrait>(
        db: &C,
        user_sub: &str,
    ) -> Result<UserProfileResponse, CustomError> {
        let profile = find_by_user_sub(db, user_sub).await?.ok_or_else(not_found)?;
        Ok(to_response(profile))
    }

    pub async fn create<C: ConnectionTrait>(
        db: &C,
        user_sub: &str,
        user_id: i64,
        request: CreateUserProfileRequest,
    ) -> Result<UserProfileResponse, CustomError> {
        ensure_no_existing(db, user_sub).await?;
        let profile = build_create_am(user_sub, user_id, request).insert(db).await?;
        Ok(to_response(profile))
    }

    pub async fn update<C: ConnectionTrait>(
        db: &C,
        user_sub: &str,
        request: UpdateUserProfileRequest,
    ) -> Result<UserProfileResponse, CustomError> {
        let profile = find_by_user_sub(db, user_sub).await?.ok_or_else(not_found)?;
        let mut am: UserProfileActiveModel = profile.into();
        apply_update_patch(&mut am, request);
        let updated = am.update(db).await?;
        Ok(to_response(updated))
    }

    /// Patient/dentist services call this inside their transaction to seed
    /// or refresh identity columns on the profile row.
    pub async fn upsert_identity<C: ConnectionTrait>(
        db: &C,
        user_sub: &str,
        user_id: i64,
        full_name: Option<String>,
        phone: Option<String>,
        email: Option<String>,
        cnp: Option<String>,
    ) -> Result<UserProfileResponse, CustomError> {
        if let Some(existing) = find_by_user_id(db, user_id).await? {
            let updated = update_identity(db, existing, full_name, phone, email, cnp).await?;
            return Ok(to_response(updated));
        }
        let inserted = insert_identity(db, user_sub, user_id, full_name, phone, email, cnp).await?;
        Ok(to_response(inserted))
    }
}

fn not_found() -> CustomError {
    CustomError::new(HttpCodeW::NotFound, "User profile not found".to_string())
}

fn already_exists() -> CustomError {
    CustomError::new(HttpCodeW::Conflict, "User profile already exists".to_string())
}

fn to_response(profile: UserProfileModel) -> UserProfileResponse {
    UserProfileResponse {
        id: profile.id, user_sub: profile.user_sub, user_id: profile.user_id,
        schema_version: profile.schema_version, attributes: profile.attributes,
        full_name: profile.full_name, phone: profile.phone,
        email: profile.email, cnp: profile.cnp,
        created_at: profile.created_at, updated_at: profile.updated_at,
    }
}

fn merge_identity(
    am: &mut UserProfileActiveModel,
    full_name: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    cnp: Option<String>,
) {
    if let Some(n) = full_name { am.full_name = Set(Some(n)); }
    if let Some(p) = phone { am.phone = Set(Some(p)); }
    if let Some(e) = email { am.email = Set(Some(e)); }
    if let Some(c) = cnp { am.cnp = Set(Some(c)); }
}

fn apply_update_patch(am: &mut UserProfileActiveModel, request: UpdateUserProfileRequest) {
    if let Some(attributes) = request.attributes { am.attributes = Set(attributes); }
    merge_identity(am, request.full_name, request.phone, request.email, request.cnp);
    am.updated_at = Set(chrono::Utc::now().naive_utc());
}

fn build_create_am(
    user_sub: &str,
    user_id: i64,
    request: CreateUserProfileRequest,
) -> UserProfileActiveModel {
    let now = chrono::Utc::now().naive_utc();
    let attributes = request.attributes.unwrap_or(serde_json::json!({}));
    UserProfileActiveModel {
        user_sub: Set(user_sub.to_string()), user_id: Set(user_id),
        schema_version: Set("1.0".to_string()), attributes: Set(attributes),
        full_name: Set(request.full_name), phone: Set(request.phone),
        email: Set(request.email), cnp: Set(request.cnp),
        created_at: Set(now), updated_at: Set(now),
        ..Default::default()
    }
}

async fn find_by_user_sub<C: ConnectionTrait>(
    db: &C,
    user_sub: &str,
) -> Result<Option<UserProfileModel>, CustomError> {
    Ok(UserProfile::find()
        .filter(UserProfileColumn::UserSub.eq(user_sub))
        .one(db)
        .await?)
}

async fn find_by_user_id<C: ConnectionTrait>(
    db: &C,
    user_id: i64,
) -> Result<Option<UserProfileModel>, CustomError> {
    Ok(UserProfile::find()
        .filter(UserProfileColumn::UserId.eq(user_id))
        .one(db)
        .await?)
}

async fn ensure_no_existing<C: ConnectionTrait>(
    db: &C,
    user_sub: &str,
) -> Result<(), CustomError> {
    if find_by_user_sub(db, user_sub).await?.is_some() {
        return Err(already_exists());
    }
    Ok(())
}

async fn update_identity<C: ConnectionTrait>(
    db: &C,
    existing: UserProfileModel,
    full_name: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    cnp: Option<String>,
) -> Result<UserProfileModel, CustomError> {
    let mut am: UserProfileActiveModel = existing.into();
    merge_identity(&mut am, full_name, phone, email, cnp);
    am.updated_at = Set(chrono::Utc::now().naive_utc());
    Ok(am.update(db).await?)
}

async fn insert_identity<C: ConnectionTrait>(
    db: &C,
    user_sub: &str,
    user_id: i64,
    full_name: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    cnp: Option<String>,
) -> Result<UserProfileModel, CustomError> {
    let mut am = build_identity_seed_am(user_sub, user_id);
    merge_identity(&mut am, full_name, phone, email, cnp);
    Ok(am.insert(db).await?)
}

fn build_identity_seed_am(user_sub: &str, user_id: i64) -> UserProfileActiveModel {
    let now = chrono::Utc::now().naive_utc();
    UserProfileActiveModel {
        user_sub: Set(user_sub.to_string()), user_id: Set(user_id),
        schema_version: Set("1.0".to_string()),
        attributes: Set(serde_json::json!({})),
        created_at: Set(now), updated_at: Set(now),
        ..Default::default()
    }
}
