use std::collections::BTreeMap;

use promptfoo_rs::compatibility::release_gate::{
    ReleaseChannel as CompatibilityReleaseChannel, ReleaseGateStatus, ReleaseGateSummary,
};
use promptfoo_rs::release::{
    release_candidate_gate, PackageNames, PackagingArtifact, PackagingSmokeReport, PerformanceHost,
    PerformanceRun, ReleaseCandidateGateConfig, ReleaseChannel, SecurityRun,
};

#[test]
fn test_16_2_1_runtime_smoke_script_replaces_synthetic_performance_literals() {
    /* TEST-16.2.1 */
    let script = std::fs::read_to_string("scripts/release/runtime-smoke.sh")
        .expect("runtime smoke script should exist");

    assert!(
        script.contains("measure_ms"),
        "runtime smoke must measure command duration rather than write fixed observed values"
    );
    assert!(
        script.contains("mock_eval_cases"),
        "runtime smoke must record measured mock eval evidence"
    );
    for forbidden in [
        "\"cli_cold_start_ms\": 120",
        "\"mock_eval_duration_ms\": 2750",
        "\"memory_baseline_mb\": 64",
        "cat > target/release-gates/performance.json <<'JSON'",
    ] {
        assert!(
            !script.contains(forbidden),
            "synthetic literal remains: {forbidden}"
        );
    }
}

#[test]
fn test_16_2_2_security_and_performance_blockers_disable_stable_release() {
    /* TEST-16.2.2 */
    let summary = release_candidate_gate(&ReleaseCandidateGateConfig {
        trace_id: "trace-16.2.blocked".to_string(),
        adapter_commands: release_adapter_commands(),
        compatibility: ready_compatibility_summary(),
        performance: slow_performance_run(),
        security: insecure_run(),
        packaging: package_smoke_report(),
        artifact_paths: vec![
            "target/release-gates/performance.json".to_string(),
            "target/release-gates/security.json".to_string(),
        ],
    });

    assert_eq!(summary.decision, ReleaseChannel::Prerelease, "{summary:#?}");
    assert!(!summary.stable_allowed, "{summary:#?}");
    assert_eq!(
        summary.gate_statuses.get("performance"),
        Some(&ReleaseGateStatus::Blocked),
        "{summary:#?}"
    );
    assert_eq!(
        summary.gate_statuses.get("security"),
        Some(&ReleaseGateStatus::Blocked),
        "{summary:#?}"
    );
}

#[test]
fn test_16_2_3_runtime_smoke_script_derives_release_candidate_decision() {
    /* TEST-16.2.3 */
    let script = std::fs::read_to_string("scripts/release/runtime-smoke.sh")
        .expect("runtime smoke script should exist");

    assert!(
        script.contains("stable_allowed_from_gate"),
        "runtime smoke must derive stable_allowed from gate statuses"
    );
    assert!(
        script.contains("validate_report_json"),
        "runtime smoke must validate generated report JSON before success"
    );
    for forbidden in [
        "\"decision\": \"stable\"",
        "\"stable_allowed\": true",
        "cat > target/release-gates/release-candidate.json <<'JSON'",
    ] {
        assert!(
            !script.contains(forbidden),
            "fixed release decision remains: {forbidden}"
        );
    }
}

#[test]
fn test_16_2_4_runtime_smoke_runs_cli_command_closure_and_security_checks() {
    /* TEST-16.2.4 */
    let script = std::fs::read_to_string("scripts/release/runtime-smoke.sh")
        .expect("runtime smoke script should exist");

    assert!(
        script.contains("--test cli_command_behavior_closure"),
        "{script}"
    );
    assert!(script.contains("--test security_redaction"), "{script}");
    assert!(script.contains("no_upload_evidence"), "{script}");
    assert!(
        script.contains("upload_attempts"),
        "runtime smoke security report must record upload attempts"
    );
}

fn release_adapter_commands() -> BTreeMap<String, String> {
    [
        ("lint", "scripts/release/lint.sh"),
        ("integration", "scripts/release/integration.sh"),
        ("e2e", "scripts/release/e2e.sh"),
        ("coverage", "scripts/release/coverage.sh"),
        ("runtime-smoke", "scripts/release/runtime-smoke.sh"),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_string(), value.to_string()))
    .collect()
}

fn ready_compatibility_summary() -> ReleaseGateSummary {
    ReleaseGateSummary {
        status: ReleaseGateStatus::Ready,
        release_channel: CompatibilityReleaseChannel::Stable,
        stable_allowed: true,
        blocking_findings: Vec::new(),
        required_p0_fixture_count: 50,
        observed_p0_fixture_count: 50,
        artifact_paths: vec!["target/release-gates/compatibility.json".to_string()],
        missing_artifact_paths: Vec::new(),
        p1_snapshot_total: 10,
        p1_snapshot_covered: 10,
        p2_registration_total: 5,
        p2_registered: 5,
        notes: vec!["compatibility gate ready".to_string()],
    }
}

fn slow_performance_run() -> PerformanceRun {
    PerformanceRun {
        cli_cold_start_ms: 301,
        mock_eval_cases: 1_000,
        mock_eval_duration_ms: 5_001,
        memory_baseline_mb: 101,
        host: PerformanceHost {
            os: "windows".to_string(),
            arch: "x86_64".to_string(),
            cpu: "local-ci".to_string(),
            rustc: "rustc 1.95.0".to_string(),
            profile: "release-candidate".to_string(),
        },
        artifact_path: "target/release-gates/performance.json".to_string(),
    }
}

fn insecure_run() -> SecurityRun {
    SecurityRun {
        custom_scripts_default_denied: false,
        unauthorized_error_code: "allowed".to_string(),
        log_sample: "Authorization=sk-live-secret".to_string(),
        artifact_sample: r#"{"apiKey":"token-123"}"#.to_string(),
        known_secret_values: vec!["sk-live-secret".to_string(), "token-123".to_string()],
        upload_attempts: 1,
        no_upload_evidence: Vec::new(),
        artifact_path: "target/release-gates/security.json".to_string(),
    }
}

fn package_smoke_report() -> PackagingSmokeReport {
    PackagingSmokeReport {
        dry_run: true,
        published: false,
        package_names: PackageNames {
            viewer: "@promptfoo-rs/viewer".to_string(),
            npm_wrapper: "@promptfoo-rs/node".to_string(),
        },
        artifacts: vec![PackagingArtifact {
            name: "viewer-dist".to_string(),
            path: "target/package-smoke/viewer-dist.json".to_string(),
            version: "0.1.0".to_string(),
            checksum_sha256: "viewer-checksum".to_string(),
        }],
        no_publish_evidence: "dry_run=true; publish=false".to_string(),
    }
}
