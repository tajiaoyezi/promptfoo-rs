use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

use promptfoo_rs::compatibility::inventory::{
    extract_current_latest_inventory, CurrentLatestTargetLock,
};
use serde_json::Value;

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

fn current_latest_lock() -> CurrentLatestTargetLock {
    CurrentLatestTargetLock::from_observations(NPM_VIEW, GITHUB_LATEST_RELEASE, LS_REMOTE)
        .expect("current latest lock should parse")
}

fn fixture_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "promptfoo-rs-current-latest-cache-store-{name}-{}",
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

fn write_cache_store_source(root: &Path) {
    for relative in fixture_cache_store_sources()
        .iter()
        .chain(snapshot_cache_store_sources().iter())
        .chain(blocked_cache_store_sources().iter())
    {
        write_file(root, relative, "export const cacheStore = true;");
    }
}

fn fixture_cache_store_sources() -> &'static [&'static str] {
    &[
        "src/cache.ts",
        "src/database/evalDeletion.ts",
        "src/database/index.ts",
        "src/database/tables.ts",
        "src/storage/index.ts",
        "src/storage/localFileSystemProvider.ts",
        "src/storage/types.ts",
    ]
}

fn snapshot_cache_store_sources() -> &'static [&'static str] {
    &["src/database/signal.ts", "src/database/testing.ts"]
}

fn blocked_cache_store_sources() -> &'static [&'static str] {
    &[]
}

#[test]
fn test_31_1_1_fixture_covered_cache_store_rows_have_native_fixture_evidence() {
    /* TEST-31.1.1 */
    let root = fixture_dir("rust-fixture");
    write_cache_store_source(&root);
    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");

    for source in fixture_cache_store_sources() {
        let row = cache_store_row_for_source(&inventory.rows, source);
        assert_eq!(row.level, "P0", "{row:#?}");
        assert_eq!(row.implementation_status, "native", "{row:#?}");
        assert_eq!(row.verification_owner, "cache-resume-store", "{row:#?}");
        assert_eq!(row.evidence_kind, "fixture", "{row:#?}");
        assert!(
            row.evidence_reference.starts_with("fixture:cache-store:"),
            "{row:#?}"
        );
        assert!(!row.evidence_reference.starts_with("blocker:"), "{row:#?}");
    }

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_31_1_2_helper_cache_store_rows_are_p1_snapshot_evidence() {
    /* TEST-31.1.2 */
    let root = fixture_dir("rust-snapshot");
    write_cache_store_source(&root);
    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");

    for source in snapshot_cache_store_sources() {
        let row = cache_store_row_for_source(&inventory.rows, source);
        assert_eq!(row.level, "P1", "{row:#?}");
        assert_eq!(row.implementation_status, "later", "{row:#?}");
        assert_eq!(row.verification_owner, "cache-resume-store", "{row:#?}");
        assert_eq!(row.evidence_kind, "snapshot", "{row:#?}");
        assert!(
            row.evidence_reference.starts_with("snapshot:cache-store:"),
            "{row:#?}"
        );
    }

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_31_1_3_eval_deletion_is_covered_by_dedicated_lifecycle_fixture() {
    /* TEST-31.1.3 */
    let root = fixture_dir("rust-eval-deletion-fixture");
    write_cache_store_source(&root);
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

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_31_1_4_script_and_rust_extractors_emit_equivalent_cache_store_evidence() {
    /* TEST-31.1.4 */
    let root = fixture_dir("script-source");
    write_cache_store_source(&root);
    let gate_dir = fixture_dir("script-gate");
    run_current_latest_source_inventory_script(&root, &gate_dir);

    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");
    let script = read_json(&gate_dir.join("current-latest-source-inventory.json"));
    let script_rows = script["rows"]
        .as_array()
        .expect("script rows should be an array");

    assert_eq!(
        cache_store_rows_with_json(script_rows, "P0", "native", "fixture").len(),
        7
    );
    assert_eq!(
        cache_store_rows_with_json(script_rows, "P1", "later", "snapshot").len(),
        2
    );
    assert_eq!(
        cache_store_rows_with_json(script_rows, "P0", "blocked", "blocker").len(),
        0
    );

    let rust_rows = inventory
        .rows
        .iter()
        .filter(|row| row.category == "cache-store")
        .map(|row| {
            (
                row.stable_id.clone(),
                (
                    row.level.clone(),
                    row.implementation_status.clone(),
                    row.verification_owner.clone(),
                    row.evidence_kind.clone(),
                    row.evidence_reference.clone(),
                ),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let shell_rows = script_rows
        .iter()
        .filter(|row| row["category"] == Value::String("cache-store".to_string()))
        .map(|row| {
            (
                row["stable_id"].as_str().unwrap_or_default().to_string(),
                (
                    row["level"].as_str().unwrap_or_default().to_string(),
                    row["implementation_status"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    row["verification_owner"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    row["evidence_kind"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    row["evidence_reference"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                ),
            )
        })
        .collect::<BTreeMap<_, _>>();

    assert_eq!(rust_rows, shell_rows);

    let _ = std::fs::remove_dir_all(root);
    let _ = std::fs::remove_dir_all(gate_dir);
}

#[test]
fn test_31_1_5_golden_and_quality_clear_remaining_cache_store_blocker() {
    /* TEST-31.1.5 */
    let root = fixture_dir("quality-source");
    write_cache_store_source(&root);
    let gate_dir = fixture_dir("quality-gate");
    run_current_latest_source_inventory_script(&root, &gate_dir);
    run_current_latest_script(&gate_dir, "scripts/release/current-latest-golden-corpus.sh");
    run_current_latest_script(&gate_dir, "scripts/release/current-latest-quality-gate.sh");

    let golden = read_json(&gate_dir.join("current-latest-golden-corpus.json"));
    let quality = read_json(&gate_dir.join("current-latest-quality.json"));
    let blockers = golden["release_blockers"]
        .as_array()
        .expect("golden blockers should be an array");
    let cache_store_blockers = blockers
        .iter()
        .filter(|blocker| {
            blocker["capability"]
                .as_str()
                .unwrap_or_default()
                .starts_with("cache-store:")
        })
        .collect::<Vec<_>>();

    assert!(cache_store_blockers.is_empty(), "{cache_store_blockers:#?}");
    assert_eq!(golden["blocker_count"], Value::from(0));
    assert_eq!(
        quality["perfect_refactor_claim_allowed"],
        Value::Bool(false)
    );

    let _ = std::fs::remove_dir_all(root);
    let _ = std::fs::remove_dir_all(gate_dir);
}

fn cache_store_row_for_source<'a>(
    rows: &'a [promptfoo_rs::compatibility::inventory::CurrentLatestInventoryRow],
    source: &str,
) -> &'a promptfoo_rs::compatibility::inventory::CurrentLatestInventoryRow {
    rows.iter()
        .find(|row| row.source_file == source && row.category == "cache-store")
        .unwrap_or_else(|| panic!("missing cache-store row for {source}: {rows:#?}"))
}

fn cache_store_rows_with_json<'a>(
    rows: &'a [Value],
    level: &str,
    implementation_status: &str,
    evidence_kind: &str,
) -> Vec<&'a Value> {
    rows.iter()
        .filter(|row| {
            row["category"] == Value::String("cache-store".to_string())
                && row["level"] == Value::String(level.to_string())
                && row["implementation_status"] == Value::String(implementation_status.to_string())
                && row["evidence_kind"] == Value::String(evidence_kind.to_string())
        })
        .collect()
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
