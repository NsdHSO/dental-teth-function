use doppler_rs::apis::{configuration::Configuration, default_api};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ConfigService {
    pub access_token_public_key: String,
    pub app_env: String,
    pub auth_base_url: String,
    pub auth_api_key: Option<String>,
    pub database_public_url: String,
    pub database_url: String,
    pub doppler_env: String,
    pub host: String,
    pub pgdata: String,
    pub pgdatabase: String,
    pub pghost: String,
    pub pgpassword: String,
    pub pgport: String,
    pub pguser: String,
    pub port: u16,
    pub postgres_db: String,
    pub postgres_password: String,
    pub postgres_user: String,
    pub railway_deployment_draining_seconds: String,
    pub rust_log: String,
    pub sqlx_log: bool,
    pub ssl_cert_days: String,
    pub strapi_api: String,
    pub mode: String,
}

impl ConfigService {
    pub async fn new() -> Self {
        // Try to fetch secrets from Doppler
        let secrets = Self::fetch_from_doppler().await;

        ConfigService {
            access_token_public_key: Self::get_value(&secrets, "ACCESS_TOKEN_PUBLIC_KEY", ""),
            app_env: Self::get_value(&secrets, "APP_ENV", "dev"),
            auth_base_url: Self::get_value(&secrets, "AUTH_BASE_URL", "http://localhost:8081"),
            auth_api_key: Self::get_optional_value(&secrets, "AUTH_API_KEY"),
            database_public_url: Self::get_value(&secrets, "DATABASE_PUBLIC_URL", ""),
            database_url: Self::get_value_required(&secrets, "DATABASE_URL"),
            doppler_env: Self::get_value(&secrets, "DOPPLER_ENV", ""),
            host: Self::get_value(&secrets, "HOST", "127.0.0.1"),
            pgdata: Self::get_value(&secrets, "PGDATA", ""),
            pgdatabase: Self::get_value(&secrets, "PGDATABASE", ""),
            pghost: Self::get_value(&secrets, "PGHOST", ""),
            pgpassword: Self::get_value(&secrets, "PGPASSWORD", ""),
            pgport: Self::get_value(&secrets, "PGPORT", ""),
            pguser: Self::get_value(&secrets, "PGUSER", ""),
            port: Self::get_value(&secrets, "PORT", "8080")
                .parse::<u16>()
                .expect("PORT must be a valid u16"),
            postgres_db: Self::get_value(&secrets, "POSTGRES_DB", ""),
            postgres_password: Self::get_value(&secrets, "POSTGRES_PASSWORD", ""),
            postgres_user: Self::get_value(&secrets, "POSTGRES_USER", ""),
            railway_deployment_draining_seconds: Self::get_value(
                &secrets,
                "RAILWAY_DEPLOYMENT_DRAINING_SECONDS",
                "",
            ),
            rust_log: Self::get_value(&secrets, "RUST_LOG", "info"),
            sqlx_log: Self::get_value(&secrets, "SQLX_LOG", "false")
                .parse()
                .unwrap_or(false),
            ssl_cert_days: Self::get_value(&secrets, "SSL_CERT_DAYS", ""),
            strapi_api: Self::get_value(&secrets, "STRAPI_API", ""),
            mode: Self::get_value_required(&secrets, "MODE"),
        }
    }

    async fn fetch_from_doppler() -> Option<HashMap<String, String>> {
        // Check if DOPPLER_TOKEN is set (service token)
        let doppler_token = match std::env::var("DOPPLER_TOKEN") {
            Ok(token) => token,
            Err(_) => {
                println!("DOPPLER_TOKEN not found, falling back to local environment variables");
                return None;
            }
        };

        // Determine project and config from env
        let project = match std::env::var("DOPPLER_PROJECT") {
            Ok(p) if !p.is_empty() => p,
            _ => {
                eprintln!(
                    "DOPPLER_PROJECT not set; cannot query Doppler. Falling back to local env."
                );
                return None;
            }
        };
        // Support either DOPPLER_ENV or DOPPLER_CONFIG, default to "dev"
        let config_name = std::env::var("DOPPLER_ENV")
            .ok()
            .filter(|v| !v.is_empty())
            .or_else(|| std::env::var("DOPPLER_CONFIG").ok())
            .unwrap_or_else(|| "dev".to_string());

        // Create Doppler configuration with bearer token
        let mut cfg = Configuration::new();
        cfg.bearer_access_token = Some(doppler_token);

        // Fetch a fixed set of known keys using `secrets_get`, like the working gandul-function project.
        let keys = [
            "ACCESS_TOKEN_PUBLIC_KEY",
            "APP_ENV",
            "AUTH_BASE_URL",
            "AUTH_API_KEY",
            "DATABASE_PUBLIC_URL",
            "DATABASE_URL",
            "DOPPLER_ENV",
            "HOST",
            "PGDATA",
            "PGDATABASE",
            "PGHOST",
            "PGPASSWORD",
            "PGPORT",
            "PGUSER",
            "PORT",
            "POSTGRES_DB",
            "POSTGRES_PASSWORD",
            "POSTGRES_USER",
            "RAILWAY_DEPLOYMENT_DRAINING_SECONDS",
            "RUST_LOG",
            "SQLX_LOG",
            "SSL_CERT_DAYS",
            "STRAPI_API",
            "MODE",
        ];

        let mut out: HashMap<String, String> = HashMap::new();
        for key in keys {
            match default_api::secrets_get(&cfg, &project, &config_name, key).await {
                Ok(resp) => {
                    if let Some(val) = resp
                        .value
                        .as_ref()
                        .and_then(|v| v.computed.as_ref())
                        .map(|s| s.to_string())
                    {
                        out.insert(key.to_string(), val);
                    }
                }
                Err(err) => {
                    // Non-fatal: skip missing keys
                    eprintln!("Doppler: could not fetch {key}: {err:?}");
                }
            }
        }

        if out.is_empty() {
            println!(
                "No secrets fetched from Doppler, falling back to local environment variables"
            );
            None
        } else {
            println!("Successfully fetched {} secrets from Doppler", out.len());
            Some(out)
        }
    }

    fn get_value(secrets: &Option<HashMap<String, String>>, key: &str, default: &str) -> String {
        secrets
            .as_ref()
            .and_then(|s| s.get(key).cloned())
            .or_else(|| std::env::var(key).ok())
            .unwrap_or_else(|| default.to_string())
    }

    fn get_value_required(secrets: &Option<HashMap<String, String>>, key: &str) -> String {
        secrets
            .as_ref()
            .and_then(|s| s.get(key).cloned())
            .or_else(|| std::env::var(key).ok())
            .unwrap_or_else(|| panic!("{} must be set", key))
    }

    fn get_optional_value(secrets: &Option<HashMap<String, String>>, key: &str) -> Option<String> {
        secrets
            .as_ref()
            .and_then(|s| s.get(key).cloned())
            .or_else(|| std::env::var(key).ok())
    }
}
