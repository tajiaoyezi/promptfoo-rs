use std::path::{Path, PathBuf};

use promptfoo_rs::release::{
    classify_publication_blockers, collect_channel_evidence, write_release_installability_report,
    ChannelEvidenceStatus, PublicationReadiness, ReleaseChannel, ReleaseInstallabilityConfig,
    ReleaseInstallabilityRunner,
};

#[test]
fn test_17_5_1_installability_report_records_all_dry_run_channels() {
    /* TEST-17.5.1 */
    let report = run_report("test-17-5-installability");

    assert!(report.installability_ready, "{report:#?}");
    assert!(!report.channels.is_empty(), "{report:#?}");
    for channel in [
        ReleaseChannel::GitHubReleases,
        ReleaseChannel::Cargo,
        ReleaseChannel::NpmWrapper,
        ReleaseChannel::Docker,
        ReleaseChannel::Homebrew,
        ReleaseChannel::GitHubAction,
    ] {
        let evidence = report
            .channel(channel)
            .unwrap_or_else(|| panic!("missing channel evidence for {channel:?}: {report:#?}"));
        assert!(
            matches!(
                evidence.status,
                ChannelEvidenceStatus::Ready
                    | ChannelEvidenceStatus::ToolUnavailable
                    | ChannelEvidenceStatus::CredentialBlocked
            ),
            "{evidence:#?}"
        );
        assert!(
            !evidence.command.trim().is_empty()
                || !evidence.blocker.as_deref().unwrap_or("").is_empty(),
            "{evidence:#?}"
        );
    }

    assert!(
        report
            .artifact_paths
            .iter()
            .any(|path| path.ends_with("release-archive.tar.gz")),
        "{report:#?}"
    );
    assert!(report
        .artifact_paths
        .iter()
        .any(|path| path.ends_with("cargo-package-dry-run.json")));
    assert!(report
        .artifact_paths
        .iter()
        .any(|path| path.ends_with("npm-pack.json")));
}

#[test]
fn test_17_5_2_publication_is_credential_blocked_without_external_artifacts() {
    /* TEST-17.5.2 */
    let report = run_report("test-17-5-publication");
    let readiness = classify_publication_blockers(&report);

    assert_eq!(readiness, PublicationReadiness::CredentialBlocked);
    assert_eq!(
        report.publication_ready,
        PublicationReadiness::CredentialBlocked
    );
    assert!(report.credential_blocked, "{report:#?}");
    assert!(
        report
            .publication_blockers
            .iter()
            .any(|blocker| blocker.contains("GitHub")),
        "{report:#?}"
    );
    assert!(
        report
            .channels
            .iter()
            .all(|channel| !channel.published && channel.external_url.is_none()),
        "{report:#?}"
    );
}

#[test]
fn test_17_5_3_release_workflow_requires_full_gate_before_stable_artifacts() {
    /* TEST-17.5.3 */
    let workflow =
        std::fs::read_to_string(".github/workflows/release.yml").expect("workflow should exist");
    let installability_script = std::fs::read_to_string("scripts/release/installability.sh")
        .expect("installability script should exist");

    let gate_index = workflow
        .find("s2v_verify_full \"install lint typecheck unit-test integration e2e coverage build runtime-smoke\"")
        .expect("workflow must run full S2V phase gate");
    let corpus_index = workflow
        .find("real-upstream-corpus")
        .expect("workflow must require real upstream corpus artifacts");
    let build_index = workflow
        .find("Build release binary")
        .expect("workflow must build release binary");
    assert!(gate_index < build_index, "{workflow}");
    assert!(corpus_index < build_index, "{workflow}");

    assert!(
        installability_script.contains("runtime-smoke.sh")
            && installability_script.contains("real-upstream-corpus")
            && installability_script.contains("installability.json"),
        "{installability_script}"
    );
}

#[test]
fn test_17_5_4_report_has_checksums_no_upload_and_no_secret_leakage() {
    /* TEST-17.5.4 */
    let report = run_report("test-17-5-security");
    let output = PathBuf::from("target/test-release-installability/test-17-5-security/report.json");
    write_release_installability_report(&report, &output).expect("report should write");
    let json = std::fs::read_to_string(&output).expect("report should be readable");

    assert!(
        report.no_upload_evidence.contains("no upload"),
        "{report:#?}"
    );
    assert_eq!(report.security_gate_status, "ready");
    assert!(!report.checksums.is_empty(), "{report:#?}");
    assert!(report
        .checksums
        .iter()
        .all(|checksum| checksum.sha256.len() == 64));
    for forbidden in [
        "sk-",
        "ghp_",
        "NPM_TOKEN",
        "CARGO_REGISTRY_TOKEN",
        "DOCKER_PASSWORD",
    ] {
        assert!(
            !json.contains(forbidden),
            "secret marker leaked: {forbidden}"
        );
    }
}

#[test]
fn test_17_5_5_collect_channel_evidence_records_tool_blockers() {
    let cargo = collect_channel_evidence(ReleaseChannel::Cargo, Path::new("."));
    assert_eq!(cargo.status, ChannelEvidenceStatus::Ready, "{cargo:#?}");
    assert!(cargo.command.contains("cargo package"), "{cargo:#?}");

    let homebrew = collect_channel_evidence(ReleaseChannel::Homebrew, Path::new("."));
    assert!(
        matches!(
            homebrew.status,
            ChannelEvidenceStatus::Ready | ChannelEvidenceStatus::ToolUnavailable
        ),
        "{homebrew:#?}"
    );
    assert!(
        homebrew.command.contains("brew")
            || homebrew
                .blocker
                .as_deref()
                .unwrap_or("")
                .contains("Homebrew"),
        "{homebrew:#?}"
    );
}

fn run_report(case: &str) -> promptfoo_rs::release::ReleaseInstallabilityReport {
    ReleaseInstallabilityRunner::run(&ReleaseInstallabilityConfig {
        workspace: PathBuf::from("."),
        out_dir: PathBuf::from("target/test-release-installability").join(case),
        version: "0.1.0".to_string(),
        publish_credentials_present: false,
    })
    .expect("installability report should run")
}
