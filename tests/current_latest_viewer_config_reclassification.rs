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
        "promptfoo-rs-current-latest-viewer-config-{name}-{}",
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

fn write_viewer_and_config_source(root: &Path) {
    for relative in app_viewer_config_sources() {
        write_file(root, relative, "export const viewerConfig = true;");
    }
    for relative in non_app_config_sources() {
        write_file(root, relative, "export const runtimeConfig = true;");
    }
}

fn app_viewer_config_sources() -> &'static [&'static str] {
    &[
        "src/app/vite.config.ts",
        "src/app/src/pages/eval/components/ConfigModal.tsx",
        "src/app/src/stores/evalConfig.ts",
    ]
}

fn non_app_config_sources() -> &'static [&'static str] {
    &[
        "src/util/config/load.ts",
        "src/commands/config.ts",
        "src/configTypes.ts",
        "src/globalConfig/cloud.ts",
        "src/server/config/serverConfig.ts",
    ]
}

#[test]
fn test_26_1_1_app_config_rows_remain_viewer_evidence_without_config_blockers() {
    /* TEST-26.1.1 */
    let root = fixture_dir("rust-viewer");
    write_viewer_and_config_source(&root);

    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");

    for source in app_viewer_config_sources() {
        let rows = inventory
            .rows
            .iter()
            .filter(|row| row.source_file == *source)
            .collect::<Vec<_>>();
        assert!(
            !rows.is_empty(),
            "missing rows for {source}: {inventory:#?}"
        );
        assert!(
            rows.iter().all(|row| row.category != "config"),
            "src/app viewer config source must not create duplicate P0 config row: {rows:#?}"
        );
        let viewer = rows
            .iter()
            .find(|row| row.category == "viewer")
            .expect("src/app config source should remain tracked as viewer evidence");
        assert_eq!(viewer.level, "P1", "{viewer:#?}");
        assert_eq!(viewer.implementation_status, "later", "{viewer:#?}");
        assert_eq!(viewer.verification_owner, "web-viewer", "{viewer:#?}");
        assert_eq!(viewer.evidence_kind, "snapshot", "{viewer:#?}");
        assert!(!viewer.evidence_reference.trim().is_empty(), "{viewer:#?}");
    }

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_26_1_2_non_app_config_rows_remain_p0_config_evidence_or_blockers() {
    /* TEST-26.1.2 */
    let root = fixture_dir("rust-core-config");
    write_viewer_and_config_source(&root);

    let inventory = extract_current_latest_inventory(&current_latest_lock(), &root)
        .expect("current latest inventory should extract");

    for source in non_app_config_sources() {
        let row = inventory
            .rows
            .iter()
            .find(|row| row.source_file == *source && row.category == "config")
            .unwrap_or_else(|| panic!("missing non-app config row for {source}: {inventory:#?}"));
        assert_eq!(row.level, "P0", "{row:#?}");
        assert!(
            matches!(row.implementation_status.as_str(), "native" | "blocked"),
            "{row:#?}"
        );
        assert!(
            matches!(
                row.verification_owner.as_str(),
                "config-loader" | "external-authority"
            ),
            "{row:#?}"
        );
        assert!(
            matches!(row.evidence_kind.as_str(), "fixture" | "blocker"),
            "{row:#?}"
        );
        assert!(
            row.evidence_reference.starts_with("fixture:config:")
                || row.evidence_reference.starts_with("blocker:config:"),
            "{row:#?}"
        );
    }

    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn test_26_1_3_script_artifacts_remove_app_config_blockers_without_dropping_rows() {
    /* TEST-26.1.3 */
    let root = fixture_dir("script-source");
    write_viewer_and_config_source(&root);
    let gate_dir = fixture_dir("script-gate");
    run_current_latest_source_inventory_script(&root, &gate_dir);

    let inventory = read_json(&gate_dir.join("current-latest-source-inventory.json"));
    let matrix = read_json(&gate_dir.join("current-latest-matrix.json"));

    assert_json_has_viewer_rows_without_config_rows(&inventory, app_viewer_config_sources());
    assert_json_has_viewer_rows_without_config_rows(&matrix, app_viewer_config_sources());
    assert_json_has_non_app_config_rows(&inventory, non_app_config_sources());
    assert_json_has_non_app_config_rows(&matrix, non_app_config_sources());
    assert_eq!(inventory["unclassified_rows"], Value::Array(vec![]));
    assert_eq!(matrix["unclassified_rows"], Value::Array(vec![]));
    assert_eq!(inventory["rows_missing_evidence"], Value::Array(vec![]));
    assert_eq!(matrix["rows_missing_evidence"], Value::Array(vec![]));

    let _ = std::fs::remove_dir_all(root);
    let _ = std::fs::remove_dir_all(gate_dir);
}

#[test]
fn test_26_1_4_golden_and_quality_keep_real_blockers_visible() {
    /* TEST-26.1.4 */
    let root = fixture_dir("quality-source");
    write_viewer_and_config_source(&root);
    let gate_dir = fixture_dir("quality-gate");
    run_current_latest_source_inventory_script(&root, &gate_dir);
    run_current_latest_script(&gate_dir, "scripts/release/current-latest-golden-corpus.sh");
    run_current_latest_script(&gate_dir, "scripts/release/current-latest-quality-gate.sh");

    let golden = read_json(&gate_dir.join("current-latest-golden-corpus.json"));
    let quality = read_json(&gate_dir.join("current-latest-quality.json"));
    let blockers = golden["release_blockers"]
        .as_array()
        .expect("golden blockers should be an array");

    assert!(
        blockers.iter().all(|blocker| !blocker["capability"]
            .as_str()
            .unwrap_or_default()
            .starts_with("config:src-app")),
        "viewer config rows should not remain duplicate config blockers: {blockers:#?}"
    );
    assert!(
        blockers.iter().any(|blocker| blocker["capability"]
            .as_str()
            .unwrap_or_default()
            .starts_with("config:src-globalconfig"))
            || blockers.iter().any(|blocker| blocker["capability"]
                .as_str()
                .unwrap_or_default()
                .starts_with("config:src-server-config")),
        "non-app external config blocker should remain visible: {blockers:#?}"
    );
    assert_eq!(golden["perfect_refactor_claim_allowed"], Value::Bool(false));
    assert_eq!(
        quality["perfect_refactor_claim_allowed"],
        Value::Bool(false)
    );
    assert!(
        quality["blockers"]
            .as_array()
            .expect("quality blockers should be an array")
            .iter()
            .any(
                |blocker| blocker["category"] == Value::String("golden-corpus".to_string())
                    || blocker["category"] == Value::String("current-target".to_string())
                    || blocker["category"] == Value::String("publication-authority".to_string())
            ),
        "quality gate must keep real blockers visible: {quality:#?}"
    );

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

fn assert_json_has_viewer_rows_without_config_rows(report: &Value, sources: &[&str]) {
    for source in sources {
        let rows = rows_for_source(report, source);
        assert!(!rows.is_empty(), "missing rows for {source}: {report:#?}");
        assert!(
            rows.iter()
                .all(|row| row["category"] != Value::String("config".to_string())),
            "src/app viewer config source must not create duplicate config row: {rows:#?}"
        );
        assert!(
            rows.iter().any(|row| {
                row["category"] == Value::String("viewer".to_string())
                    && row["level"] == Value::String("P1".to_string())
                    && row["evidence_kind"] == Value::String("snapshot".to_string())
            }),
            "src/app viewer config source should remain viewer P1 evidence: {rows:#?}"
        );
    }
}

fn assert_json_has_non_app_config_rows(report: &Value, sources: &[&str]) {
    for source in sources {
        let rows = rows_for_source(report, source);
        assert!(
            rows.iter().any(|row| {
                row["category"] == Value::String("config".to_string())
                    && row["level"] == Value::String("P0".to_string())
                    && (row["evidence_kind"] == Value::String("fixture".to_string())
                        || row["evidence_kind"] == Value::String("blocker".to_string()))
                    && (row["evidence_reference"]
                        .as_str()
                        .unwrap_or_default()
                        .starts_with("fixture:config:")
                        || row["evidence_reference"]
                            .as_str()
                            .unwrap_or_default()
                            .starts_with("blocker:config:"))
            }),
            "non-app config row should remain P0 config evidence or blocker: {source} {rows:#?}"
        );
    }
}

fn rows_for_source<'a>(report: &'a Value, source: &str) -> Vec<&'a Value> {
    let source_reference_suffix = format!(":{source}");
    report["rows"]
        .as_array()
        .expect("report rows should be an array")
        .iter()
        .filter(|row| {
            row["source_file"] == Value::String(source.to_string())
                || row["source_reference"]
                    .as_str()
                    .is_some_and(|reference| reference.ends_with(&source_reference_suffix))
        })
        .collect()
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
