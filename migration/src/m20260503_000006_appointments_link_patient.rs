use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        conn.execute_unprepared(
            "ALTER TABLE dental.appointments ADD COLUMN IF NOT EXISTS patient_id BIGINT",
        )
        .await?;

        // Backfill: for each existing appointment, create a synthetic user + user_profile + patient
        // and link the appointment. Wrapped in an existence check so the migration is replayable
        // after the legacy patient_* columns have already been dropped.
        conn.execute_unprepared(
            r#"
            DO $$
            DECLARE
                appt RECORD;
                new_user_id    BIGINT;
                new_patient_id BIGINT;
                synth_sub      TEXT;
            BEGIN
                IF NOT EXISTS (
                    SELECT 1 FROM information_schema.columns
                    WHERE table_schema = 'dental'
                      AND table_name   = 'appointments'
                      AND column_name  = 'patient_name'
                ) THEN
                    RETURN;
                END IF;

                FOR appt IN SELECT id, patient_name, patient_phone, patient_email
                            FROM dental.appointments
                            WHERE patient_id IS NULL
                LOOP
                    synth_sub := 'local:patient:' || gen_random_uuid()::text;

                    INSERT INTO dental.users (auth_user_id)
                    VALUES (synth_sub)
                    RETURNING id INTO new_user_id;

                    INSERT INTO dental.user_profiles
                        (user_sub, user_id, schema_version, attributes,
                         full_name, phone, email)
                    VALUES
                        (synth_sub, new_user_id, '1.0', '{}'::jsonb,
                         appt.patient_name, appt.patient_phone, appt.patient_email);

                    INSERT INTO dental.patients (user_id)
                    VALUES (new_user_id)
                    RETURNING id INTO new_patient_id;

                    UPDATE dental.appointments
                    SET    patient_id = new_patient_id
                    WHERE  id = appt.id;
                END LOOP;
            END $$;
            "#,
        )
        .await?;

        conn.execute_unprepared(
            r#"
            ALTER TABLE dental.appointments
                ALTER COLUMN patient_id SET NOT NULL,
                DROP CONSTRAINT IF EXISTS fk_appointments_patient_id,
                ADD CONSTRAINT fk_appointments_patient_id
                    FOREIGN KEY (patient_id) REFERENCES dental.patients(id);
            "#,
        )
        .await?;

        conn.execute_unprepared(
            r#"
            ALTER TABLE dental.appointments
                DROP COLUMN IF EXISTS patient_name,
                DROP COLUMN IF EXISTS patient_phone,
                DROP COLUMN IF EXISTS patient_email;
            "#,
        )
        .await?;

        conn.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_appointments_patient_id ON dental.appointments (patient_id)",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();

        conn.execute_unprepared(
            r#"
            ALTER TABLE dental.appointments
                ADD COLUMN patient_name  TEXT,
                ADD COLUMN patient_phone TEXT,
                ADD COLUMN patient_email TEXT;
            "#,
        )
        .await?;

        // Best-effort restore of inline fields from the backfilled identity columns.
        conn.execute_unprepared(
            r#"
            UPDATE dental.appointments a
            SET    patient_name  = up.full_name,
                   patient_phone = up.phone,
                   patient_email = up.email
            FROM   dental.patients p
            JOIN   dental.user_profiles up ON up.user_id = p.user_id
            WHERE  a.patient_id = p.id;
            "#,
        )
        .await?;

        conn.execute_unprepared(
            r#"
            ALTER TABLE dental.appointments
                ALTER COLUMN patient_name SET NOT NULL;
            "#,
        )
        .await?;

        conn.execute_unprepared(
            "DROP INDEX IF EXISTS dental.idx_appointments_patient_id",
        )
        .await?;

        conn.execute_unprepared(
            r#"
            ALTER TABLE dental.appointments
                DROP CONSTRAINT IF EXISTS fk_appointments_patient_id,
                DROP COLUMN IF EXISTS patient_id;
            "#,
        )
        .await?;

        Ok(())
    }
}
