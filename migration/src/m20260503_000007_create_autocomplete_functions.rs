use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        conn.execute_unprepared(
            r#"
            CREATE OR REPLACE FUNCTION dental.fn_autocomplete_score(
                needle text,
                haystack text
            ) RETURNS real
            LANGUAGE sql IMMUTABLE PARALLEL SAFE AS $$
              SELECT COALESCE(
                similarity(
                  dental.f_unaccent(lower(coalesce(needle, ''))),
                  dental.f_unaccent(lower(coalesce(haystack, '')))
                ),
                0
              )
            $$;
            "#,
        )
        .await?;

        conn.execute_unprepared(
            r#"
            CREATE OR REPLACE FUNCTION dental.search_patients(
                q text,
                max_results int DEFAULT 10
            ) RETURNS TABLE (
                patient_id bigint,
                user_id    bigint,
                full_name  text,
                phone      text,
                email      text,
                cnp        text,
                score      real
            )
            LANGUAGE sql STABLE PARALLEL SAFE AS $$
              WITH q_norm AS (
                SELECT dental.f_unaccent(lower(coalesce(q, ''))) AS n
              )
              SELECT
                p.id,
                up.user_id,
                up.full_name,
                up.phone,
                up.email,
                up.cnp,
                GREATEST(
                  dental.fn_autocomplete_score(q, up.full_name),
                  dental.fn_autocomplete_score(q, up.phone),
                  dental.fn_autocomplete_score(q, up.email),
                  dental.fn_autocomplete_score(q, up.cnp)
                ) AS score
              FROM dental.patients p
              JOIN dental.user_profiles up ON up.user_id = p.user_id
              WHERE
                dental.f_unaccent(lower(coalesce(up.full_name, ''))) % (SELECT n FROM q_norm)
                OR dental.f_unaccent(lower(coalesce(up.email, '')))  % (SELECT n FROM q_norm)
                OR coalesce(up.phone, '') ILIKE '%' || q || '%'
                OR coalesce(up.cnp, '')   ILIKE '%' || q || '%'
              ORDER BY score DESC, up.full_name ASC
              LIMIT GREATEST(max_results, 1)
            $$;
            "#,
        )
        .await?;

        conn.execute_unprepared(
            r#"
            CREATE OR REPLACE FUNCTION dental.search_dentists(
                q text,
                max_results int DEFAULT 10
            ) RETURNS TABLE (
                dentist_id    bigint,
                user_id       bigint,
                full_name     text,
                specialty     text,
                is_available  boolean,
                score         real
            )
            LANGUAGE sql STABLE PARALLEL SAFE AS $$
              WITH q_norm AS (
                SELECT dental.f_unaccent(lower(coalesce(q, ''))) AS n
              )
              SELECT
                d.id,
                up.user_id,
                up.full_name,
                d.specialty,
                d.is_available,
                GREATEST(
                  dental.fn_autocomplete_score(q, up.full_name),
                  dental.fn_autocomplete_score(q, d.specialty)
                ) AS score
              FROM dental.dentists d
              JOIN dental.user_profiles up ON up.user_id = d.user_id
              WHERE
                dental.f_unaccent(lower(coalesce(up.full_name, ''))) % (SELECT n FROM q_norm)
                OR dental.f_unaccent(lower(coalesce(d.specialty, ''))) % (SELECT n FROM q_norm)
              ORDER BY score DESC, up.full_name ASC
              LIMIT GREATEST(max_results, 1)
            $$;
            "#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();
        conn.execute_unprepared("DROP FUNCTION IF EXISTS dental.search_dentists(text, int)").await?;
        conn.execute_unprepared("DROP FUNCTION IF EXISTS dental.search_patients(text, int)").await?;
        conn.execute_unprepared("DROP FUNCTION IF EXISTS dental.fn_autocomplete_score(text, text)").await?;
        Ok(())
    }
}
