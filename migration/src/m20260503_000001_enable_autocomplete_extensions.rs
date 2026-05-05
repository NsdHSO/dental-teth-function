use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        conn.execute_unprepared("CREATE EXTENSION IF NOT EXISTS pg_trgm").await?;
        conn.execute_unprepared("CREATE EXTENSION IF NOT EXISTS unaccent").await?;

        // Wrap unaccent in an IMMUTABLE function so it can be used in expression indexes.
        // SET search_path pins resolution of `unaccent` regardless of the caller's session
        // search_path (pool connections may not have `dental, public` set).
        conn.execute_unprepared(
            r#"
            CREATE OR REPLACE FUNCTION dental.f_unaccent(text) RETURNS text
            LANGUAGE sql IMMUTABLE PARALLEL SAFE STRICT
            SET search_path = dental, public AS $$
              SELECT unaccent($1)
            $$;
            "#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();
        conn.execute_unprepared("DROP FUNCTION IF EXISTS dental.f_unaccent(text)").await?;
        // Leave the extensions installed; they are cluster-wide and may be in use elsewhere.
        Ok(())
    }
}
