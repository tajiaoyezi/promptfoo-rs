use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use promptfoo_rs::compatibility::executor::CommandSpec;
use promptfoo_rs::compatibility::fixtures::{FixtureManifest, Priority, ProviderMocking};
use promptfoo_rs::compatibility::harness::{
    BaselineReference, ExecutableHarnessRunner, PromptfooCommand,
};
use serde_json::Value;

fn fixture() -> FixtureManifest {
    FixtureManifest {
        id: "TEST-12.2-fixture".to_string(),
        test_id: "TEST-12.2.1".to_string(),
        matrix_item_ids: vec!["command:eval".to_string()],
        priority: Priority::P0,
        provider_mocking: ProviderMocking::Mock,
        required_env: vec![],
        expected_outputs: vec!["json".to_string()],
        normalization_rules: vec!["timestamp".to_string(), "latency".to_string()],
        blocks_stable_release: true,
    }
}

fn temp_artifact_root(test_name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be monotonic enough for test temp path")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("promptfoo-rs-{test_name}-{nanos}"));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("temp artifact root should be created");
    root
}

fn read_json(path: &Path) -> Value {
    let text = fs::read_to_string(path).expect("json artifact should be readable");
    serde_json::from_str(&text).expect("json artifact should parse")
}

#[test]
fn test_12_2_1_runner_executes_upstream_and_rs_for_same_fixture() {
    /* TEST-12.2.1 */
    let exe = std::env::current_exe().expect("test binary path is available");
    let upstream = CommandSpec::new(&exe)
        .arg("--list")
        .env("PROMPTFOO_TEST_ENGINE", "upstream");
    let rs = CommandSpec::new(&exe)
        .arg("--list")
        .env("PROMPTFOO_TEST_ENGINE", "rs");
    let runner = ExecutableHarnessRunner::new(
        temp_artifact_root("test-12-2-1"),
        BaselineReference::npm("promptfoo@0.121.13"),
    )
    .with_run_id("TEST-12.2.1")
    .with_command_specs(upstream, rs);

    let artifacts = runner
        .run_fixture(&fixture())
        .expect("fixture should execute through upstream and rs commands");

    let upstream_raw = read_json(&artifacts.upstream_raw_path);
    let rs_raw = read_json(&artifacts.rs_raw_path);

    assert_eq!(artifacts.run_id, "TEST-12.2.1");
    assert_eq!(upstream_raw["engine"], "upstream-promptfoo");
    assert_eq!(rs_raw["engine"], "promptfoo-rs");
    assert_eq!(upstream_raw["fixture_id"], rs_raw["fixture_id"]);
    assert_eq!(upstream_raw["execution"]["exit_code"], 0);
    assert_eq!(rs_raw["execution"]["exit_code"], 0);
}

#[test]
fn test_12_2_2_runner_persists_raw_normalized_diff_and_metadata_tree() {
    /* TEST-12.2.2 */
    let exe = std::env::current_exe().expect("test binary path is available");
    let runner = ExecutableHarnessRunner::new(
        temp_artifact_root("test-12-2-2"),
        BaselineReference::npm("promptfoo@0.121.13"),
    )
    .with_run_id("TEST-12.2.2")
    .with_command_specs(
        CommandSpec::new(&exe).arg("--list"),
        CommandSpec::new(&exe).arg("--list"),
    );

    let artifacts = runner
        .run_fixture(&fixture())
        .expect("fixture should persist artifacts");

    assert!(artifacts.run_dir.exists());
    assert!(artifacts.metadata_path.exists());
    assert!(artifacts.upstream_raw_path.exists());
    assert!(artifacts.rs_raw_path.exists());
    assert!(artifacts.upstream_normalized_path.exists());
    assert!(artifacts.rs_normalized_path.exists());
    assert!(artifacts.diff_path.exists());

    let metadata = read_json(&artifacts.metadata_path);
    let diff = read_json(&artifacts.diff_path);
    assert_eq!(metadata["fixture_id"], "TEST-12.2-fixture");
    assert_eq!(metadata["run_id"], "TEST-12.2.2");
    assert_eq!(metadata["baseline"]["reference"], "promptfoo@0.121.13");
    assert!(!diff
        .as_array()
        .expect("diff artifact should be an array")
        .is_empty());
}

#[test]
fn test_12_2_3_command_policy_enforces_timeout_env_update_disable_and_no_secrets() {
    /* TEST-12.2.3 */
    let baseline = BaselineReference::npm("promptfoo@0.121.13");
    let upstream = PromptfooCommand::upstream_pinned(&baseline);
    let rs = PromptfooCommand::current_rs(Path::new("target/debug/promptfoo-rs"));

    assert!(upstream.env_clear);
    assert!(rs.env_clear);
    assert_eq!(
        upstream.env.get("PROMPTFOO_DISABLE_UPDATE"),
        Some(&"true".to_string())
    );
    assert_eq!(
        rs.env.get("PROMPTFOO_DISABLE_UPDATE"),
        Some(&"true".to_string())
    );
    assert!(upstream.timeout_ms > 0 && upstream.timeout_ms <= 120_000);
    assert!(rs.timeout_ms > 0 && rs.timeout_ms <= 120_000);
    assert!(upstream
        .args
        .iter()
        .any(|arg| arg.contains("promptfoo@0.121.13")));

    let exe = std::env::current_exe().expect("test binary path is available");
    let runner = ExecutableHarnessRunner::new(
        temp_artifact_root("test-12-2-3"),
        BaselineReference::npm("promptfoo@0.121.13"),
    )
    .with_command_specs(
        CommandSpec::new(&exe).arg("--list"),
        CommandSpec::new(&exe).arg("--list"),
    );
    let mut secret_fixture = fixture();
    secret_fixture.required_env = vec!["OPENAI_API_KEY".to_string()];

    let error = runner
        .run_fixture(&secret_fixture)
        .expect_err("fixtures requiring real secrets are rejected");
    assert!(error.to_string().contains("real secret"));
}
