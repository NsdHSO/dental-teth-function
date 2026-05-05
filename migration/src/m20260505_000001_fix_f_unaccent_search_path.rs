use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        // Pin `search_path` inside the function body so `unaccent` resolves
        // regardless of the caller's session search_path. The runtime pool
        // does not set `search_path`, so without this clause `unaccent`
        // (installed under `dental`) is invisible to pool connections.
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

        // Revert to the original body without the SET clause.
        conn.execute_unprepared(
            r#"
            CREATE OR REPLACE FUNCTION dental.f_unaccent(text) RETURNS text
            LANGUAGE sql IMMUTABLE PARALLEL SAFE STRICT AS $$
              SELECT unaccent($1)
            $$;
            "#,
        )
        .await?;

        Ok(())
    }
}
