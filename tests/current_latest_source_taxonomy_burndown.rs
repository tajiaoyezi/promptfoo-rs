use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

use promptfoo_rs::compatibility::diff::DiffClass;
use promptfoo_rs::compatibility::harness::{
    build_current_latest_golden_corpus, evaluate_current_latest_release_blockers,
};
use promptfoo_rs::compatibility::inventory::{
    extract_current_latest_inventory, reconcile_current_latest_matrix,
    write_current_latest_inventory_artifacts, CurrentLatestTargetLock,
};
use promptfoo_rs::compatibility::matrix::CapabilityMatrix;
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
        "promptfoo-rs-current-latest-taxonomy-{name}-{}",
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

fn write_taxonomy_source(root: &Path) {
    for relative in [
        "src/evaluate.ts",
        "src/evaluator.ts",
        "src/cache.ts",
        "src/database/index.ts",
        "src/storage/index.ts",
        "src/scheduler/index.ts",
        "src/scheduler/providerWrapper.ts",
        "src/prompts/processors/python.ts",
        "src/prompts/index.ts",
        "src/matchers/llmGrading.ts",
        "src/external/matchers/deepeval.ts",
        "src/redteam/providers/bestOfN.ts",
        "src/redteam/extraction/purpose.ts",
        "src/redteam/types.ts",
        "src/types/api.ts",
        "src/contracts/prompts.ts",
        "src/models/eval.ts",
        "src/validators/prompts.ts",
        "src/server/routes/eval.ts",
        "src/openapi/server.ts",
        "src/blobs/filesystemProvider.ts",
        "src/tracing/evaluatorTracing.ts",
        "src/python/wrapper.ts",
        "src/ruby/wrapper.ts",
        "src/importers/openaiEvals/convert.ts",
        "src/integrations/langfuse.ts",
        "src/share.ts",
        "src/feedback.ts",
        "src/telemetry.ts",
        "src/updates.ts",
        "src/util/fetch/index.ts",
        "src/logger.ts",
    ] {
        write_file(root, relative, "export const marker = true;");
    }
}

#[test]
fn test_25_1_1_representative_previous_unknown_source_families_are_classified() {
    /* TEST-25.1.1 */
    let root = fixture_dir("classified-families");
    write_taxonomy_source(&root);

    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");
    let categories = inventory
        .rows
        .iter()
        .map(|row| row.category.as_str())
        .collect::<BTreeSet<_>>();

    assert!(
        inventory.unclassified_rows.is_empty(),
        "unexpected unclassified rows: {:?}",
        inventory.unclassified_rows
    );
    for expected in [
        "eval-runner",
        "cache-store",
        "prompt-processing",
        "assertion-support",
        "redteam-support",
        "schema",
        "viewer",
        "script-bridge",
        "integration",
        "cloud-share",
        "observability",
        "runtime-support",
    ] {
        assert!(
            categories.contains(expected),
            "missing category {expected}: {categories:?}"
        );
    }

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_25_1_2_taxonomy_preserves_p0_p1_p2_evidence_semantics() {
    /* TEST-25.1.2 */
    let root = fixture_dir("semantic-levels");
    write_taxonomy_source(&root);

    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");

    let eval_row = inventory
        .rows
        .iter()
        .find(|row| row.source_file == "src/scheduler/providerWrapper.ts")
        .expect("unproven eval runtime row should exist");
    assert_eq!(eval_row.category, "eval-runner");
    assert_eq!(eval_row.level, "P0");
    assert_eq!(eval_row.evidence_kind, "blocker");
    assert!(eval_row
        .blocker_reason
        .as_deref()
        .unwrap_or_default()
        .contains("dedicated current-latest eval-runner evidence"));

    let schema_row = inventory
        .rows
        .iter()
        .find(|row| row.source_file == "src/types/api.ts")
        .expect("schema row should exist");
    assert_eq!(schema_row.category, "schema");
    assert_eq!(schema_row.level, "P1");
    assert_eq!(schema_row.evidence_kind, "snapshot");

    let observability_row = inventory
        .rows
        .iter()
        .find(|row| row.source_file == "src/tracing/evaluatorTracing.ts")
        .expect("observability row should exist");
    assert_eq!(observability_row.category, "observability");
    assert_eq!(observability_row.level, "P1");
    assert_eq!(observability_row.evidence_kind, "snapshot");

    let cloud_row = inventory
        .rows
        .iter()
        .find(|row| row.source_file == "src/share.ts")
        .expect("cloud/share row should exist");
    assert_eq!(cloud_row.category, "cloud-share");
    assert_eq!(cloud_row.level, "P2");
    assert_eq!(cloud_row.evidence_kind, "registration");
    assert!(cloud_row
        .blocker_reason
        .as_deref()
        .unwrap_or_default()
        .contains("unsupported"));

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_25_1_3_script_writes_zero_unclassified_inventory_and_matrix_artifacts() {
    /* TEST-25.1.3 */
    let root = fixture_dir("script-source");
    write_taxonomy_source(&root);
    let gate_dir = fixture_dir("script-gate");
    let lock = current_latest_lock();
    let lock_path = gate_dir.join("current-latest-target.json");
    std::fs::write(
        &lock_path,
        serde_json::to_string_pretty(&lock).expect("lock should serialize"),
    )
    .expect("lock fixture should write");

    let command = format!(
        "CURRENT_LATEST_TARGET_LOCK_FILE='{}' CURRENT_LATEST_SOURCE_ROOT='{}' CURRENT_LATEST_GATE_DIR='{}' bash scripts/release/current-latest-source-inventory.sh",
        shell_escape(&lock_path),
        shell_escape(&root),
        shell_escape(&gate_dir)
    );
    let script_output = Command::new(git_bash())
        .args(["-lc", &command])
        .output()
        .expect("current latest inventory script should execute");
    assert!(
        script_output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&script_output.stdout),
        String::from_utf8_lossy(&script_output.stderr)
    );

    let inventory: Value = serde_json::from_str(
        &std::fs::read_to_string(gate_dir.join("current-latest-source-inventory.json"))
            .expect("inventory artifact should be readable"),
    )
    .expect("inventory artifact should parse");
    let matrix: Value = serde_json::from_str(
        &std::fs::read_to_string(gate_dir.join("current-latest-matrix.json"))
            .expect("matrix artifact should be readable"),
    )
    .expect("matrix artifact should parse");
    assert_eq!(inventory["unclassified_rows"], Value::Array(vec![]));
    assert_eq!(matrix["unclassified_rows"], Value::Array(vec![]));
    assert_eq!(inventory["rows_missing_evidence"], Value::Array(vec![]));
    assert_eq!(matrix["rows_missing_evidence"], Value::Array(vec![]));

    let _ = std::fs::remove_dir_all(root);
    let _ = std::fs::remove_dir_all(gate_dir);
}

#[test]
fn test_25_1_4_golden_corpus_keeps_real_blockers_without_taxonomy_unclassified() {
    /* TEST-25.1.4 */
    let root = fixture_dir("golden-source");
    write_taxonomy_source(&root);
    let gate_dir = fixture_dir("golden-gate");
    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");
    let matrix = reconcile_current_latest_matrix(&inventory, &CapabilityMatrix { rows: vec![] });
    write_current_latest_inventory_artifacts(
        &inventory,
        &matrix,
        &gate_dir.join("current-latest-source-inventory.json"),
        &gate_dir.join("current-latest-matrix.json"),
    )
    .expect("inventory artifacts should write");

    let report = build_current_latest_golden_corpus(
        &gate_dir.join("current-latest-matrix.json"),
        &gate_dir.join("fixtures"),
        &gate_dir.join("artifacts"),
    )
    .expect("current latest golden corpus should build");
    let blockers = evaluate_current_latest_release_blockers(&report);

    assert!(!report.perfect_refactor_claim_allowed, "{report:#?}");
    assert!(
        blockers
            .iter()
            .any(|finding| finding.class == DiffClass::Bug),
        "expected explicit P0 blockers to remain: {blockers:#?}"
    );
    assert!(
        blockers
            .iter()
            .all(|finding| finding.class != DiffClass::Unclassified),
        "taxonomy cleanup should remove only unknown blockers: {blockers:#?}"
    );

    let _ = std::fs::remove_dir_all(root);
    let _ = std::fs::remove_dir_all(gate_dir);
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
