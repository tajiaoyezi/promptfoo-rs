use std::fs;

use promptfoo_rs::compatibility::diff::DiffFinding;
use promptfoo_rs::compatibility::release_gate::{
    ReleaseChannel as GateReleaseChannel, ReleaseGateStatus, ReleaseGateSummary,
};
use promptfoo_rs::release::{
    default_release_checklist, evaluate_release_readiness, InstallChannel, ReleaseArtifactKind,
    ReleaseChannel,
};

fn ready_gate_summary() -> ReleaseGateSummary {
    ReleaseGateSummary {
        status: ReleaseGateStatus::Ready,
        release_channel: GateReleaseChannel::Stable,
        stable_allowed: true,
        blocking_findings: Vec::new(),
        required_p0_fixture_count: 50,
        observed_p0_fixture_count: 50,
        artifact_paths: vec!["compatibility/artifacts/release-gate/summary.json".to_string()],
        missing_artifact_paths: Vec::new(),
        p1_snapshot_total: 4,
        p1_snapshot_covered: 4,
        p2_registration_total: 2,
        p2_registered: 2,
        notes: vec![
            "P0 golden diff blockers: 0".to_string(),
            "P1 snapshot coverage: 4/4".to_string(),
            "P2 registration coverage: 2/2".to_string(),
        ],
    }
}

fn blocked_gate_summary() -> ReleaseGateSummary {
    ReleaseGateSummary {
        status: ReleaseGateStatus::Blocked,
        stable_allowed: false,
        blocking_findings: vec![DiffFinding::bug(
            "Eval runner",
            "$.summary.failed",
            "P0 failed count drift",
        )],
        ..ready_gate_summary()
    }
}

#[test]
fn test_10_2_1_release_checklist_contains_compatibility_gate_evidence() {
    // TEST-10.2.1
    let checklist = default_release_checklist();

    assert!(checklist.compatibility_gate.required_for_stable);
    assert!(checklist
        .compatibility_gate
        .evidence_paths
        .contains(&"docs/compatibility/baseline.lock.md"));
    assert!(checklist
        .compatibility_gate
        .evidence_paths
        .contains(&"docs/compatibility/matrix.md"));
    assert!(checklist.compatibility_gate.policy.contains("P0"));
    assert!(checklist
        .artifacts
        .iter()
        .any(|artifact| artifact.kind == ReleaseArtifactKind::Binary));
    assert!(checklist
        .artifacts
        .iter()
        .any(|artifact| artifact.kind == ReleaseArtifactKind::Container));

    let decision = evaluate_release_readiness(&ready_gate_summary(), &checklist);
    assert!(decision.stable_allowed);
    assert_eq!(decision.channel, ReleaseChannel::Stable);
}

#[test]
fn test_10_2_2_docs_and_install_channels_cover_release_surface() {
    // TEST-10.2.2
    let checklist = default_release_checklist();

    for channel in [
        InstallChannel::GitHubReleases,
        InstallChannel::Homebrew,
        InstallChannel::Cargo,
        InstallChannel::Docker,
        InstallChannel::NpmWrapper,
        InstallChannel::GitHubAction,
    ] {
        assert!(checklist.install_channels.contains(&channel));
    }

    assert!(checklist.docs.is_complete());
    for path in checklist.docs.required_paths() {
        let contents = fs::read_to_string(path).unwrap_or_else(|error| {
            panic!("required release doc {path} should be readable: {error}")
        });
        assert!(
            contents.contains("compatibility release gate")
                || contents.contains("Compatibility Matrix")
                || contents.contains("S2V"),
            "{path} should explain release or compatibility evidence"
        );
    }

    let readme = fs::read_to_string("README.md").expect("README exists");
    assert!(readme.contains("GitHub Releases"));
    assert!(readme.contains("cargo install"));
    assert!(readme.contains("Docker"));
    assert!(readme.contains("GitHub Action"));
}

#[test]
fn test_10_2_3_blocked_stable_release_downgrades_to_prerelease() {
    // TEST-10.2.3
    let checklist = default_release_checklist();
    let decision = evaluate_release_readiness(&blocked_gate_summary(), &checklist);

    assert!(!decision.stable_allowed);
    assert_eq!(decision.channel, ReleaseChannel::Prerelease);
    assert!(decision
        .reasons
        .iter()
        .any(|reason| reason.contains("compatibility release gate blocked")));
    assert!(decision
        .reasons
        .iter()
        .any(|reason| reason.contains("stable release is disabled")));
}
