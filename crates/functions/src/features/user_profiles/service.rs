use http_response::{CustomError, HttpCodeW};
use models::dto::user_profile::{ActiveModel as UserProfileActiveModel, Column, Entity as UserProfile, Model as UserProfileModel};
use models::internal::user_profile::{CreateUserProfileRequest, UpdateUserProfileRequest, UserProfileResponse};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub struct UserProfileService;

impl UserProfileService {
    pub async fn get_by_user_sub(
        db: &DatabaseConnection,
        user_sub: &str,
    ) -> Result<UserProfileResponse, CustomError> {
        let profile = UserProfile::find()
            .filter(Column::UserSub.eq(user_sub))
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(
                    HttpCodeW::NotFound,
                    "User profile not found".to_string(),
                )
            })?;

        Ok(Self::to_response(profile))
    }

    pub async fn create(
        db: &DatabaseConnection,
        user_sub: &str,
        request: CreateUserProfileRequest,
    ) -> Result<UserProfileResponse, CustomError> {
        let existing = UserProfile::find()
            .filter(Column::UserSub.eq(user_sub))
            .one(db)
            .await?;

        if existing.is_some() {
            return Err(CustomError::new(
                HttpCodeW::Conflict,
                "User profile already exists".to_string(),
            ));
        }

        let now = chrono::Utc::now().naive_utc();
        let attributes = request.attributes.unwrap_or(serde_json::json!({}));

        let new_profile = UserProfileActiveModel {
            user_sub: Set(user_sub.to_string()),
            schema_version: Set("1.0".to_string()),
            attributes: Set(attributes),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let profile = new_profile.insert(db).await?;

        Ok(Self::to_response(profile))
    }

    pub async fn update(
        db: &DatabaseConnection,
        user_sub: &str,
        request: UpdateUserProfileRequest,
    ) -> Result<UserProfileResponse, CustomError> {
        let profile = UserProfile::find()
            .filter(Column::UserSub.eq(user_sub))
            .one(db)
            .await?
            .ok_or_else(|| {
                CustomError::new(
                    HttpCodeW::NotFound,
                    "User profile not found".to_string(),
                )
            })?;

        let now = chrono::Utc::now().naive_utc();
        let mut active_model: UserProfileActiveModel = profile.into();

        if let Some(attributes) = request.attributes {
            active_model.attributes = Set(attributes);
        }
        active_model.updated_at = Set(now);

        let updated = active_model.update(db).await?;

        Ok(Self::to_response(updated))
    }

    pub async fn upsert(
        db: &DatabaseConnection,
        user_sub: &str,
        request: CreateUserProfileRequest,
    ) -> Result<UserProfileResponse, CustomError> {
        let existing = UserProfile::find()
            .filter(Column::UserSub.eq(user_sub))
            .one(db)
            .await?;

        match existing {
            Some(_profile) => {
                Self::update(
                    db,
                    user_sub,
                    UpdateUserProfileRequest {
                        attributes: request.attributes,
                    },
                )
                .await
            }
            None => Self::create(db, user_sub, request).await,
        }
    }

    fn to_response(profile: UserProfileModel) -> UserProfileResponse {
        UserProfileResponse {
            id: profile.id,
            user_sub: profile.user_sub,
            schema_version: profile.schema_version,
            attributes: profile.attributes,
            created_at: profile.created_at,
            updated_at: profile.updated_at,
        }
    }
}