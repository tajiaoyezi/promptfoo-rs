use std::path::PathBuf;

use promptfoo_rs::release::{
    collect_publication_authority, validate_publication_evidence,
    write_publication_authority_report, ChannelEvidenceStatus, CredentialProbeStatus,
    PublicationAuthorityStatus, PublicationReadiness, PublishedEvidence, ReleaseChannel,
};

#[test]
fn test_18_4_1_channels_separate_installability_from_authority_status() {
    /* TEST-18.4.1 */
    let report = collect_publication_authority(&release_channels());

    assert_eq!(report.schema, "promptfoo-rs.publication-authority.v1");
    assert_eq!(
        report.publication_ready,
        PublicationReadiness::CredentialBlocked,
        "{report:#?}"
    );
    for channel in release_channels() {
        let evidence = report
            .channel(channel)
            .unwrap_or_else(|| panic!("missing publication authority evidence for {channel:?}"));
        assert!(
            matches!(
                evidence.installability_status,
                ChannelEvidenceStatus::Ready
                    | ChannelEvidenceStatus::ToolUnavailable
                    | ChannelEvidenceStatus::CredentialBlocked
                    | ChannelEvidenceStatus::Blocked
            ),
            "{evidence:#?}"
        );
        assert!(
            matches!(
                evidence.authority_status,
                PublicationAuthorityStatus::CredentialBlocked
                    | PublicationAuthorityStatus::ToolUnavailable
                    | PublicationAuthorityStatus::LegalBrandBlocked
                    | PublicationAuthorityStatus::Blocked
            ),
            "{evidence:#?}"
        );
        assert!(
            matches!(
                evidence.credential_probe.status,
                CredentialProbeStatus::MissingCredentials
                    | CredentialProbeStatus::ToolUnavailable
                    | CredentialProbeStatus::NotRequired
            ),
            "{evidence:#?}"
        );
        assert!(
            !evidence.legal_brand_requirement.trim().is_empty(),
            "{evidence:#?}"
        );
        assert!(!evidence.published, "{evidence:#?}");
        assert!(evidence.published_evidence.is_none(), "{evidence:#?}");
    }
}

#[test]
fn test_18_4_2_published_true_requires_external_evidence_not_dry_run() {
    /* TEST-18.4.2 */
    let mut report = collect_publication_authority(&[ReleaseChannel::Cargo]);
    report.channels[0].published = true;
    report.channels[0].published_evidence = None;

    let decision = validate_publication_evidence(&report);
    assert_eq!(decision.publication_ready, PublicationReadiness::Blocked);
    assert_eq!(
        decision.invalid_published_evidence,
        vec![ReleaseChannel::Cargo],
        "{decision:#?}"
    );
    assert!(
        decision
            .blockers
            .iter()
            .any(|blocker| blocker.contains("external evidence")),
        "{decision:#?}"
    );

    report.channels[0].published_evidence = Some(PublishedEvidence {
        url: "https://crates.io/crates/promptfoo-rs/0.1.0".to_string(),
        digest: "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
            .to_string(),
    });
    let decision = validate_publication_evidence(&report);
    assert!(decision.invalid_published_evidence.is_empty(), "{decision:#?}");
    assert_eq!(decision.publication_ready, PublicationReadiness::Ready);
}

#[test]
fn test_18_4_3_missing_credentials_and_homebrew_tooling_block_publication_candidate() {
    /* TEST-18.4.3 */
    let report = collect_publication_authority(&release_channels());
    let output = PathBuf::from("target/test-publication-authority/report.json");
    write_publication_authority_report(&report, &output).expect("report should write");

    let json = std::fs::read_to_string(&output).expect("publication authority report should exist");
    let value: serde_json::Value = serde_json::from_str(&json).expect("report should be json");
    assert_eq!(value["publication_ready"], "credential-blocked");
    assert_eq!(value["credential_blocked"], true);
    assert_eq!(value["channels"][0]["published"], false);
    assert!(json.contains("credential_probe"), "{json}");
    assert!(json.contains("legal_brand_requirement"), "{json}");
    assert!(json.contains("publication-authority"), "{json}");

    let decision = validate_publication_evidence(&report);
    assert!(decision.credential_blocked, "{decision:#?}");
    assert!(
        decision
            .blockers
            .iter()
            .any(|blocker| blocker.contains("Homebrew")),
        "{decision:#?}"
    );

    let installability_script = std::fs::read_to_string("scripts/release/installability.sh")
        .expect("installability script should exist");
    let runtime_script = std::fs::read_to_string("scripts/release/runtime-smoke.sh")
        .expect("runtime smoke script should exist");
    assert!(installability_script.contains("publication-authority.json"));
    assert!(installability_script.contains("credential_probe"));
    assert!(installability_script.contains("legal_brand_requirement"));
    assert!(runtime_script.contains("publication_authority"));
    assert!(runtime_script.contains("publication-authority.json"));
}

#[test]
fn test_18_4_4_release_docs_list_remaining_publication_blockers_without_availability_claim() {
    /* TEST-18.4.4 */
    let docs = std::fs::read_to_string("docs/release.md").expect("release docs should exist");

    assert!(docs.contains("Publication Authority Gate"), "{docs}");
    assert!(docs.contains("credential-blocked"), "{docs}");
    for channel in [
        "GitHub Releases",
        "Cargo",
        "npm wrapper",
        "Docker",
        "Homebrew",
        "GitHub Action",
    ] {
        assert!(docs.contains(channel), "missing {channel}: {docs}");
    }
    assert!(docs.contains("published=false"), "{docs}");
    assert!(!docs.contains("published=true"), "{docs}");
    assert!(!docs.contains("stable public availability"), "{docs}");
}

fn release_channels() -> [ReleaseChannel; 6] {
    [
        ReleaseChannel::GitHubReleases,
        ReleaseChannel::Cargo,
        ReleaseChannel::NpmWrapper,
        ReleaseChannel::Docker,
        ReleaseChannel::Homebrew,
        ReleaseChannel::GitHubAction,
    ]
}
