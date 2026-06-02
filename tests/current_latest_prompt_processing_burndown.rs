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
96e556507e4bbee5110d94286d500c4605ccc38b\tHEAD
4860e990c7e9a2f8f677173fb92cf9867b34d03f\trefs/tags/0.121.13
1c743afe0e4807882e858c4f322fc064fa5f0770\trefs/tags/code-scan-action-0.1.7
";

fn current_latest_lock() -> CurrentLatestTargetLock {
    CurrentLatestTargetLock::from_observations(NPM_VIEW, GITHUB_LATEST_RELEASE, LS_REMOTE)
        .expect("current latest lock should parse")
}

fn fixture_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "promptfoo-rs-current-latest-prompt-processing-{name}-{}",
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

fn write_prompt_processing_source(root: &Path) {
    for relative in fixture_prompt_processing_sources()
        .iter()
        .chain(snapshot_prompt_processing_sources().iter())
        .chain(blocked_prompt_processing_sources().iter())
    {
        write_file(root, relative, "export const promptProcessing = true;");
    }
}

fn fixture_prompt_processing_sources() -> &'static [&'static str] {
    &[
        "src/prompts/index.ts",
        "src/prompts/processors/jinja.ts",
        "src/prompts/processors/json.ts",
        "src/prompts/processors/markdown.ts",
        "src/prompts/processors/string.ts",
        "src/prompts/processors/text.ts",
        "src/prompts/utils.ts",
    ]
}

fn snapshot_prompt_processing_sources() -> &'static [&'static str] {
    &[
        "src/external/prompts/ragas.ts",
        "src/prompts/constants.ts",
        "src/prompts/grading.ts",
    ]
}

fn blocked_prompt_processing_sources() -> &'static [&'static str] {
    &[
        "src/prompts/processors/executable.ts",
        "src/prompts/processors/javascript.ts",
        "src/prompts/processors/python.ts",
    ]
}

#[test]
fn test_30_1_1_fixture_covered_prompt_processing_rows_have_native_fixture_evidence() {
    /* TEST-30.1.1 */
    let root = fixture_dir("rust-fixture");
    write_prompt_processing_source(&root);
    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");

    for source in fixture_prompt_processing_sources() {
        let row = prompt_processing_row_for_source(&inventory.rows, source);
        assert_eq!(row.level, "P0", "{row:#?}");
        assert_eq!(row.implementation_status, "native", "{row:#?}");
        assert_eq!(row.verification_owner, "config-loader", "{row:#?}");
        assert_eq!(row.evidence_kind, "fixture", "{row:#?}");
        assert!(
            row.evidence_reference
                .starts_with("fixture:prompt-processing:"),
            "{row:#?}"
        );
        assert!(!row.evidence_reference.starts_with("blocker:"), "{row:#?}");
    }

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_30_1_2_static_and_external_prompt_rows_are_p1_snapshot_evidence() {
    /* TEST-30.1.2 */
    let root = fixture_dir("rust-snapshot");
    write_prompt_processing_source(&root);
    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");

    for source in snapshot_prompt_processing_sources() {
        let row = prompt_processing_row_for_source(&inventory.rows, source);
        assert_eq!(row.level, "P1", "{row:#?}");
        assert_eq!(row.implementation_status, "later", "{row:#?}");
        assert_eq!(row.verification_owner, "config-loader", "{row:#?}");
        assert_eq!(row.evidence_kind, "snapshot", "{row:#?}");
        assert!(
            row.evidence_reference
                .starts_with("snapshot:prompt-processing:"),
            "{row:#?}"
        );
    }

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_30_1_3_phase35_script_prompt_processor_rows_have_native_fixture_evidence() {
    /* TEST-30.1.3 */
    let root = fixture_dir("rust-blocked");
    write_prompt_processing_source(&root);
    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");

    for source in blocked_prompt_processing_sources() {
        let row = prompt_processing_row_for_source(&inventory.rows, source);
        assert_eq!(row.level, "P0", "{row:#?}");
        assert_eq!(row.implementation_status, "native", "{row:#?}");
        assert_eq!(row.verification_owner, "script-bridge", "{row:#?}");
        assert_eq!(row.evidence_kind, "fixture", "{row:#?}");
        assert!(
            row.evidence_reference
                .starts_with("fixture:prompt-processing:"),
            "{row:#?}"
        );
        assert!(row
            .blocker_reason
            .as_deref()
            .unwrap_or_default()
            .contains("script bridge"));
    }

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_30_1_4_script_and_rust_extractors_emit_equivalent_prompt_processing_evidence() {
    /* TEST-30.1.4 */
    let root = fixture_dir("script-source");
    write_prompt_processing_source(&root);
    let gate_dir = fixture_dir("script-gate");
    run_current_latest_source_inventory_script(&root, &gate_dir);

    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");
    let script = read_json(&gate_dir.join("current-latest-source-inventory.json"));
    let script_rows = script["rows"]
        .as_array()
        .expect("script rows should be an array");

    assert_eq!(
        prompt_processing_rows_with_json(script_rows, "P0", "native", "fixture").len(),
        10
    );
    assert_eq!(
        prompt_processing_rows_with_json(script_rows, "P1", "later", "snapshot").len(),
        3
    );
    assert_eq!(
        prompt_processing_rows_with_json(script_rows, "P0", "blocked", "blocker").len(),
        0
    );

    let rust_rows = inventory
        .rows
        .iter()
        .filter(|row| row.category == "prompt-processing")
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
        .filter(|row| row["category"] == Value::String("prompt-processing".to_string()))
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
fn test_30_1_5_golden_and_quality_keep_remaining_prompt_processing_blockers_visible() {
    /* TEST-30.1.5 */
    let root = fixture_dir("quality-source");
    write_prompt_processing_source(&root);
    let gate_dir = fixture_dir("quality-gate");
    run_current_latest_source_inventory_script(&root, &gate_dir);
    run_current_latest_script(&gate_dir, "scripts/release/current-latest-golden-corpus.sh");
    run_current_latest_script(&gate_dir, "scripts/release/current-latest-quality-gate.sh");

    let golden = read_json(&gate_dir.join("current-latest-golden-corpus.json"));
    let quality = read_json(&gate_dir.join("current-latest-quality.json"));
    let blockers = golden["release_blockers"]
        .as_array()
        .expect("golden blockers should be an array");
    let prompt_processing_blockers = blockers
        .iter()
        .filter(|blocker| {
            blocker["capability"]
                .as_str()
                .unwrap_or_default()
                .starts_with("prompt-processing:")
        })
        .collect::<Vec<_>>();

    assert_eq!(
        prompt_processing_blockers.len(),
        0,
        "{prompt_processing_blockers:#?}"
    );
    assert_eq!(golden["blocker_count"], Value::from(0));
    assert_eq!(golden["perfect_refactor_claim_allowed"], Value::Bool(false));
    assert_eq!(
        quality["perfect_refactor_claim_allowed"],
        Value::Bool(false)
    );

    let _ = std::fs::remove_dir_all(root);
    let _ = std::fs::remove_dir_all(gate_dir);
}

fn prompt_processing_row_for_source<'a>(
    rows: &'a [promptfoo_rs::compatibility::inventory::CurrentLatestInventoryRow],
    source: &str,
) -> &'a promptfoo_rs::compatibility::inventory::CurrentLatestInventoryRow {
    rows.iter()
        .find(|row| row.source_file == source && row.category == "prompt-processing")
        .unwrap_or_else(|| panic!("missing prompt-processing row for {source}: {rows:#?}"))
}

fn prompt_processing_rows_with_json<'a>(
    rows: &'a [Value],
    level: &str,
    implementation_status: &str,
    evidence_kind: &str,
) -> Vec<&'a Value> {
    rows.iter()
        .filter(|row| {
            row["category"] == Value::String("prompt-processing".to_string())
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
