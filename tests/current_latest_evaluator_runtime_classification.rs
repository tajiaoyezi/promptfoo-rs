use std::path::Path;
use std::process::Command;

use promptfoo_rs::compatibility::diff::DiffClass;
use promptfoo_rs::compatibility::harness::{
    build_current_latest_golden_corpus, evaluate_current_latest_release_blockers,
};
use promptfoo_rs::compatibility::inventory::{
    extract_current_latest_inventory, CurrentLatestTargetLock,
};
use serde_json::Value;

const NPM_VIEW: &str = r#"{
  "version": "0.121.14",
  "gitHead": "7a48c5fce614bee617efbb3b7fc93d404c75b628",
  "dist": {
    "tarball": "https://registry.npmjs.org/promptfoo/-/promptfoo-0.121.14.tgz",
    "integrity": "sha512-YUeBMqwfv3xZC7HJ3ohwk2e0i3DdCitOrvWZPijCOMywp/S+CZEjyqVh1pUzR1PgDo9eBBn9WXyw2wbDBihcpA=="
  },
  "time": {
    "modified": "2026-06-02T13:59:19.451Z"
  }
}"#;

const GITHUB_LATEST_RELEASE: &str = r#"{
  "tag_name": "0.121.14",
  "name": "0.121.14",
  "target_commitish": "7a48c5fce614bee617efbb3b7fc93d404c75b628",
  "published_at": "2026-06-02T13:49:18Z",
  "html_url": "https://github.com/promptfoo/promptfoo/releases/tag/0.121.14"
}"#;

const LS_REMOTE: &str = "\
4d22e57f5f9b4c7cdde494f00558d9afde8b4975\tHEAD
7a48c5fce614bee617efbb3b7fc93d404c75b628\trefs/tags/0.121.14
";

fn current_latest_lock() -> CurrentLatestTargetLock {
    CurrentLatestTargetLock::from_observations(NPM_VIEW, GITHUB_LATEST_RELEASE, LS_REMOTE)
        .expect("current latest lock should parse")
}

fn fixture_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "promptfoo-rs-current-latest-evaluator-runtime-{name}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("fixture dir should create");
    dir
}

fn write_file(root: &Path, relative: &str, contents: &str) {
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("parent dir should create");
    }
    std::fs::write(path, contents).expect("fixture file should write");
}

fn write_evaluator_runtime_source(root: &Path) {
    write_file(
        root,
        "src/evaluator/runtime.ts",
        "export const evaluatorRuntime = true;",
    );
}

#[test]
fn test_39_1_1_rust_classifies_evaluator_runtime_as_eval_runner() {
    /* TEST-39.1.1 */
    let root = fixture_dir("rust-category");
    write_evaluator_runtime_source(&root);

    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");
    let row = inventory
        .rows
        .iter()
        .find(|row| row.source_file == "src/evaluator/runtime.ts")
        .unwrap_or_else(|| panic!("missing evaluator runtime row: {inventory:#?}"));

    assert_eq!(row.stable_id, "eval-runner:src-evaluator-runtime");
    assert_eq!(row.category, "eval-runner");
    assert!(
        !inventory
            .unclassified_rows
            .iter()
            .any(|id| id == "unclassified:src-evaluator-runtime"),
        "{inventory:#?}"
    );

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_39_1_2_evaluator_runtime_remains_p0_blocked_until_fixture_exists() {
    /* TEST-39.1.2 */
    let root = fixture_dir("rust-metadata");
    write_evaluator_runtime_source(&root);

    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");
    let row = inventory
        .rows
        .iter()
        .find(|row| row.source_file == "src/evaluator/runtime.ts")
        .unwrap_or_else(|| panic!("missing evaluator runtime row: {inventory:#?}"));

    assert_eq!(row.level, "P0", "{row:#?}");
    assert_eq!(row.implementation_status, "blocked", "{row:#?}");
    assert_eq!(row.verification_owner, "eval-runner", "{row:#?}");
    assert_eq!(row.evidence_kind, "blocker", "{row:#?}");
    assert_eq!(
        row.evidence_reference,
        "blocker:eval-runner:src-evaluator-runtime"
    );
    assert!(
        row.blocker_reason
            .as_deref()
            .unwrap_or_default()
            .contains("dedicated current-latest eval-runner runtime fixture evidence"),
        "{row:#?}"
    );

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_39_1_3_shell_extractor_matches_rust_evaluator_runtime_classification() {
    /* TEST-39.1.3 */
    let root = fixture_dir("script-source");
    write_evaluator_runtime_source(&root);
    let gate_dir = fixture_dir("script-gate");
    run_current_latest_source_inventory_script(&root, &gate_dir);

    let inventory = read_json(&gate_dir.join("current-latest-source-inventory.json"));
    let matrix = read_json(&gate_dir.join("current-latest-matrix.json"));
    assert_eq!(inventory["unclassified_rows"], Value::Array(vec![]));
    assert_eq!(matrix["unclassified_rows"], Value::Array(vec![]));

    let row = inventory["rows"]
        .as_array()
        .expect("inventory rows should be array")
        .iter()
        .find(|row| row["source_file"] == Value::String("src/evaluator/runtime.ts".to_string()))
        .unwrap_or_else(|| panic!("missing evaluator runtime row: {inventory:#?}"));
    assert_eq!(row["stable_id"], "eval-runner:src-evaluator-runtime");
    assert_eq!(row["category"], "eval-runner");
    assert_eq!(row["implementation_status"], "blocked");
    assert_eq!(row["verification_owner"], "eval-runner");
    assert_eq!(row["evidence_kind"], "blocker");

    let _ = std::fs::remove_dir_all(root);
    let _ = std::fs::remove_dir_all(gate_dir);
}

#[test]
fn test_39_1_4_golden_keeps_evaluator_runtime_blocker_without_unclassified_diff() {
    /* TEST-39.1.4 */
    let root = fixture_dir("golden-source");
    write_evaluator_runtime_source(&root);
    let gate_dir = fixture_dir("golden-gate");
    run_current_latest_source_inventory_script(&root, &gate_dir);

    let report = build_current_latest_golden_corpus(
        &gate_dir.join("current-latest-matrix.json"),
        &gate_dir.join("fixtures"),
        &gate_dir.join("artifacts"),
    )
    .expect("current latest golden corpus should build");
    let blockers = evaluate_current_latest_release_blockers(&report);

    assert!(
        blockers
            .iter()
            .any(|finding| finding.class == DiffClass::Bug),
        "explicit evaluator runtime P0 blocker should remain: {blockers:#?}"
    );
    assert!(
        blockers
            .iter()
            .all(|finding| finding.class != DiffClass::Unclassified),
        "taxonomy cleanup should remove unknown blocker class: {blockers:#?}"
    );
    assert!(!report.perfect_refactor_claim_allowed, "{report:#?}");

    let _ = std::fs::remove_dir_all(root);
    let _ = std::fs::remove_dir_all(gate_dir);
}

fn run_current_latest_source_inventory_script(root: &Path, gate_dir: &Path) {
    let lock_path = gate_dir.join("current-latest-target.json");
    std::fs::write(
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
    serde_json::from_str(&std::fs::read_to_string(path).expect("json should be readable"))
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
