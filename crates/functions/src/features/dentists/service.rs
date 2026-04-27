use http_response::{CustomError, HttpCodeW};
use models::dto::dentist::{ActiveModel as DentistActiveModel, Column, Entity as Dentist, Model as DentistModel};
use models::dto::user::Entity as User;
use models::internal::dentist::{
    CreateDentistRequest, DentistResponse, ListDentistsResponse, Pagination as DentistPagination,
    UpdateDentistRequest,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, Set};

pub struct DentistService;

impl DentistService {
    pub async fn create(
        db: &DatabaseConnection,
        request: CreateDentistRequest,
    ) -> Result<DentistResponse, CustomError> {
        let user = User::find_by_id(request.user_id)
            .one(db)
            .await?
            .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "User not found".to_string()))?;

        let now = chrono::Utc::now().naive_utc();
        let new_dentist = DentistActiveModel {
            user_id: Set(request.user_id),
            specialty: Set(request.specialty),
            license_number: Set(request.license_number),
            photo_url: Set(request.photo_url),
            bio: Set(request.bio),
            is_available: Set(request.is_available.unwrap_or(true)),
            consultation_duration: Set(request.consultation_duration.unwrap_or(30)),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let dentist = new_dentist.insert(db).await?;

        Ok(Self::to_response_with_user(dentist, user.auth_user_id.clone()))
    }

    pub async fn get_by_id(db: &DatabaseConnection, id: i64) -> Result<DentistResponse, CustomError> {
        let dentist = Dentist::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "Dentist not found".to_string()))?;

        let user = User::find_by_id(dentist.user_id)
            .one(db)
            .await?
            .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "Linked user not found".to_string()))?;

        Ok(Self::to_response_with_user(dentist, user.auth_user_id))
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: i64,
        request: UpdateDentistRequest,
    ) -> Result<DentistResponse, CustomError> {
        let dentist = Dentist::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "Dentist not found".to_string()))?;

        let user = User::find_by_id(dentist.user_id)
            .one(db)
            .await?
            .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "Linked user not found".to_string()))?;

        let now = chrono::Utc::now().naive_utc();
        let mut active_model: DentistActiveModel = dentist.into();

        if let Some(specialty) = request.specialty {
            active_model.specialty = Set(Some(specialty));
        }
        if let Some(license_number) = request.license_number {
            active_model.license_number = Set(Some(license_number));
        }
        if let Some(photo_url) = request.photo_url {
            active_model.photo_url = Set(Some(photo_url));
        }
        if let Some(bio) = request.bio {
            active_model.bio = Set(Some(bio));
        }
        if let Some(is_available) = request.is_available {
            active_model.is_available = Set(is_available);
        }
        if let Some(consultation_duration) = request.consultation_duration {
            active_model.consultation_duration = Set(consultation_duration);
        }
        active_model.updated_at = Set(now);

        let updated = active_model.update(db).await?;

        Ok(Self::to_response_with_user(updated, user.auth_user_id))
    }

    pub async fn delete(db: &DatabaseConnection, id: i64) -> Result<(), CustomError> {
        let dentist = Dentist::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| CustomError::new(HttpCodeW::NotFound, "Dentist not found".to_string()))?;

        let active_model: DentistActiveModel = dentist.into();
        active_model.delete(db).await?;

        Ok(())
    }

    pub async fn list(
        db: &DatabaseConnection,
        page: i64,
        limit: i64,
        specialty: Option<String>,
    ) -> Result<ListDentistsResponse, CustomError> {
        let page = if page < 1 { 1 } else { page };
        let limit = if limit < 1 || limit > 100 { 20 } else { limit };

        let mut query = Dentist::find();

        if let Some(specialty_filter) = specialty {
            query = query.filter(Column::Specialty.eq(specialty_filter));
        }

        let paginator = query.paginate(db, limit as u64);
        let data = paginator.fetch_page((page - 1) as u64).await?;
        let total = paginator.num_items().await?;

        let mut results = Vec::new();
        for dentist in data {
            let user = User::find_by_id(dentist.user_id)
                .one(db)
                .await?;
            let auth_id = user.map(|u| u.auth_user_id).unwrap_or_else(|| "Unknown".to_string());
            results.push(Self::to_response_with_user(dentist, auth_id));
        }

        let total_pages = (total as f64 / limit as f64).ceil() as i64;

        Ok(ListDentistsResponse {
            data: results,
            pagination: DentistPagination {
                page,
                limit,
                total: total as i64,
                total_pages,
            },
        })
    }

    fn to_response_with_user(dentist: DentistModel, auth_user_id: String) -> DentistResponse {
        DentistResponse {
            id: dentist.id,
            user_id: dentist.user_id,
            name: auth_user_id,
            email: None,
            phone: None,
            specialty: dentist.specialty,
            license_number: dentist.license_number,
            photo_url: dentist.photo_url,
            bio: dentist.bio,
            is_available: dentist.is_available,
            consultation_duration: dentist.consultation_duration,
            created_at: dentist.created_at,
            updated_at: dentist.updated_at,
        }
    }
}