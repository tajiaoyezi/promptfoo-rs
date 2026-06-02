use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use promptfoo_rs::compatibility::inventory::{
    extract_current_latest_inventory, CurrentLatestTargetLock,
};
use promptfoo_rs::results::{
    AssertionResultRecord, ResultQuery, ResultRecord, ResultStatus, SqliteResultStore,
};
use serde_json::{json, Value};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

const NPM_VIEW: &str = r#"{
  "version": "0.121.13",
  "gitHead": "4860e990c7e9a2f8f677173fb92cf9867b34d03f",
  "dist": {
    "tarball": "https://registry.npmjs.org/promptfoo/-/promptfoo-0.121.13.tgz",
    "integrity": "sha512-DBPSixUophzcD7S7lML6SqVwnVtrhK5A3HsZ03IG9Xrw0t24r5imG7nLj+YMb0vlAjbdFtE7yFG+rsqDpfYp6g=="
  },
  "time": {
    "modified": "2026-05-28T23:59:40.582Z"
  }
}"#;

const GITHUB_LATEST_RELEASE: &str = r#"{
  "tag_name": "code-scan-action-0.1.7",
  "name": "code-scan-action: 0.1.7",
  "target_commitish": "1c743afe0e4807882e858c4f322fc064fa5f0770",
  "published_at": "2026-05-29T03:02:57Z",
  "html_url": "https://github.com/promptfoo/promptfoo/releases/tag/code-scan-action-0.1.7"
}"#;

const LS_REMOTE: &str = "\
1d09dfeb5f0766905409117f923dd5c4b0838d9f\tHEAD
4860e990c7e9a2f8f677173fb92cf9867b34d03f\trefs/tags/0.121.13
1c743afe0e4807882e858c4f322fc064fa5f0770\trefs/tags/code-scan-action-0.1.7
";

#[tokio::test]
async fn test_33_1_1_delete_eval_removes_selected_results_and_assertions() {
    /* TEST-33.1.1 */
    let path = temp_path("delete-selected", "db");
    let store = SqliteResultStore::open(&path).expect("sqlite store opens");
    store
        .insert(&result_record("eval-delete", "case-1", "delete-assert"))
        .expect("first target record inserts");
    store
        .insert(&result_record("eval-delete", "case-2", "delete-assert"))
        .expect("second target record inserts");
    store
        .insert(&result_record("eval-keep", "case-1", "keep-assert"))
        .expect("unrelated record inserts");

    let deleted = store
        .delete_eval("eval-delete")
        .expect("eval deletion succeeds");

    assert_eq!(deleted, 2);
    assert!(store
        .query(ResultQuery::new().eval_id("eval-delete"))
        .expect("query deleted eval")
        .is_empty());
    assert_eq!(
        store
            .query(ResultQuery::new().eval_id("eval-keep"))
            .expect("query kept eval")
            .len(),
        1
    );
    assert_eq!(assertion_count(&path, "delete-assert").await, 0);
    assert_eq!(assertion_count(&path, "keep-assert").await, 1);

    let _ = fs::remove_file(path);
}

#[tokio::test]
async fn test_33_1_2_delete_missing_eval_is_noop_and_preserves_cache_rows() {
    /* TEST-33.1.2 */
    let path = temp_path("delete-missing", "db");
    let store = SqliteResultStore::open(&path).expect("sqlite store opens");
    store
        .insert(&result_record("eval-keep", "case-1", "keep-assert"))
        .expect("record inserts");

    let deleted = store
        .delete_eval("eval-missing")
        .expect("missing eval deletion succeeds");

    assert_eq!(deleted, 0);
    assert_eq!(
        store
            .query(ResultQuery::new().eval_id("eval-keep"))
            .expect("query kept eval"),
        vec![result_record("eval-keep", "case-1", "keep-assert")]
    );
    assert_eq!(assertion_count(&path, "keep-assert").await, 1);

    let _ = fs::remove_file(path);
}

#[test]
fn test_33_1_3_eval_deletion_row_has_native_fixture_evidence() {
    /* TEST-33.1.3 */
    let root = fixture_dir("rust-fixture");
    write_eval_deletion_source(&root);
    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");
    let row = cache_store_row_for_source(&inventory.rows, "src/database/evalDeletion.ts");

    assert_eq!(row.level, "P0", "{row:#?}");
    assert_eq!(row.implementation_status, "native", "{row:#?}");
    assert_eq!(row.verification_owner, "cache-resume-store", "{row:#?}");
    assert_eq!(row.evidence_kind, "fixture", "{row:#?}");
    assert_eq!(
        row.evidence_reference, "fixture:cache-store:src-database-evaldeletion",
        "{row:#?}"
    );
    assert!(!row
        .blocker_reason
        .as_deref()
        .unwrap_or_default()
        .contains("dedicated current-latest cache-store evidence is required"));

    let _ = fs::remove_dir_all(root);
}

#[test]
fn test_33_1_4_script_and_rust_extractors_drop_cache_store_blocker() {
    /* TEST-33.1.4 */
    let root = fixture_dir("script-source");
    write_eval_deletion_source(&root);
    let gate_dir = fixture_dir("script-gate");
    run_current_latest_source_inventory_script(&root, &gate_dir);
    run_current_latest_script(&gate_dir, "scripts/release/current-latest-golden-corpus.sh");
    run_current_latest_script(&gate_dir, "scripts/release/current-latest-quality-gate.sh");

    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");
    let rust_row = cache_store_row_for_source(&inventory.rows, "src/database/evalDeletion.ts");
    let script = read_json(&gate_dir.join("current-latest-source-inventory.json"));
    let script_rows = script["rows"]
        .as_array()
        .expect("script rows should be an array");
    let shell_row = script_rows
        .iter()
        .find(|row| {
            row["stable_id"] == Value::String("cache-store:src-database-evaldeletion".to_string())
        })
        .expect("shell eval deletion row exists");

    assert_eq!(
        (
            rust_row.level.as_str(),
            rust_row.implementation_status.as_str(),
            rust_row.verification_owner.as_str(),
            rust_row.evidence_kind.as_str(),
            rust_row.evidence_reference.as_str(),
        ),
        (
            shell_row["level"].as_str().unwrap_or_default(),
            shell_row["implementation_status"]
                .as_str()
                .unwrap_or_default(),
            shell_row["verification_owner"].as_str().unwrap_or_default(),
            shell_row["evidence_kind"].as_str().unwrap_or_default(),
            shell_row["evidence_reference"].as_str().unwrap_or_default(),
        )
    );

    let golden = read_json(&gate_dir.join("current-latest-golden-corpus.json"));
    let blockers = golden["release_blockers"]
        .as_array()
        .expect("golden blockers should be an array");
    assert!(
        blockers.iter().all(|blocker| !blocker["capability"]
            .as_str()
            .unwrap_or_default()
            .starts_with("cache-store:")),
        "{blockers:#?}"
    );
    assert_eq!(golden["blocker_count"], Value::from(0));

    let _ = fs::remove_dir_all(root);
    let _ = fs::remove_dir_all(gate_dir);
}

fn result_record(eval_id: &str, case_id: &str, assertion_type: &str) -> ResultRecord {
    ResultRecord {
        eval_id: eval_id.to_string(),
        case_id: case_id.to_string(),
        provider_id: "echo".to_string(),
        status: ResultStatus::Passed,
        result: Some(json!({ "output": case_id })),
        assertion_results: vec![AssertionResultRecord {
            assertion_type: assertion_type.to_string(),
            status: ResultStatus::Passed,
            message: Some("ok".to_string()),
        }],
        latency_ms: 7,
        metadata: json!({ "fixture": "TEST-33.1" }),
        error: None,
    }
}

async fn assertion_count(path: &Path, assertion_type: &str) -> i64 {
    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(false);
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await
        .expect("sqlite pool opens");
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM assertion_results WHERE assertion_type = ?",
    )
    .bind(assertion_type)
    .fetch_one(&pool)
    .await
    .expect("assertion count query works");
    pool.close().await;
    count
}

fn current_latest_lock() -> CurrentLatestTargetLock {
    CurrentLatestTargetLock::from_observations(NPM_VIEW, GITHUB_LATEST_RELEASE, LS_REMOTE)
        .expect("current latest lock should parse")
}

fn fixture_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "promptfoo-rs-current-latest-eval-deletion-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("fixture dir should create");
    dir
}

fn temp_path(name: &str, extension: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time is after epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "promptfoo-rs-current-latest-eval-deletion-{name}-{}-{nanos}.{extension}",
        std::process::id()
    ));
    let _ = fs::remove_file(&path);
    path
}

fn write_eval_deletion_source(root: &Path) {
    let path = root.join("src/database/evalDeletion.ts");
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent dir should create");
    }
    fs::write(path, "export const evalDeletion = true;").expect("fixture file should write");
}

fn cache_store_row_for_source<'a>(
    rows: &'a [promptfoo_rs::compatibility::inventory::CurrentLatestInventoryRow],
    source: &str,
) -> &'a promptfoo_rs::compatibility::inventory::CurrentLatestInventoryRow {
    rows.iter()
        .find(|row| row.source_file == source && row.category == "cache-store")
        .unwrap_or_else(|| panic!("missing cache-store row for {source}: {rows:#?}"))
}

fn run_current_latest_source_inventory_script(root: &Path, gate_dir: &Path) {
    let lock_path = gate_dir.join("current-latest-target.json");
    fs::write(
        &lock_path,
        serde_json::to_string_pretty(&current_latest_lock()).expect("lock should serialize"),
    )
    .expect("lock fixture should write");

    let command = format!(
        "CURRENT_LATEST_TARGET_LOCK_FILE='{}' CURRENT_LATEST_SOURCE_ROOT='{}' CURRENT_LATEST_GATE_DIR='{}' bash scripts/release/current-latest-source-inventory.sh",
        shell_escape(&lock_path),
        shell_escape(root),
        shell_escape(gate_dir)
    );
    run_bash(&command);
}

fn run_current_latest_script(gate_dir: &Path, script: &str) {
    let command = format!(
        "CURRENT_LATEST_GATE_DIR='{}' bash {}",
        shell_escape(gate_dir),
        script
    );
    run_bash(&command);
}

fn run_bash(command: &str) {
    let output = Command::new(git_bash())
        .args(["-lc", command])
        .output()
        .expect("bash script should execute");
    assert!(
        output.status.success(),
        "command:\n{command}\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn read_json(path: &Path) -> Value {
    serde_json::from_str(&fs::read_to_string(path).expect("json should be readable"))
        .expect("json should parse")
}

fn shell_escape(path: &Path) -> String {
    path.to_string_lossy()
        .replace('\\', "/")
        .replace('\'', "'\\''")
}

fn git_bash() -> &'static str {
    if cfg!(windows) {
        r"C:\Program Files\Git\bin\bash.exe"
    } else {
        "bash"
    }
}
