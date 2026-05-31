use std::path::Path;

use promptfoo_rs::compatibility::harness::{
    validate_corpus_artifacts, write_corpus_index, CorpusFixtureArtifacts, CorpusRunSummary,
};
use promptfoo_rs::compatibility::release_gate::ReleaseGateStatus;

fn ready_fixture(index: usize) -> CorpusFixtureArtifacts {
    CorpusFixtureArtifacts {
        fixture_id: format!("p0-fixture-{index:02}"),
        matrix_item_ids: vec!["command:eval".to_string()],
        upstream_command: "npx --yes promptfoo@0.121.13 eval -c promptfooconfig.yaml".to_string(),
        rs_command: "target/release/promptfoo-rs eval -c promptfooconfig.yaml".to_string(),
        used_test_binary: false,
        upstream_exit_code: 0,
        rs_exit_code: 0,
        duration_ms: 25,
        normalization_rules: vec!["time".to_string(), "path".to_string(), "latency".to_string()],
        artifact_paths: vec![
            format!("target/release-gates/real-upstream-corpus/p0-fixture-{index:02}/metadata.json"),
            format!("target/release-gates/real-upstream-corpus/p0-fixture-{index:02}/raw/upstream.json"),
            format!("target/release-gates/real-upstream-corpus/p0-fixture-{index:02}/raw/rs.json"),
            format!("target/release-gates/real-upstream-corpus/p0-fixture-{index:02}/normalized/upstream.json"),
            format!("target/release-gates/real-upstream-corpus/p0-fixture-{index:02}/normalized/rs.json"),
            format!("target/release-gates/real-upstream-corpus/p0-fixture-{index:02}/diff/findings.json"),
        ],
        diff_findings: Vec::new(),
    }
}

#[test]
fn test_17_3_1_real_corpus_summary_requires_50_real_upstream_runs() {
    /* TEST-17.3.1 */
    let summary = CorpusRunSummary::new((0..50).map(ready_fixture).collect());
    let gate = validate_corpus_artifacts(&summary, 50);

    assert_eq!(gate.status, ReleaseGateStatus::Ready, "{gate:#?}");
    assert!(gate.stable_allowed, "{gate:#?}");
    assert_eq!(gate.observed_p0_fixture_count, 50);
    assert!(summary.fixtures.iter().all(|fixture| !fixture.used_test_binary));

    let too_small = CorpusRunSummary::new((0..49).map(ready_fixture).collect());
    let blocked = validate_corpus_artifacts(&too_small, 50);
    assert_eq!(blocked.status, ReleaseGateStatus::Blocked, "{blocked:#?}");
    assert!(!blocked.stable_allowed, "{blocked:#?}");
}

#[test]
fn test_17_3_2_fixture_artifacts_record_raw_normalized_diff_and_metadata() {
    /* TEST-17.3.2 */
    let mut fixture = ready_fixture(0);
    fixture.duration_ms = 31;
    let summary = CorpusRunSummary::new(vec![fixture.clone()]);

    assert_eq!(summary.fixtures[0].fixture_id, "p0-fixture-00");
    assert!(summary.fixtures[0]
        .artifact_paths
        .iter()
        .any(|path| path.ends_with("raw/upstream.json")));
    assert!(summary.fixtures[0]
        .artifact_paths
        .iter()
        .any(|path| path.ends_with("normalized/rs.json")));
    assert!(summary.fixtures[0]
        .artifact_paths
        .iter()
        .any(|path| path.ends_with("diff/findings.json")));
    assert!(summary.fixtures[0].duration_ms > 0);

    let path = std::env::temp_dir().join(format!(
        "promptfoo-rs-corpus-index-{}.json",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&path);
    write_corpus_index(&summary, Path::new(&path)).expect("index should write");
    let json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&path).expect("index should read"))
            .expect("index should be json");
    let _ = std::fs::remove_file(&path);
    assert_eq!(json["schema"], "promptfoo-rs.real-upstream-corpus.v1");
    assert_eq!(json["fixtures"].as_array().unwrap().len(), 1);
}

#[test]
fn test_17_3_3_corpus_script_executes_real_upstream_not_test_binary() {
    /* TEST-17.3.3 */
    let script = std::fs::read_to_string("scripts/release/real-upstream-corpus.sh")
        .expect("real upstream corpus script should exist");

    assert!(script.contains("npx --yes promptfoo@0.121.13"), "{script}");
    assert!(script.contains("used_test_binary"), "{script}");
    assert!(script.contains("false"), "{script}");
    assert!(script.contains("required_p0_fixture_count"), "{script}");
    assert!(!script.contains("current_exe"), "{script}");
}

#[test]
fn test_17_3_4_runtime_and_integration_gates_include_real_corpus() {
    /* TEST-17.3.4 */
    let runtime = std::fs::read_to_string("scripts/release/runtime-smoke.sh")
        .expect("runtime smoke script should exist");
    let integration = std::fs::read_to_string("scripts/release/integration.sh")
        .expect("integration script should exist");

    assert!(runtime.contains("real-upstream-corpus.sh"), "{runtime}");
    assert!(runtime.contains("real-upstream-corpus/index.json"), "{runtime}");
    assert!(
        integration.contains("--test real_p0_golden_corpus_runner"),
        "{integration}"
    );
}
