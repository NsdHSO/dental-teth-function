use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        conn.execute_unprepared(
            r#"
            ALTER TABLE dental.user_profiles
                ADD COLUMN user_id   BIGINT,
                ADD COLUMN full_name TEXT,
                ADD COLUMN phone     TEXT,
                ADD COLUMN email     TEXT,
                ADD COLUMN cnp       TEXT;
            "#,
        )
        .await?;

        // Backfill user_id from the user_sub <-> users.auth_user_id mapping.
        conn.execute_unprepared(
            r#"
            UPDATE dental.user_profiles up
            SET    user_id = u.id
            FROM   dental.users u
            WHERE  u.auth_user_id = up.user_sub;
            "#,
        )
        .await?;

        conn.execute_unprepared(
            r#"
            ALTER TABLE dental.user_profiles
                ALTER COLUMN user_id SET NOT NULL,
                ADD CONSTRAINT fk_user_profiles_user_id
                    FOREIGN KEY (user_id) REFERENCES dental.users(id) ON DELETE CASCADE,
                ADD CONSTRAINT uq_user_profiles_user_id UNIQUE (user_id);
            "#,
        )
        .await?;

        conn.execute_unprepared(
            r#"
            CREATE INDEX idx_up_full_name_trgm ON dental.user_profiles
              USING gin (dental.f_unaccent(lower(full_name)) gin_trgm_ops);
            "#,
        )
        .await?;

        conn.execute_unprepared(
            r#"
            CREATE INDEX idx_up_email_trgm ON dental.user_profiles
              USING gin (dental.f_unaccent(lower(email)) gin_trgm_ops);
            "#,
        )
        .await?;

        conn.execute_unprepared(
            "CREATE INDEX idx_up_phone ON dental.user_profiles (phone)",
        )
        .await?;
        conn.execute_unprepared(
            "CREATE INDEX idx_up_cnp ON dental.user_profiles (cnp)",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        conn.execute_unprepared("DROP INDEX IF EXISTS dental.idx_up_cnp").await?;
        conn.execute_unprepared("DROP INDEX IF EXISTS dental.idx_up_phone").await?;
        conn.execute_unprepared("DROP INDEX IF EXISTS dental.idx_up_email_trgm").await?;
        conn.execute_unprepared("DROP INDEX IF EXISTS dental.idx_up_full_name_trgm").await?;

        conn.execute_unprepared(
            r#"
            ALTER TABLE dental.user_profiles
                DROP CONSTRAINT IF EXISTS uq_user_profiles_user_id,
                DROP CONSTRAINT IF EXISTS fk_user_profiles_user_id,
                DROP COLUMN IF EXISTS cnp,
                DROP COLUMN IF EXISTS email,
                DROP COLUMN IF EXISTS phone,
                DROP COLUMN IF EXISTS full_name,
                DROP COLUMN IF EXISTS user_id;
            "#,
        )
        .await?;

        Ok(())
    }
}
