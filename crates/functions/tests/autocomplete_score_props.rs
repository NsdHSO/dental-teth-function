use proptest::prelude::*;
use sea_orm::{Database, DatabaseConnection, DatabaseBackend, FromQueryResult, Statement};
use std::sync::OnceLock;

#[derive(FromQueryResult)]
struct ScoreRow {
    s: f32,
}

fn db() -> &'static DatabaseConnection {
    static DB: OnceLock<DatabaseConnection> = OnceLock::new();
    DB.get_or_init(|| {
        let url = std::env::var("DATABASE_URL_TEST").expect("DATABASE_URL_TEST");
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async { Database::connect(&url).await.expect("connect") })
    })
}

fn score(q: &str, candidate: &str) -> f32 {
    let conn = db();
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let stmt = Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            "SELECT dental.fn_autocomplete_score($1, $2)::real AS s",
            [q.into(), candidate.into()],
        );
        ScoreRow::find_by_statement(stmt)
            .one(conn)
            .await
            .expect("score row")
            .expect("score exists")
            .s
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]

    #[test]
    fn score_is_bounded(q in "[\\PC]{1,32}", c in "[\\PC]{1,64}") {
        let s = score(&q, &c);
        prop_assert!(s.is_finite());
        prop_assert!(s >= 0.0);
        prop_assert!(s <= 3.5, "score must stay within the documented ceiling, got {s}");
    }

    #[test]
    fn score_is_diacritic_insensitive(q in "[a-zA-Z]{2,16}") {
        let plain = "Popescu";
        let accented = "Popéscu";
        let a = score(&q, plain);
        let b = score(&q, accented);
        prop_assert!((a - b).abs() < 1e-3, "diacritics must not change score: {a} vs {b}");
    }
}

#[test]
fn prefix_beats_typo() {
    let prefix = score("ion", "Ionescu");
    let typo = score("xon", "Ionescu");
    assert!(prefix >= typo, "true prefix must score >= typo: {prefix} vs {typo}");
}
