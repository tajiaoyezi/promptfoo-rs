use promptfoo_rs::compatibility::harness::{
    reject_floating_baseline, ArtifactEngine, BaselineReference, FixtureSpec, HarnessRunner,
};
use promptfoo_rs::compatibility::normalize::{normalize_artifact, NormalizationRules};
use serde_json::json;

#[test]
fn test_6_1_1_harness_locks_baseline_and_rejects_latest() {
    let pinned_npm = BaselineReference::npm("promptfoo@0.121.13");
    let pinned_git = BaselineReference::git_commit("4860e990c7e9a2f8f677173fb92cf9867b34d03f");

    reject_floating_baseline(&pinned_npm).expect("pinned npm baseline is accepted");
    reject_floating_baseline(&pinned_git).expect("pinned git commit is accepted");

    let latest = BaselineReference::npm("promptfoo@latest");
    let err = reject_floating_baseline(&latest).expect_err("latest is rejected");
    assert!(err.to_string().contains("floating baseline"));

    let head = BaselineReference::git_commit("HEAD");
    let err = reject_floating_baseline(&head).expect_err("HEAD is rejected");
    assert!(err.to_string().contains("floating baseline"));
}

#[test]
fn test_6_1_2_same_fixture_generates_upstream_and_rs_artifacts() {
    let fixture = FixtureSpec::new(
        "minimal-eval",
        BaselineReference::npm("promptfoo@0.121.13"),
        json!({
            "providers": [{ "id": "echo" }],
            "prompts": ["Hello {{name}}"],
            "tests": [{ "vars": { "name": "Ada" } }]
        }),
    );

    let artifacts = HarnessRunner::new()
        .run_fixture(&fixture)
        .expect("fixture runs through harness");

    assert_eq!(artifacts.fixture_name, "minimal-eval");
    assert_eq!(artifacts.baseline.reference, "promptfoo@0.121.13");
    assert_eq!(artifacts.upstream.engine, ArtifactEngine::UpstreamPromptfoo);
    assert_eq!(artifacts.rs.engine, ArtifactEngine::PromptfooRs);
    assert_eq!(artifacts.upstream.fixture_name, artifacts.rs.fixture_name);
    assert_eq!(
        artifacts.upstream.payload["input"],
        artifacts.rs.payload["input"]
    );
}

#[test]
fn test_6_1_3_normalization_rules_snapshot_time_path_random_id_and_latency() {
    let fixture = FixtureSpec::new(
        "normalization",
        BaselineReference::npm("promptfoo@0.121.13"),
        json!({ "tests": [] }),
    );
    let artifact = HarnessRunner::new()
        .run_fixture(&fixture)
        .expect("fixture runs")
        .upstream
        .with_payload(json!({
            "timestamp": "2026-05-30T12:34:56Z",
            "config_path": "C:\\Users\\15783\\AppData\\Local\\Temp\\promptfoo\\case.yaml",
            "run_id": "run_01HZY7WJ4T6RB8Q1P9Q3MZ7R2K",
            "latency_ms": 2487,
            "nested": {
                "durationMs": 91,
                "artifactPath": "/tmp/promptfoo-rs/result.json"
            }
        }));

    let normalized =
        normalize_artifact(&artifact, &NormalizationRules::default_promptfoo_0_121_13());

    assert_eq!(
        normalized.payload,
        json!({
            "timestamp": "<normalized-timestamp>",
            "config_path": "<normalized-path>",
            "run_id": "<normalized-random-id>",
            "latency_ms": "<normalized-latency>",
            "nested": {
                "durationMs": "<normalized-latency>",
                "artifactPath": "<normalized-path>"
            }
        })
    );
    assert_eq!(
        normalized.applied_rules,
        vec!["latency", "path", "random-id", "timestamp"]
    );
}
