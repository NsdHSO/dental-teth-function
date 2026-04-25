use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};
use sea_orm_migration::prelude::*;
use sea_orm_migration::SchemaManager;
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    // Get DATABASE_URL from environment
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    println!("Connecting to database...");
    let db = Database::connect(&db_url).await?;

    // Set search_path to church schema FIRST
    println!("Setting search_path to church, public...");
    db.execute_unprepared("SET search_path TO church, public").await?;

    // Create seaql_migrations table in church schema if it doesn't exist
    println!("Ensuring church.seaql_migrations table exists...");
    db.execute_unprepared(
        "CREATE TABLE IF NOT EXISTS church.seaql_migrations (
            version character varying NOT NULL,
            applied_at bigint NOT NULL,
            CONSTRAINT seaql_migrations_pkey PRIMARY KEY (version)
        )"
    ).await?;

    println!("Checking for pending migrations...");

    // Get already applied migrations from church.seaql_migrations
    let applied_migrations = get_applied_migrations(&db).await?;
    println!("Found {} applied migrations in church.seaql_migrations", applied_migrations.len());

    // Get all migrations from the migrator
    let migrations = migration::Migrator::migrations();
    let mut pending_count = 0;
    let mut applied_count = 0;

    // Create SchemaManager for running migrations
    let manager = SchemaManager::new(&db);

    for migration in migrations {
        let version = migration.name().to_string();

        if applied_migrations.contains(&version) {
            applied_count += 1;
            continue; // Skip already applied migrations
        }

        pending_count += 1;
        println!("Applying migration '{}'", version);

        // Run the migration
        migration.up(&manager).await?;

        // Record in church.seaql_migrations
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        db.execute(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "INSERT INTO church.seaql_migrations (version, applied_at) VALUES ($1, $2)",
            vec![version.clone().into(), now.into()]
        )).await?;

        println!("Migration '{}' has been applied", version);
    }

    if pending_count == 0 {
        println!("✅ No pending migrations (all {} migrations already applied)", applied_count);
    } else {
        println!("✅ Applied {} new migration(s), {} were already applied", pending_count, applied_count);
    }

    // Verify final count
    let result = db.query_one(
        Statement::from_string(
            DbBackend::Postgres,
            "SELECT COUNT(*) as count FROM church.seaql_migrations".to_string()
        )
    ).await?;

    if let Some(row) = result {
        let count: i64 = row.try_get("", "count")?;
        println!("📊 Total migrations tracked in church.seaql_migrations: {}", count);
    }

    Ok(())
}

async fn get_applied_migrations(db: &sea_orm::DatabaseConnection) -> Result<HashSet<String>, DbErr> {
    let rows = db.query_all(
        Statement::from_string(
            DbBackend::Postgres,
            "SELECT version FROM church.seaql_migrations".to_string()
        )
    ).await?;

    let mut applied = HashSet::new();
    for row in rows {
        let version: String = row.try_get("", "version")?;
        applied.insert(version);
    }

    Ok(applied)
}
