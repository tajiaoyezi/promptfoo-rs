use std::path::PathBuf;

use promptfoo_rs::compatibility::provider_assertion::{
    collect_external_authority_blockers, validate_external_authority_gate,
    write_external_authority_blockers, ExternalAuthorityStatus, ExternalAuthorityType,
};

#[test]
fn test_19_4_1_external_blockers_have_item_level_authority_decisions() {
    /* TEST-19.4.1 */
    let report = collect_external_authority_blockers();

    assert_eq!(report.schema, "promptfoo-rs.external-authority-blockers.v1");
    assert_eq!(report.provider_external_blocker_count, 15, "{report:#?}");
    assert!(
        report.publication_blocker_count >= 6,
        "publication blockers should be linked: {report:#?}"
    );
    assert_eq!(report.blocker_count, report.blockers.len(), "{report:#?}");

    for blocker in &report.blockers {
        assert!(!blocker.item_id.trim().is_empty(), "{blocker:#?}");
        assert!(!blocker.source_reference.trim().is_empty(), "{blocker:#?}");
        assert!(!blocker.required_decision.trim().is_empty(), "{blocker:#?}");
        assert!(
            !blocker.safe_local_fallback.trim().is_empty(),
            "{blocker:#?}"
        );
        assert!(!blocker.release_impact.trim().is_empty(), "{blocker:#?}");
        assert_ne!(blocker.current_status, ExternalAuthorityStatus::Ready);
    }

    assert!(report.blockers.iter().any(|blocker| {
        blocker.item_id.contains("codex")
            && blocker.authority_type == ExternalAuthorityType::ProductAuthority
    }));
    assert!(report.blockers.iter().any(|blocker| {
        blocker.item_id.contains("billing")
            && blocker.authority_type == ExternalAuthorityType::Account
    }));
}

#[test]
fn test_19_4_2_local_fallback_never_sets_external_authority_ready() {
    /* TEST-19.4.2 */
    let report = collect_external_authority_blockers();
    let decision = validate_external_authority_gate(&report);

    assert!(!decision.ready, "{decision:#?}");
    assert_eq!(decision.ready_count, 0, "{decision:#?}");
    assert!(
        decision.invalid_ready_items.is_empty(),
        "ready items are invalid while external proof is absent: {decision:#?}"
    );
    assert!(
        report
            .blockers
            .iter()
            .all(|blocker| !blocker.safe_local_fallback.contains("live parity")),
        "{report:#?}"
    );
    assert!(
        report
            .blockers
            .iter()
            .any(|blocker| blocker.current_status == ExternalAuthorityStatus::WaivedWithBoundary),
        "safe local fallbacks should be waiver-with-boundary records: {report:#?}"
    );
}

#[test]
fn test_19_4_3_publication_authority_blockers_are_linked() {
    /* TEST-19.4.3 */
    let report = collect_external_authority_blockers();
    let publication = report
        .blockers
        .iter()
        .filter(|blocker| blocker.item_id.starts_with("publication:"))
        .collect::<Vec<_>>();

    assert!(
        publication.len() >= 6,
        "every publication channel should stay visible: {report:#?}"
    );
    for blocker in publication {
        assert_eq!(
            blocker.authority_type,
            ExternalAuthorityType::PublicationAuthority,
            "{blocker:#?}"
        );
        assert!(
            blocker
                .source_reference
                .contains("publication-authority.json"),
            "{blocker:#?}"
        );
        assert!(
            blocker.required_decision.contains("credentials")
                && blocker.required_decision.contains("authority"),
            "{blocker:#?}"
        );
        assert!(blocker.release_impact.contains("published=false"));
    }

    assert!(report
        .source_artifacts
        .iter()
        .any(|artifact| artifact == "target/release-gates/publication-authority.json"));
}

#[test]
fn test_19_4_4_release_candidate_and_docs_keep_external_blockers_visible() {
    /* TEST-19.4.4 */
    let report = collect_external_authority_blockers();
    let output = PathBuf::from("target/test-external-authority-blockers/report.json");
    write_external_authority_blockers(&report, &output).expect("external authority report writes");

    let json = std::fs::read_to_string(&output).expect("report should exist");
    let value: serde_json::Value = serde_json::from_str(&json).expect("report should be json");
    assert_eq!(value["status"], "blocked");
    assert_eq!(value["ready_count"], 0);
    assert!(json.contains("external-authority-blockers"), "{json}");
    assert!(json.contains("required_decision"), "{json}");
    assert!(json.contains("safe_local_fallback"), "{json}");

    let runtime_script = std::fs::read_to_string("scripts/release/runtime-smoke.sh")
        .expect("runtime smoke script should exist");
    assert!(runtime_script.contains("external-authority-blockers.json"));
    assert!(runtime_script.contains("external_authority"));

    for docs_path in [
        "docs/compatibility/matrix.md",
        "docs/release.md",
        "docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md",
    ] {
        let docs = std::fs::read_to_string(docs_path).expect("docs should exist");
        assert!(docs.contains("Task 19.4"), "{docs_path}: {docs}");
        assert!(
            docs.contains("external-authority-blockers.json"),
            "{docs_path}: {docs}"
        );
        assert!(
            !docs.contains("perfect refactor complete"),
            "{docs_path}: {docs}"
        );
    }
}
