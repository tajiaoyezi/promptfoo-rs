use std::collections::BTreeMap;

use promptfoo_rs::compatibility::release_gate::{
    ReleaseChannel as CompatibilityReleaseChannel, ReleaseGateStatus, ReleaseGateSummary,
};
use promptfoo_rs::release::{
    evaluate_performance_baseline, release_candidate_gate, PackageNames, PackagingArtifact,
    PackagingSmokeReport, PerformanceHost, PerformanceRun, ReleaseCandidateGateConfig,
    ReleaseChannel, SecurityRun,
};

#[test]
fn test_15_2_2_performance_gate_blocks_noisy_or_slow_release_candidate() {
    /* TEST-15.2.2 */
    let passing = evaluate_performance_baseline(&fast_performance_run());

    assert_eq!(passing.status, ReleaseGateStatus::Ready, "{passing:#?}");
    assert!(passing.blocking_evidence.is_empty(), "{passing:#?}");
    assert_eq!(passing.thresholds.cli_cold_start_ms, 300);
    assert_eq!(passing.thresholds.mock_eval_duration_ms, 5_000);
    assert_eq!(passing.thresholds.memory_baseline_mb, 100);
    assert_eq!(passing.run.mock_eval_cases, 1_000);
    assert!(!passing.run.host.os.trim().is_empty(), "{passing:#?}");

    let blocked = evaluate_performance_baseline(&PerformanceRun {
        cli_cold_start_ms: 301,
        mock_eval_cases: 999,
        mock_eval_duration_ms: 5_001,
        memory_baseline_mb: 101,
        ..fast_performance_run()
    });

    assert_eq!(blocked.status, ReleaseGateStatus::Blocked, "{blocked:#?}");
    assert!(
        blocked
            .blocking_evidence
            .iter()
            .any(|evidence| evidence.contains("CLI cold start")),
        "{blocked:#?}"
    );
    assert!(
        blocked
            .blocking_evidence
            .iter()
            .any(|evidence| evidence.contains("1000 mock eval")),
        "{blocked:#?}"
    );
    assert!(
        blocked
            .blocking_evidence
            .iter()
            .any(|evidence| evidence.contains("memory baseline")),
        "{blocked:#?}"
    );
}

#[test]
fn test_15_2_4_release_candidate_summary_records_trace_artifacts_and_decision() {
    /* TEST-15.2.4 */
    let summary = release_candidate_gate(&ReleaseCandidateGateConfig {
        trace_id: "trace-15.2.1-local".to_string(),
        adapter_commands: release_adapter_commands(),
        compatibility: ready_compatibility_summary(),
        performance: fast_performance_run(),
        security: secure_run(),
        packaging: package_smoke_report(),
        artifact_paths: vec![
            "target/release-gates/performance.json".to_string(),
            "target/release-gates/security.json".to_string(),
        ],
    });

    assert_eq!(summary.trace_id, "trace-15.2.1-local");
    assert_eq!(summary.decision, ReleaseChannel::Stable, "{summary:#?}");
    assert!(summary.stable_allowed, "{summary:#?}");
    for gate in [
        "adapter",
        "compatibility",
        "performance",
        "security",
        "packaging",
        "observability",
    ] {
        assert_eq!(
            summary.gate_statuses.get(gate),
            Some(&ReleaseGateStatus::Ready),
            "{summary:#?}"
        );
    }
    assert!(
        summary
            .artifact_paths
            .iter()
            .any(|path| path.ends_with("viewer-dist.json")),
        "{summary:#?}"
    );
    assert!(
        summary
            .artifact_paths
            .iter()
            .any(|path| path.ends_with("performance.json")),
        "{summary:#?}"
    );
}

fn fast_performance_run() -> PerformanceRun {
    PerformanceRun {
        cli_cold_start_ms: 120,
        mock_eval_cases: 1_000,
        mock_eval_duration_ms: 2_750,
        memory_baseline_mb: 64,
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

fn secure_run() -> SecurityRun {
    SecurityRun {
        custom_scripts_default_denied: true,
        unauthorized_error_code: "script_not_authorized".to_string(),
        log_sample: "Authorization=[REDACTED]".to_string(),
        artifact_sample: r#"{"apiKey":"[REDACTED]"}"#.to_string(),
        known_secret_values: vec!["sk-live-secret".to_string(), "token-123".to_string()],
        upload_attempts: 0,
        no_upload_evidence: vec!["local-only release smoke".to_string()],
        artifact_path: "target/release-gates/security.json".to_string(),
    }
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

fn package_smoke_report() -> PackagingSmokeReport {
    PackagingSmokeReport {
        dry_run: true,
        published: false,
        package_names: PackageNames {
            viewer: "@promptfoo-rs/viewer".to_string(),
            npm_wrapper: "@promptfoo-rs/node".to_string(),
        },
        artifacts: vec![
            PackagingArtifact {
                name: "viewer-dist".to_string(),
                path: "target/package-smoke/viewer-dist.json".to_string(),
                version: "0.1.0".to_string(),
                checksum_sha256: "viewer-checksum".to_string(),
            },
            PackagingArtifact {
                name: "npm-wrapper-dist".to_string(),
                path: "target/package-smoke/npm-wrapper-dist.json".to_string(),
                version: "0.1.0".to_string(),
                checksum_sha256: "npm-checksum".to_string(),
            },
        ],
        no_publish_evidence: "dry_run=true; publish=false".to_string(),
    }
}
