use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use promptfoo_rs::cache::resume::{ResumeRecord, ResumeStore};
use promptfoo_rs::cache::{cache_key, CacheKeyInput, TestCaseKeyInput};
use promptfoo_rs::eval::retry::{retry_with_backoff, BackoffSchedule, RetryPolicy};
use serde_json::json;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

#[test]
fn test_3_2_1_cache_key_fixture_covers_provider_config_and_case() {
    let mut vars = BTreeMap::new();
    vars.insert("name".to_string(), "Ada".to_string());

    let input = CacheKeyInput {
        provider_id: "openai:gpt-4o-mini".to_string(),
        provider_config: json!({
            "model": "gpt-4o-mini",
            "temperature": 0,
        }),
        prompt: "Hello {{name}}".to_string(),
        test_case: TestCaseKeyInput {
            vars,
            assertions: vec![json!({"type": "contains", "value": "Ada"})],
        },
    };

    let key = cache_key(&input);
    assert!(key.starts_with("sha256:"), "{key}");
    assert_eq!(key.len(), "sha256:".len() + 64);
    assert_eq!(key, cache_key(&input));

    let mut changed_provider = input.clone();
    changed_provider.provider_config["temperature"] = json!(1);
    assert_ne!(key, cache_key(&changed_provider));

    let mut changed_case = input.clone();
    changed_case
        .test_case
        .vars
        .insert("name".to_string(), "Grace".to_string());
    assert_ne!(key, cache_key(&changed_case));
}

#[tokio::test]
async fn test_3_2_2_resume_from_partial_jsonl_and_sqlite_state() {
    let fixture = FixtureDir::new("test_3_2_2");
    let jsonl_path = fixture.path("partial-results.jsonl");
    fixture.write(
        "partial-results.jsonl",
        r#"{"case_id":"case-1","cache_key":"sha256:aaa","status":"completed","output":"one"}
not-json
{"case_id":"case-3","cache_key":"sha256:ccc","status":"completed","output":"three"}
"#,
    );

    let jsonl_state = ResumeStore::load(&jsonl_path).expect("TEST-3.2.2 JSONL state should load");
    assert_eq!(jsonl_state.completed_case_ids(), vec!["case-1", "case-3"]);
    assert_eq!(jsonl_state.corrupt_records.len(), 1);
    assert_eq!(
        jsonl_state.remaining_cases(["case-1", "case-2", "case-3"]),
        vec!["case-2"]
    );

    let sqlite_path = fixture.path("partial-results.sqlite");
    seed_sqlite(
        &sqlite_path,
        &[
            ResumeRecord::completed("case-1", "sha256:aaa", "one"),
            ResumeRecord::completed("case-3", "sha256:ccc", "three"),
        ],
    )
    .await;

    let sqlite_state =
        ResumeStore::load(&sqlite_path).expect("TEST-3.2.2 SQLite state should load");
    assert_eq!(sqlite_state.completed_case_ids(), vec!["case-1", "case-3"]);
    assert_eq!(
        sqlite_state.remaining_cases(["case-1", "case-2", "case-3"]),
        vec!["case-2"]
    );
}

#[tokio::test]
async fn test_3_2_3_retry_errors_and_backoff_failure_path_is_reproducible() {
    let policy = RetryPolicy {
        max_attempts: 3,
        retry_errors: vec!["rate limit".to_string()],
        backoff: BackoffSchedule::from_millis([10, 20]),
    };
    let attempts = Arc::new(Mutex::new(Vec::new()));

    let failure = retry_with_backoff(policy, {
        let attempts = attempts.clone();
        move |attempt| {
            let attempts = attempts.clone();
            async move {
                attempts.lock().unwrap().push(attempt);
                Err::<(), _>("rate limit: 429".to_string())
            }
        }
    })
    .await
    .expect_err("TEST-3.2.3 should expose deterministic retry failure");

    assert_eq!(*attempts.lock().unwrap(), vec![1, 2, 3]);
    assert_eq!(failure.attempts, 3);
    assert_eq!(
        failure.backoff_delays,
        vec![Duration::from_millis(10), Duration::from_millis(20)]
    );
    assert_eq!(failure.errors.len(), 3);

    let non_retryable = retry_with_backoff(
        RetryPolicy {
            max_attempts: 3,
            retry_errors: vec!["rate limit".to_string()],
            backoff: BackoffSchedule::from_millis([10, 20]),
        },
        |_attempt| async { Err::<(), _>("validation error".to_string()) },
    )
    .await
    .expect_err("non-retryable errors should stop after first attempt");

    assert_eq!(non_retryable.attempts, 1);
    assert!(non_retryable.backoff_delays.is_empty());
}

async fn seed_sqlite(path: &Path, records: &[ResumeRecord]) {
    let options = SqliteConnectOptions::from_str(&path.to_string_lossy())
        .expect("sqlite path should parse")
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .expect("sqlite fixture should open");

    sqlx::query(
        "CREATE TABLE results (
            case_id TEXT PRIMARY KEY NOT NULL,
            cache_key TEXT NOT NULL,
            status TEXT NOT NULL,
            output TEXT
        )",
    )
    .execute(&pool)
    .await
    .expect("results table should be created");

    for record in records {
        sqlx::query(
            "INSERT INTO results (case_id, cache_key, status, output)
             VALUES (?1, ?2, ?3, ?4)",
        )
        .bind(&record.case_id)
        .bind(&record.cache_key)
        .bind(&record.status)
        .bind(&record.output)
        .execute(&pool)
        .await
        .expect("result row should insert");
    }

    pool.close().await;
}

struct FixtureDir {
    root: PathBuf,
}

impl FixtureDir {
    fn new(name: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("promptfoo-rs-{name}-{nonce}"));
        fs::create_dir_all(&root).expect("fixture root should be created");
        Self { root }
    }

    fn path(&self, relative: &str) -> PathBuf {
        self.root.join(relative)
    }

    fn write(&self, relative: &str, contents: &str) {
        let path = self.path(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("fixture parent should be created");
        }
        fs::write(path, contents).expect("fixture file should be written");
    }
}

impl Drop for FixtureDir {
    fn drop(&mut self) {
        if Path::new(&self.root).exists() {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}
