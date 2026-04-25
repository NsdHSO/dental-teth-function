# Auth Integration Crate

Authentication integration with external auth server for the Church Management System.

## Overview

This crate provides everything needed to integrate with the external auth server (https://github.com/NsdHSO/auth). It
handles:

- JWT token verification
- User authentication middleware
- User data synchronization
- Request-scoped user context

## Architecture

```
┌─────────────────┐
│  Auth Server    │ ← Single source of truth for authentication
│  (External)     │
└────────┬────────┘
         │ REST API
         ↓
┌─────────────────┐
│  AuthClient     │ ← HTTP client for auth server
└────────┬────────┘
         │
         ↓
┌─────────────────┐
│  AuthMiddleware │ ← Validates JWT on every request
└────────┬────────┘
         │
         ↓
┌─────────────────┐
│  UserContext    │ ← Attached to request (user_id, role, email)
└────────┬────────┘
         │
         ↓
┌─────────────────┐
│  UserService    │ ← Combines auth data + church data
└─────────────────┘
```

## Components

### 1. AuthClient

HTTP client for communicating with the auth server.

**Methods:**

- `get_user(user_id)` - Fetch user by ID
- `get_user_by_email(email)` - Fetch user by email
- `verify_token(token)` - Verify JWT token
- `user_exists(user_id)` - Check if user exists

**Example:**

```rust
use auth_integration::AuthClient;

let client = AuthClient::new(
    "https://auth.example.com".to_string(),
    Some("your-api-key".to_string())
);

let user = client.get_user("user-uuid").await?;
println!("User: {} {}", user.first_name, user.last_name);
```

### 2. AuthMiddleware

Actix-web middleware that validates JWT tokens on incoming requests.

**Features:**

- Extracts `Authorization: Bearer <token>` header
- Verifies token with auth server
- Attaches `UserContext` to request extensions
- Supports public paths (no auth required)

**Example:**

```rust
use auth_integration::AuthMiddleware;
use actix_web::{App, HttpServer};

HttpServer::new(move || {
    App::new()
        .wrap(
            AuthMiddleware::new(auth_client.clone())
                .with_public_paths(vec![
                    "/health".to_string(),
                    "/api/public".to_string(),
                ])
        )
        .service(your_routes)
})
.bind(("127.0.0.1", 8080))?
.run()
.await
```

### 3. UserContext

User information attached to authenticated requests.

**Fields:**

- `auth_user_id: String` - User ID from auth server
- `email: Option<String>` - User email
- `role: String` - User role (admin, user, etc.)
- `token: String` - JWT token

**Example:**

```rust
use auth_integration::UserContext;
use actix_web::{get, HttpResponse, Responder};

#[get("/profile")]
async fn get_profile(user: UserContext) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "user_id": user.auth_user_id,
        "email": user.email,
        "role": user.role,
        "is_admin": user.is_admin(),
    }))
}
```

### 4. UserService

Service layer combining auth server data with local church data.

**Methods:**

- `get_complete_user(church_user_id)` - Get full user (auth + church data)
- `find_by_auth_id(auth_user_id)` - Find church user by auth ID
- `find_by_email(email)` - Find user by email
- `link_auth_user(auth_user_id)` - Link auth user to church system
- `link_auth_user_by_email(email)` - Link by email

**Example:**

```rust
use auth_integration::UserService;

let user_service = UserService::new(db.clone(), auth_client.clone());

// Link an auth user to church system
let church_user = user_service
.link_auth_user_by_email("john@example.com")
.await?;

// Get complete user data
let complete_user = user_service
.get_complete_user(church_user.id)
.await?;

println!("Name: {}", complete_user.full_name());
println!("Email: {}", complete_user.email());
println!("Role: {}", complete_user.role());
```

## Setup

### 1. Add to Workspace

Update root `Cargo.toml`:

```toml
[workspace]
members = [
    # ... other crates
    "crates/auth-integration",
]
```

### 2. Add Dependency

In your application's `Cargo.toml`:

```toml
[dependencies]
auth-integration = { path = "../crates/auth-integration" }
```

### 3. Configure Environment

Add to `.env`:

```bash
AUTH_SERVER_URL=https://auth.example.com
AUTH_SERVER_API_KEY=your-api-key
```

### 4. Initialize in main.rs

```rust
use auth_integration::{AuthClient, AuthMiddleware, UserService};
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load config
    let auth_url = std::env::var("AUTH_SERVER_URL")
        .expect("AUTH_SERVER_URL must be set");
    let auth_key = std::env::var("AUTH_SERVER_API_KEY").ok();

    // Create auth client
    let auth_client = AuthClient::new(auth_url, auth_key);

    // Create user service
    let db = /* your database connection */;
    let user_service = UserService::new(db.clone(), auth_client.clone());

    // Start server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_service.clone()))
            .wrap(AuthMiddleware::new(auth_client.clone()))
            .service(
                web::scope("/api")
                    .service(protected_routes)
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
```

## Request Flow

### Protected Endpoint

```
1. Request comes in with: Authorization: Bearer <jwt>
   ↓
2. AuthMiddleware intercepts request
   ↓
3. Extracts token from header
   ↓
4. Calls auth_client.verify_token(token)
   ↓
5. Auth server validates token → returns user_id, role, email
   ↓
6. Middleware creates UserContext { auth_user_id, role, email, token }
   ↓
7. Attaches UserContext to request extensions
   ↓
8. Request proceeds to handler
   ↓
9. Handler extracts UserContext from request
   ↓
10. Handler uses user_id to query church-specific data
```

### Public Endpoint

```
1. Request comes in to /health
   ↓
2. AuthMiddleware checks if path is public
   ↓
3. Path is public → skip authentication
   ↓
4. Request proceeds directly to handler
```

## Error Handling

### Token Verification Failures

Returns `401 Unauthorized` with JSON body:

```json
{
  "error": "Token verification failed: <reason>"
}
```

### Missing Authorization Header

Returns `401 Unauthorized` with JSON body:

```json
{
  "error": "Missing or invalid Authorization header"
}
```

### User Not Found

The service returns `anyhow::Error` which you should handle in your handlers:

```rust
use actix_web::{get, HttpResponse, Responder};

#[get("/users/{id}")]
async fn get_user(
    user_service: web::Data<UserService>,
    path: web::Path<i64>,
) -> impl Responder {
    match user_service.get_complete_user(*path).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::NotFound().json(serde_json::json!({
            "error": format!("User not found: {}", e)
        })),
    }
}
```

## Testing

### Unit Tests

```bash
cargo test -p auth-integration
```

### Integration Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_auth_middleware() {
        // Create mock auth client
        let auth_client = AuthClient::new(
            "http://localhost:8080".to_string(),
            None
        );

        // Test middleware
        // ...
    }
}
```

## Migration Guide

If you have existing user data, follow these steps:

### 1. Add auth_user_id column

```sql
ALTER TABLE users ADD COLUMN auth_user_id VARCHAR(255);
CREATE UNIQUE INDEX idx_users_auth_user_id ON users(auth_user_id);
```

### 2. Link existing users

```rust
use auth_integration::UserService;

async fn migrate_users(user_service: &UserService) {
    let users = /* fetch all users from church DB */;

    for user in users {
        // Create user in auth server (via auth server API)
        // Then link to church system
        user_service
            .link_auth_user(&auth_user_id)
            .await
            .expect("Failed to link user");
    }
}
```

### 3. Remove old columns

```sql
ALTER TABLE users
    DROP COLUMN email,
    DROP COLUMN password_hash,
    DROP COLUMN first_name,
    DROP COLUMN last_name,
    DROP COLUMN is_active,
    DROP COLUMN is_email_verified,
    DROP COLUMN last_login;
```

## Performance Considerations

### Caching (Recommended)

The current implementation makes a fresh API call to the auth server on every request. For better performance, consider
adding caching:

```rust
use redis::AsyncCommands;

async fn get_user_cached(
    auth_client: &AuthClient,
    cache: &redis::Client,
    user_id: &str,
) -> Result<AuthUser> {
    let cache_key = format!("auth_user:{}", user_id);

    // Try cache first
    let mut conn = cache.get_async_connection().await?;
    if let Some(cached) = conn.get::<_, Option<String>>(&cache_key).await? {
        return Ok(serde_json::from_str(&cached)?);
    }

    // Cache miss - fetch from auth server
    let user = auth_client.get_user(user_id).await?;

    // Cache for 10 minutes
    let serialized = serde_json::to_string(&user)?;
    conn.set_ex(&cache_key, serialized, 600).await?;

    Ok(user)
}
```

## Security Notes

1. **Always use HTTPS** for auth server communication in production
2. **Never log JWT tokens** - they contain sensitive information
3. **Validate token expiration** - tokens should have short lifetimes
4. **Use API keys** for server-to-server auth (AUTH_SERVER_API_KEY)
5. **Rotate API keys** regularly

## Troubleshooting

### "Token verification failed"

- Check that AUTH_SERVER_URL is correct
- Verify token is not expired
- Ensure auth server is running

### "User not found in auth server"

- User may not exist in auth server
- Check user_id/email is correct
- Verify auth server API key has correct permissions

### "Database connection error"

- Check database is running
- Verify connection string
- Ensure migrations have been run

## Contributing

When adding features:

1. Update this README
2. Add unit tests
3. Update example code
4. Document breaking changes

## License

Same as parent project
