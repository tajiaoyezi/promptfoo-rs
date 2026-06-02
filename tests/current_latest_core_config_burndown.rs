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
        "promptfoo-rs-current-latest-core-config-{name}-{}",
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

fn write_config_source(root: &Path) {
    for relative in runtime_config_sources() {
        write_file(root, relative, "export const runtimeConfig = true;");
    }
    for relative in auxiliary_config_sources() {
        write_file(root, relative, "export const auxiliaryConfig = true;");
    }
    for relative in external_config_sources() {
        write_file(root, relative, "export const externalConfig = true;");
    }
    write_file(
        root,
        "src/redteam/plugins/policy/evals/promptfooconfig.yaml",
        "prompts:\n  - redteam config fixture\n",
    );
}

fn runtime_config_sources() -> &'static [&'static str] {
    &[
        "src/commands/config.ts",
        "src/configTypes.ts",
        "src/util/config/default.ts",
        "src/util/config/extensions.ts",
        "src/util/config/load.ts",
        "src/util/config/manage.ts",
        "src/util/config/writer.ts",
    ]
}

fn auxiliary_config_sources() -> &'static [&'static str] {
    &[
        "src/codeScan/config/loader.ts",
        "src/codeScan/config/schema.ts",
        "src/commands/mcp/tools/validatePromptfooConfig.ts",
    ]
}

fn external_config_sources() -> &'static [&'static str] {
    &[
        "src/globalConfig/accounts.ts",
        "src/globalConfig/cloud.ts",
        "src/globalConfig/globalConfig.ts",
        "src/server/config/serverConfig.ts",
        "src/server/routes/configs.ts",
        "src/tracing/otelConfig.ts",
        "src/types/api/configs.ts",
    ]
}

fn redteam_config_source() -> &'static str {
    "src/redteam/plugins/policy/evals/promptfooconfig.yaml"
}

#[test]
fn test_27_1_1_runtime_and_redteam_config_rows_have_fixture_evidence() {
    /* TEST-27.1.1 */
    let root = fixture_dir("runtime");
    write_config_source(&root);
    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");

    for source in runtime_config_sources() {
        let row = config_row_for_source(&inventory.rows, source);
        assert_eq!(row.level, "P0", "{row:#?}");
        assert_eq!(row.implementation_status, "native", "{row:#?}");
        assert_eq!(row.verification_owner, "config-loader", "{row:#?}");
        assert_eq!(row.evidence_kind, "fixture", "{row:#?}");
        assert!(
            row.evidence_reference.starts_with("fixture:config:"),
            "{row:#?}"
        );
        assert!(!row.evidence_reference.starts_with("blocker:"), "{row:#?}");
    }

    let redteam = config_row_for_source(&inventory.rows, redteam_config_source());
    assert_eq!(redteam.level, "P0", "{redteam:#?}");
    assert_eq!(redteam.implementation_status, "native", "{redteam:#?}");
    assert_eq!(redteam.verification_owner, "redteam-engine", "{redteam:#?}");
    assert_eq!(redteam.evidence_kind, "fixture", "{redteam:#?}");
    assert!(
        redteam.evidence_reference.starts_with("fixture:config:"),
        "{redteam:#?}"
    );

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_27_1_2_auxiliary_config_rows_are_p1_snapshot_evidence() {
    /* TEST-27.1.2 */
    let root = fixture_dir("auxiliary");
    write_config_source(&root);
    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");

    for source in auxiliary_config_sources() {
        let row = config_row_for_source(&inventory.rows, source);
        assert_eq!(row.level, "P1", "{row:#?}");
        assert_eq!(row.implementation_status, "later", "{row:#?}");
        assert!(matches!(
            row.verification_owner.as_str(),
            "scan-engine" | "mcp-runtime"
        ));
        assert_eq!(row.evidence_kind, "snapshot", "{row:#?}");
        assert!(
            row.evidence_reference.starts_with("snapshot:config:"),
            "{row:#?}"
        );
        assert!(!row.evidence_reference.starts_with("blocker:"), "{row:#?}");
    }

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_27_1_3_external_config_rows_remain_explicit_blockers() {
    /* TEST-27.1.3 */
    let root = fixture_dir("external");
    write_config_source(&root);
    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");

    for source in external_config_sources() {
        let row = config_row_for_source(&inventory.rows, source);
        assert_eq!(row.level, "P0", "{row:#?}");
        assert_eq!(row.implementation_status, "blocked", "{row:#?}");
        assert_eq!(row.verification_owner, "external-authority", "{row:#?}");
        assert_eq!(row.evidence_kind, "blocker", "{row:#?}");
        assert!(
            row.evidence_reference.starts_with("blocker:config:"),
            "{row:#?}"
        );
        assert!(row
            .blocker_reason
            .as_deref()
            .unwrap_or_default()
            .contains("external"));
    }

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_27_1_4_script_artifacts_reduce_config_blockers_to_external_only() {
    /* TEST-27.1.4 */
    let root = fixture_dir("script-source");
    write_config_source(&root);
    let gate_dir = fixture_dir("script-gate");
    run_current_latest_source_inventory_script(&root, &gate_dir);
    run_current_latest_script(&gate_dir, "scripts/release/current-latest-golden-corpus.sh");
    run_current_latest_script(&gate_dir, "scripts/release/current-latest-quality-gate.sh");

    let inventory = read_json(&gate_dir.join("current-latest-source-inventory.json"));
    let golden = read_json(&gate_dir.join("current-latest-golden-corpus.json"));
    let quality = read_json(&gate_dir.join("current-latest-quality.json"));
    let rows = inventory["rows"]
        .as_array()
        .expect("inventory rows should be an array");

    assert_eq!(config_rows_with(rows, "P0", "native", "fixture").len(), 8);
    assert_eq!(config_rows_with(rows, "P1", "later", "snapshot").len(), 3);
    assert_eq!(config_rows_with(rows, "P0", "blocked", "blocker").len(), 7);

    let config_blockers = golden["release_blockers"]
        .as_array()
        .expect("release blockers should be an array")
        .iter()
        .filter(|blocker| {
            blocker["capability"]
                .as_str()
                .unwrap_or_default()
                .starts_with("config:")
        })
        .collect::<Vec<_>>();
    assert_eq!(
        config_blockers.len(),
        7,
        "only external config rows should remain blockers: {config_blockers:#?}"
    );
    for blocker in config_blockers {
        let capability = blocker["capability"].as_str().unwrap_or_default();
        assert!(
            external_config_sources()
                .iter()
                .any(|source| capability == stable_config_id(source)),
            "unexpected config blocker {capability}"
        );
    }
    assert_eq!(golden["perfect_refactor_claim_allowed"], Value::Bool(false));
    assert_eq!(quality["perfect_refactor_claim_allowed"], Value::Bool(false));

    let _ = std::fs::remove_dir_all(root);
    let _ = std::fs::remove_dir_all(gate_dir);
}

fn config_row_for_source<'a>(
    rows: &'a [promptfoo_rs::compatibility::inventory::CurrentLatestInventoryRow],
    source: &str,
) -> &'a promptfoo_rs::compatibility::inventory::CurrentLatestInventoryRow {
    rows.iter()
        .find(|row| row.source_file == source && row.category == "config")
        .unwrap_or_else(|| panic!("missing config row for {source}: {rows:#?}"))
}

fn config_rows_with<'a>(
    rows: &'a [Value],
    level: &str,
    implementation_status: &str,
    evidence_kind: &str,
) -> Vec<&'a Value> {
    rows.iter()
        .filter(|row| {
            row["category"] == Value::String("config".to_string())
                && row["level"] == Value::String(level.to_string())
                && row["implementation_status"] == Value::String(implementation_status.to_string())
                && row["evidence_kind"] == Value::String(evidence_kind.to_string())
        })
        .collect()
}

fn stable_config_id(source: &str) -> String {
    let without_extension = source.rsplit_once('.').map_or(source, |(left, _)| left);
    format!("config:{}", slug(without_extension))
}

fn slug(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
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
