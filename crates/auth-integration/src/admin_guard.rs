use actix_web::{dev::Payload, web, FromRequest, HttpMessage, HttpRequest, HttpResponse};
use futures_util::future::{ready, LocalBoxFuture};
use http_response::{create_response, HttpCodeW};

use crate::subject::Subject;

/// AdminGuard extractor that ensures the request is made by an Admin user.
///
/// This extractor:
/// 1. Extracts the Subject from the request (populated by JwtAuth middleware)
/// 2. Gets the church user_id from the database
/// 3. Queries the user_roles table to check if user has "Admin" role
/// 4. Returns 403 Forbidden if not an admin
/// 5. Provides convenient access to church_user_id and auth_user_id
///
/// **IMPORTANT:** This checks the role from the CHURCH DATABASE (user_roles table),
/// NOT from the auth server JWT. The church system manages its own authorization.
///
/// # Example
///
/// ```rust
/// pub async fn admin_only_handler(
///     db: web::Data<DatabaseConnection>,
///     admin: AdminGuard,
/// ) -> Result<HttpResponse> {
///     // If we reach here, user is guaranteed to be admin
///     // admin.church_user_id - ID in church.users table
///     // admin.auth_user_id - ID from auth server
///     Ok(HttpResponse::Ok().json("Admin access granted"))
/// }
/// ```
#[derive(Clone, Debug)]
pub struct AdminGuard {
    /// The church user ID (from church.users table)
    pub church_user_id: i64,
    /// The auth server user ID (from JWT sub claim)
    pub auth_user_id: String,
}

impl FromRequest for AdminGuard {
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _pl: &mut Payload) -> Self::Future {
        // Extract the Subject that was populated by JwtAuth middleware
        let subject = match req.extensions().get::<Subject>() {
            Some(subj) => subj.clone(),
            None => {
                return Box::pin(ready(Err(actix_web::error::ErrorUnauthorized(
                    "Authentication required",
                ))));
            }
        };

        // Get database connection from app_data
        let db = match req.app_data::<web::Data<sea_orm::DatabaseConnection>>() {
            Some(db) => db.clone(),
            None => {
                return Box::pin(ready(Err(actix_web::error::ErrorInternalServerError(
                    "Database connection not available",
                ))));
            }
        };

        Box::pin(async move {
            // Import here to avoid circular dependencies
            use models::dto::role::Column as RoleColumn;
            use models::dto::user::Column as UserColumn;
            use models::dto::user_role::Column as UserRoleColumn;
            use models::dto::{Role, User, UserRole};
            use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

            // 1. Get church user by auth_user_id
            let church_user = User::find()
                .filter(UserColumn::AuthUserId.eq(&subject.sub))
                .one(db.as_ref())
                .await
                .map_err(|_| actix_web::error::ErrorInternalServerError("Database query failed"))?
                .ok_or_else(|| {
                    actix_web::error::ErrorUnauthorized("User not linked to church system")
                })?;

            // 2. Find "Admin" role
            let admin_role = Role::find()
                .filter(RoleColumn::Name.eq("Admin"))
                .one(db.as_ref())
                .await
                .map_err(|_| actix_web::error::ErrorInternalServerError("Database query failed"))?
                .ok_or_else(|| {
                    actix_web::error::ErrorInternalServerError("Admin role not configured")
                })?;

            // 3. Check if user has Admin role (active assignment)
            let has_admin_role = UserRole::find()
                .filter(UserRoleColumn::UserId.eq(church_user.id))
                .filter(UserRoleColumn::RoleId.eq(admin_role.id))
                .filter(UserRoleColumn::IsActive.eq(true))
                .one(db.as_ref())
                .await
                .map_err(|_| actix_web::error::ErrorInternalServerError("Database query failed"))?
                .is_some();

            if !has_admin_role {
                // User does not have Admin role - return 403 Forbidden
                let error_response = create_response("Admin role required", HttpCodeW::Forbidden);
                return Err(actix_web::error::InternalError::from_response(
                    "Admin role required",
                    HttpResponse::Forbidden().json(error_response),
                )
                .into());
            }

            // User has Admin role - grant access
            Ok(AdminGuard {
                church_user_id: church_user.id,
                auth_user_id: subject.sub,
            })
        })
    }
}
