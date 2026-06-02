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
        "promptfoo-rs-current-latest-eval-runner-{name}-{}",
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

fn write_eval_runner_source(root: &Path) {
    for relative in fixture_eval_runner_sources()
        .iter()
        .chain(snapshot_eval_runner_sources().iter())
        .chain(rate_limit_eval_runner_sources().iter())
    {
        write_file(root, relative, "export const evalRunner = true;");
    }
}

fn fixture_eval_runner_sources() -> &'static [&'static str] {
    &[
        "src/evaluate.ts",
        "src/evaluator.ts",
        "src/evaluatorHelpers.ts",
        "src/scheduler/index.ts",
        "src/scheduler/providerCallQueue.ts",
        "src/scheduler/retryPolicy.ts",
        "src/scheduler/slotQueue.ts",
        "src/scheduler/types.ts",
    ]
}

fn snapshot_eval_runner_sources() -> &'static [&'static str] {
    &[
        "src/optimizer/promptOptimizer.ts",
        "src/scheduler/events.ts",
        "src/testCase/synthesis.ts",
    ]
}

fn rate_limit_eval_runner_sources() -> &'static [&'static str] {
    &[
        "src/scheduler/adaptiveConcurrency.ts",
        "src/scheduler/headerParser.ts",
        "src/scheduler/providerCallExecutionContext.ts",
        "src/scheduler/providerRateLimitState.ts",
        "src/scheduler/providerWrapper.ts",
        "src/scheduler/rateLimitKey.ts",
        "src/scheduler/rateLimitRegistry.ts",
    ]
}

#[test]
fn test_29_1_1_fixture_covered_eval_runner_rows_have_native_fixture_evidence() {
    /* TEST-29.1.1 */
    let root = fixture_dir("rust-fixture");
    write_eval_runner_source(&root);
    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");

    for source in fixture_eval_runner_sources() {
        let row = eval_runner_row_for_source(&inventory.rows, source);
        assert_eq!(row.level, "P0", "{row:#?}");
        assert_eq!(row.implementation_status, "native", "{row:#?}");
        assert_eq!(row.verification_owner, "eval-runner", "{row:#?}");
        assert_eq!(row.evidence_kind, "fixture", "{row:#?}");
        assert!(
            row.evidence_reference.starts_with("fixture:eval-runner:"),
            "{row:#?}"
        );
        assert!(!row.evidence_reference.starts_with("blocker:"), "{row:#?}");
    }

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_29_1_2_optimizer_event_and_synthesis_rows_are_p1_snapshot_evidence() {
    /* TEST-29.1.2 */
    let root = fixture_dir("rust-snapshot");
    write_eval_runner_source(&root);
    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");

    for source in snapshot_eval_runner_sources() {
        let row = eval_runner_row_for_source(&inventory.rows, source);
        assert_eq!(row.level, "P1", "{row:#?}");
        assert_eq!(row.implementation_status, "later", "{row:#?}");
        assert_eq!(row.verification_owner, "eval-runner", "{row:#?}");
        assert_eq!(row.evidence_kind, "snapshot", "{row:#?}");
        assert!(
            row.evidence_reference.starts_with("snapshot:eval-runner:"),
            "{row:#?}"
        );
    }

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_29_1_3_phase34_scheduler_rows_now_have_native_fixture_evidence() {
    /* TEST-29.1.3 */
    let root = fixture_dir("rust-rate-limit-fixture");
    write_eval_runner_source(&root);
    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");

    for source in rate_limit_eval_runner_sources() {
        let row = eval_runner_row_for_source(&inventory.rows, source);
        assert_eq!(row.level, "P0", "{row:#?}");
        assert_eq!(row.implementation_status, "native", "{row:#?}");
        assert_eq!(row.verification_owner, "eval-runner", "{row:#?}");
        assert_eq!(row.evidence_kind, "fixture", "{row:#?}");
        assert!(
            row.evidence_reference.starts_with("fixture:eval-runner:"),
            "{row:#?}"
        );
    }

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_29_1_4_script_and_rust_extractors_emit_equivalent_eval_runner_evidence() {
    /* TEST-29.1.4 */
    let root = fixture_dir("script-source");
    write_eval_runner_source(&root);
    let gate_dir = fixture_dir("script-gate");
    run_current_latest_source_inventory_script(&root, &gate_dir);

    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");
    let script = read_json(&gate_dir.join("current-latest-source-inventory.json"));
    let script_rows = script["rows"]
        .as_array()
        .expect("script rows should be an array");

    assert_eq!(
        eval_runner_rows_with_json(script_rows, "P0", "native", "fixture").len(),
        15
    );
    assert_eq!(
        eval_runner_rows_with_json(script_rows, "P1", "later", "snapshot").len(),
        3
    );
    assert_eq!(
        eval_runner_rows_with_json(script_rows, "P0", "blocked", "blocker").len(),
        0
    );

    let rust_rows = inventory
        .rows
        .iter()
        .filter(|row| row.category == "eval-runner")
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
        .filter(|row| row["category"] == Value::String("eval-runner".to_string()))
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
fn test_29_1_5_golden_and_quality_have_no_eval_runner_blockers_after_phase34() {
    /* TEST-29.1.5 */
    let root = fixture_dir("quality-source");
    write_eval_runner_source(&root);
    let gate_dir = fixture_dir("quality-gate");
    run_current_latest_source_inventory_script(&root, &gate_dir);
    run_current_latest_script(&gate_dir, "scripts/release/current-latest-golden-corpus.sh");
    run_current_latest_script(&gate_dir, "scripts/release/current-latest-quality-gate.sh");

    let golden = read_json(&gate_dir.join("current-latest-golden-corpus.json"));
    let quality = read_json(&gate_dir.join("current-latest-quality.json"));
    let blockers = golden["release_blockers"]
        .as_array()
        .expect("golden blockers should be an array");
    let eval_runner_blockers = blockers
        .iter()
        .filter(|blocker| {
            blocker["capability"]
                .as_str()
                .unwrap_or_default()
                .starts_with("eval-runner:")
        })
        .collect::<Vec<_>>();

    assert!(eval_runner_blockers.is_empty(), "{eval_runner_blockers:#?}");
    assert_eq!(golden["blocker_count"], Value::from(0));
    assert_eq!(golden["perfect_refactor_claim_allowed"], Value::Bool(false));
    assert_eq!(
        quality["perfect_refactor_claim_allowed"],
        Value::Bool(false)
    );

    let _ = std::fs::remove_dir_all(root);
    let _ = std::fs::remove_dir_all(gate_dir);
}

fn eval_runner_row_for_source<'a>(
    rows: &'a [promptfoo_rs::compatibility::inventory::CurrentLatestInventoryRow],
    source: &str,
) -> &'a promptfoo_rs::compatibility::inventory::CurrentLatestInventoryRow {
    rows.iter()
        .find(|row| row.source_file == source && row.category == "eval-runner")
        .unwrap_or_else(|| panic!("missing eval-runner row for {source}: {rows:#?}"))
}

fn eval_runner_rows_with_json<'a>(
    rows: &'a [Value],
    level: &str,
    implementation_status: &str,
    evidence_kind: &str,
) -> Vec<&'a Value> {
    rows.iter()
        .filter(|row| {
            row["category"] == Value::String("eval-runner".to_string())
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
