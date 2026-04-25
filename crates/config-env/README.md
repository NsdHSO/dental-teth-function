# Config Environment Service

This crate provides a `ConfigService` that fetches environment variables from Doppler using `doppler-rs` and falls back
to local environment variables.

## Features

- Fetches all 22+ environment variables from Doppler
- Automatic fallback to local environment variables
- Type-safe configuration management
- Uses `doppler-rs` for authentication and API access

## Environment Variables

The service manages the following configuration:

- `ACCESS_TOKEN_PUBLIC_KEY`
- `APP_ENV`
- `AUTH_BASE_URL`
- `DATABASE_PUBLIC_URL`
- `DATABASE_URL` (required)
- `DOPPLER_ENV`
- `HOST`
- `PGDATA`
- `PGDATABASE`
- `PGHOST`
- `PGPASSWORD`
- `PGPORT`
- `PGUSER`
- `PORT`
- `POSTGRES_DB`
- `POSTGRES_PASSWORD`
- `POSTGRES_USER`
- `RAILWAY_DEPLOYMENT_DRAINING_SECONDS`
- `RUST_LOG`
- `SQLX_LOG`
- `SSL_CERT_DAYS`
- `STRAPI_API`

## Usage

### Setup with Doppler

1. Set your Doppler service token:

```bash
export DOPPLER_TOKEN=dp.st.your-service-token-here
```

2. Optionally set project and config (if not using service token defaults):

```bash
export DOPPLER_PROJECT=your-project
export DOPPLER_CONFIG=dev
```

### Using the ConfigService

```rust
use config_env::ConfigService;

#[tokio::main]
async fn main() {
    // Load configuration from Doppler (or local env as fallback)
    let config = ConfigService::new().await;

    println!("App running on {}:{}", config.host, config.port);
    println!("Database URL: {}", config.database_url);
    println!("Environment: {}", config.app_env);
}
```

## How It Works

1. **Doppler First**: If `DOPPLER_TOKEN` is set, the service uses `doppler-rs` to fetch all secrets from Doppler via the
   `secrets_download` API endpoint
2. **Local Fallback**: If Doppler is unavailable or `DOPPLER_TOKEN` is not set, it falls back to reading from local
   environment variables
3. **Defaults**: Sensible defaults are provided for non-critical configuration values

## Authentication

The service uses `doppler-rs` with bearer token authentication. You can use either:

- **Service Token** (recommended): `dp.st.xxx` - scoped to a specific project and config
- **Personal Token**: `dp.pt.xxx` - requires `DOPPLER_PROJECT` and `DOPPLER_CONFIG` to be set

## Error Handling

- Missing `DOPPLER_TOKEN`: Falls back to local environment variables
- Doppler API errors: Falls back to local environment variables with error logging
- Missing required variables (like `DATABASE_URL`): Application panics with clear error message
