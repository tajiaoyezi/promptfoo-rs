use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

use promptfoo_rs::compatibility::inventory::{
    extract_current_latest_inventory, reconcile_current_latest_matrix,
    write_current_latest_inventory_artifacts, CurrentLatestTargetLock,
};
use promptfoo_rs::compatibility::matrix::{CapabilityMatrix, CapabilityRow};
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
        "promptfoo-rs-current-latest-inventory-{name}-{}",
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

fn write_representative_source(root: &Path) {
    write_file(
        root,
        "src/main.ts",
        "program.option('--config <path>').option('--verbose');",
    );
    write_file(
        root,
        "src/commands/eval.ts",
        "export const flags = ['--max-concurrency', '--output'];",
    );
    write_file(
        root,
        "src/providers/openai/chat.ts",
        "export class OpenAiChat {}",
    );
    write_file(
        root,
        "src/assertions/equals.ts",
        "export const equals = true;",
    );
    write_file(root, "src/redteam/plugins/harmful.ts", "export default {};");
    write_file(
        root,
        "src/redteam/strategies/jailbreak.ts",
        "export default {};",
    );
    write_file(
        root,
        "src/util/output.ts",
        "export function writeJsonl() {}",
    );
    write_file(root, "src/config/default.ts", "export const config = {};");
    write_file(
        root,
        "src/app/page.tsx",
        "export default function Page() { return null; }",
    );
    write_file(root, "src/index.ts", "export function evaluate() {}");
    write_file(
        root,
        "examples/basic/promptfooconfig.yaml",
        "prompts: ['hello']",
    );
    write_file(root, "docs/providers/openai.md", "# OpenAI provider");
}

fn explicit_matrix() -> CapabilityMatrix {
    CapabilityMatrix {
        rows: vec![CapabilityRow {
            capability: "provider:src-providers-openai-chat".to_string(),
            level: "P0".to_string(),
            target_status: "native".to_string(),
            verification: "fixture:provider:src-providers-openai-chat".to_string(),
            owner: "provider-runtime".to_string(),
            notes: "reason: explicit current-latest provider fixture row".to_string(),
        }],
    }
}

#[test]
fn test_24_2_1_inventory_uses_locked_current_latest_head_not_frozen_source() {
    /* TEST-24.2.1 */
    let lock = current_latest_lock();
    let root = fixture_dir("locked-head");
    write_representative_source(&root);

    let report = extract_current_latest_inventory(&lock, &root)
        .expect("current latest source inventory should extract");

    assert_eq!(
        report.schema,
        "promptfoo-rs.current-latest-source-inventory.v1"
    );
    assert_eq!(
        report.target.github.default_branch_head,
        "1d09dfeb5f0766905409117f923dd5c4b0838d9f"
    );
    assert_eq!(report.extraction_mode, "current-latest-locked-source-tree");
    assert!(report.rows.iter().all(|row| row
        .source_reference
        .contains(&lock.github.default_branch_head)));
    assert!(report
        .rows
        .iter()
        .all(|row| !row.source_reference.contains("promptfoo@0.121.13:")));

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_24_2_2_rows_cover_all_current_latest_categories_with_stable_refs() {
    /* TEST-24.2.2 */
    let root = fixture_dir("category-coverage");
    write_representative_source(&root);
    let report = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest source inventory should extract");

    let categories = report
        .rows
        .iter()
        .map(|row| row.category.as_str())
        .collect::<BTreeSet<_>>();
    for expected in [
        "command",
        "flag",
        "provider",
        "assertion",
        "redteam-plugin",
        "redteam-strategy",
        "output",
        "config",
        "viewer",
        "node-api",
        "example",
        "docs",
    ] {
        assert!(
            categories.contains(expected),
            "missing category {expected}: {categories:?}"
        );
    }
    assert!(report
        .rows
        .iter()
        .any(|row| row.stable_id == "flag:max-concurrency"));
    assert!(report.rows.iter().all(|row| {
        row.stable_id == format!("{}:{}", row.category, row.name)
            && row.source_reference.contains(&row.source_file)
            && !row.source_reference.trim().is_empty()
    }));

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_24_2_3_matrix_reconciliation_adds_level_status_owner_and_evidence() {
    /* TEST-24.2.3 */
    let root = fixture_dir("matrix-reconciliation");
    write_representative_source(&root);
    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest source inventory should extract");
    let matrix = reconcile_current_latest_matrix(&inventory, &explicit_matrix());

    assert_eq!(matrix.schema, "promptfoo-rs.current-latest-matrix.v1");
    assert_eq!(matrix.rows.len(), inventory.rows.len());
    assert!(matrix.unclassified_rows.is_empty(), "{matrix:#?}");
    assert!(matrix.rows_missing_evidence.is_empty(), "{matrix:#?}");
    assert!(matrix.rows.iter().all(|row| {
        matches!(row.level.as_str(), "P0" | "P1" | "P2")
            && !row.implementation_status.trim().is_empty()
            && !row.verification_owner.trim().is_empty()
            && !row.evidence_kind.trim().is_empty()
            && !row.evidence_reference.trim().is_empty()
    }));
    let provider = matrix
        .rows
        .iter()
        .find(|row| row.item_id == "provider:src-providers-openai-chat")
        .expect("provider row should exist");
    assert_eq!(provider.level, "P0");
    assert_eq!(provider.implementation_status, "native");
    assert_eq!(provider.evidence_kind, "fixture");

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_24_2_4_unclassified_or_missing_evidence_blocks_perfect_claim_and_scripts_write_artifacts() {
    /* TEST-24.2.4 */
    let root = fixture_dir("unclassified");
    write_representative_source(&root);
    write_file(
        root.as_path(),
        "src/experimental/newSurface.ts",
        "export const x = 1;",
    );

    let lock = current_latest_lock();
    let inventory = extract_current_latest_inventory(&lock, &root)
        .expect("current latest source inventory should extract");
    let matrix = reconcile_current_latest_matrix(&inventory, &CapabilityMatrix { rows: vec![] });

    assert!(!matrix.perfect_refactor_claim_allowed, "{matrix:#?}");
    assert!(
        matrix
            .unclassified_rows
            .iter()
            .any(|row| row.starts_with("unclassified:")),
        "{matrix:#?}"
    );

    let gate_dir = fixture_dir("script-gate");
    let inventory_path = gate_dir.join("current-latest-source-inventory.json");
    let matrix_path = gate_dir.join("current-latest-matrix.json");
    write_current_latest_inventory_artifacts(
        &inventory,
        &matrix,
        Path::new(&inventory_path),
        Path::new(&matrix_path),
    )
    .expect("current latest artifacts should write");
    let written: Value = serde_json::from_str(
        &std::fs::read_to_string(&inventory_path).expect("inventory json should be readable"),
    )
    .expect("inventory json should parse");
    assert_eq!(
        written["perfect_refactor_claim_allowed"],
        Value::Bool(false)
    );

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
    assert!(gate_dir
        .join("current-latest-source-inventory.json")
        .exists());
    assert!(gate_dir.join("current-latest-matrix.json").exists());

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
