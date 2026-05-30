use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use promptfoo_rs::compatibility::diff::{DiffFinding, DiffClass};
use promptfoo_rs::compatibility::matrix::{CapabilityMatrix, CapabilityRow};
use promptfoo_rs::compatibility::release_gate::{
    assert_stable_allowed, run_full_compatibility_gate, write_release_gate_summary, GateConfig,
    ReleaseChannel, ReleaseGateStatus,
};
use serde_json::Value;

fn matrix_row(capability: &str, level: &str) -> CapabilityRow {
    CapabilityRow {
        capability: capability.to_string(),
        level: level.to_string(),
        target_status: "native".to_string(),
        verification: "golden diff".to_string(),
        owner: "leafiellune".to_string(),
        notes: "registered by TEST-12.3".to_string(),
    }
}

fn matrix() -> CapabilityMatrix {
    CapabilityMatrix {
        rows: vec![
            matrix_row("command:eval", "P0"),
            matrix_row("provider:openai", "P0"),
            matrix_row("provider:long-tail", "P2"),
        ],
    }
}

fn temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be available")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("promptfoo-rs-{name}-{nanos}"));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("temp dir should be created");
    root
}

#[test]
fn test_12_3_1_release_workflow_runs_full_gate_before_stable_artifact_build() {
    /* TEST-12.3.1 */
    let workflow = fs::read_to_string(".github/workflows/release.yml")
        .expect("release workflow should exist");
    let gate_index = workflow
        .find("Full compatibility release gate")
        .expect("release workflow should run full compatibility gate");
    let build_index = workflow
        .find("Build release binary")
        .expect("release workflow should build release binary");

    assert!(gate_index < build_index, "gate must run before release build");
    assert!(workflow.contains("golden_diff_ci_release_gate"));
    assert!(workflow.contains("compatibility/artifacts/release-gate"));
}

#[test]
fn test_12_3_2_p0_bug_unclassified_and_missing_fixture_coverage_block_stable() {
    /* TEST-12.3.2 */
    let config = GateConfig {
        matrix: matrix(),
        findings: vec![
            DiffFinding::bug("command:eval", "$.exitCode", "exit code drift"),
            DiffFinding::new(
                "provider:openai",
                "$.response",
                DiffClass::Unclassified,
                "response shape drift lacks classification",
            ),
        ],
        required_p0_fixture_count: 50,
        observed_p0_fixture_count: 49,
        artifact_paths: vec![],
        release_channel: ReleaseChannel::Stable,
    };

    let summary = run_full_compatibility_gate(&config).expect("gate should evaluate");
    assert_eq!(summary.status, ReleaseGateStatus::Blocked);
    assert_eq!(summary.blocking_findings.len(), 2);
    assert!(summary
        .notes
        .iter()
        .any(|note| note.contains("P0 fixture coverage missing: 49/50")));

    let error = assert_stable_allowed(&summary).expect_err("stable release must be blocked");
    assert_ne!(error.exit_code(), 0);
}

#[test]
fn test_12_3_3_gate_summary_records_channel_decision_and_artifact_paths() {
    /* TEST-12.3.3 */
    let root = temp_dir("test-12-3-3");
    let raw_artifact = root.join("raw-upstream.json");
    let diff_artifact = root.join("diff-findings.json");
    fs::write(&raw_artifact, "{}").expect("raw artifact should be written");
    fs::write(&diff_artifact, "[]").expect("diff artifact should be written");

    let config = GateConfig {
        matrix: matrix(),
        findings: vec![DiffFinding::new(
            "command:eval",
            "$",
            DiffClass::Matching,
            "normalized artifacts match",
        )],
        required_p0_fixture_count: 50,
        observed_p0_fixture_count: 50,
        artifact_paths: vec![raw_artifact.clone(), diff_artifact.clone()],
        release_channel: ReleaseChannel::Stable,
    };
    let summary = run_full_compatibility_gate(&config).expect("gate should evaluate");
    assert_eq!(summary.status, ReleaseGateStatus::Ready);
    assert!(summary.stable_allowed);
    assert_eq!(summary.release_channel, ReleaseChannel::Stable);
    assert!(summary.artifact_paths.contains(&raw_artifact.display().to_string()));

    let summary_path = root.join("summary.json");
    write_release_gate_summary(&summary, &summary_path).expect("summary should be written");
    let json: Value = serde_json::from_str(
        &fs::read_to_string(summary_path).expect("summary should be readable"),
    )
    .expect("summary should be json");

    assert_eq!(json["release_channel"], "stable");
    assert_eq!(json["stable_allowed"], true);
    assert_eq!(json["artifact_paths"].as_array().expect("paths array").len(), 2);
}
