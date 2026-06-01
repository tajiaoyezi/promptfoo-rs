use std::path::Path;

use promptfoo_rs::release::{
    build_current_latest_quality_report, build_perfect_refactor_claim_contract,
    evaluate_current_latest_claim, write_current_latest_quality_report, CurrentLatestQualityInputs,
    PerfectRefactorClaimInputs, PublicationReadiness,
};
use serde_json::Value;

#[test]
fn test_24_4_1_quality_report_aggregates_current_latest_release_gates() {
    /* TEST-24.4.1 */
    let report = build_current_latest_quality_report(blocked_inputs());

    assert_eq!(report.schema, "promptfoo-rs.current-latest-quality.v1");
    for key in [
        "adapter",
        "source_inventory",
        "current_latest_matrix",
        "golden_corpus",
        "regression",
        "stress",
        "property",
        "runtime_smoke",
        "current_target",
        "external_authority",
        "publication_authority",
    ] {
        assert!(
            report.gate_statuses.contains_key(key),
            "missing gate status {key}: {report:#?}"
        );
    }

    for artifact in [
        "current-latest-target.json",
        "current-latest-source-inventory.json",
        "current-latest-matrix.json",
        "current-latest-golden-corpus.json",
        "release-candidate.json",
        "perfect-refactor-claim.json",
    ] {
        assert!(
            report
                .source_artifacts
                .iter()
                .any(|path| path.ends_with(artifact)),
            "missing source artifact {artifact}: {report:#?}"
        );
    }

    assert!(
        report
            .blockers
            .iter()
            .any(|blocker| blocker.category == "golden-corpus"),
        "{report:#?}"
    );

    let path = std::env::temp_dir().join(format!(
        "promptfoo-rs-current-latest-quality-{}.json",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&path);
    write_current_latest_quality_report(&report, Path::new(&path))
        .expect("quality report should write");
    let json: Value = serde_json::from_str(
        &std::fs::read_to_string(&path).expect("quality report should be readable"),
    )
    .expect("quality report should be valid json");
    let _ = std::fs::remove_file(&path);
    assert_eq!(
        json["schema"],
        Value::String("promptfoo-rs.current-latest-quality.v1".to_string())
    );
    assert!(json["blockers"].is_array());
}

#[test]
fn test_24_4_2_claim_wording_rejects_impossible_bug_promises() {
    /* TEST-24.4.2 */
    let allowed =
        build_current_latest_quality_report(ready_inputs("no known release-blocking defects under declared gates"));
    assert!(
        !allowed
            .blockers
            .iter()
            .any(|blocker| blocker.category == "claim-wording"),
        "{allowed:#?}"
    );
    assert_eq!(
        allowed.allowed_claim_wording,
        "no known release-blocking defects under declared gates"
    );

    for wording in [
        "no potential bugs",
        "zero possible bugs",
        "bug-free complete rewrite",
    ] {
        let report = build_current_latest_quality_report(ready_inputs(wording));
        assert!(
            report
                .blockers
                .iter()
                .any(|blocker| blocker.category == "claim-wording"),
            "{wording}: {report:#?}"
        );
        assert!(
            !report.perfect_refactor_claim_allowed,
            "{wording}: {report:#?}"
        );
    }
}

#[test]
fn test_24_4_3_perfect_refactor_claim_stays_false_when_any_evidence_is_missing() {
    /* TEST-24.4.3 */
    let report = build_current_latest_quality_report(blocked_inputs());
    let claim = build_perfect_refactor_claim_contract(PerfectRefactorClaimInputs {
        local_stable_allowed: true,
        published: false,
        source_p0_accounting_blocker_count: 0,
        current_perfect_claim_allowed: false,
        publication_ready: PublicationReadiness::CredentialBlocked,
        external_authority_status: "blocked".to_string(),
        external_authority_blocker_count: 8,
        source_artifacts: source_artifacts(),
    });
    let decision = evaluate_current_latest_claim(&report, &claim);

    assert!(decision.is_err(), "{report:#?}");
    assert!(!report.perfect_refactor_claim_allowed, "{report:#?}");
    for category in [
        "source-inventory",
        "current-latest-matrix",
        "golden-corpus",
        "current-target",
        "external-authority",
        "publication-authority",
    ] {
        assert!(
            report
                .blockers
                .iter()
                .any(|blocker| blocker.category == category),
            "missing blocker category {category}: {report:#?}"
        );
    }
}

#[test]
fn test_24_4_4_local_readiness_can_pass_while_public_perfect_claim_is_blocked() {
    /* TEST-24.4.4 */
    let report = build_current_latest_quality_report(local_ready_external_blocked_inputs());

    assert!(report.local_current_latest_ready, "{report:#?}");
    assert!(!report.perfect_refactor_claim_allowed, "{report:#?}");
    assert!(report
        .blockers
        .iter()
        .any(|blocker| blocker.category == "external-authority"));
    assert!(report
        .blockers
        .iter()
        .any(|blocker| blocker.category == "publication-authority"));
    for local_category in ["regression", "stress", "property", "golden-corpus"] {
        assert!(
            !report
                .blockers
                .iter()
                .any(|blocker| blocker.category == local_category),
            "{local_category}: {report:#?}"
        );
    }

    let runtime_script =
        std::fs::read_to_string("scripts/release/runtime-smoke.sh").expect("runtime smoke exists");
    assert!(runtime_script.contains("current-latest-quality-gate.sh"));
    assert!(runtime_script.contains("current_latest_quality"));
    let integration =
        std::fs::read_to_string("scripts/release/integration.sh").expect("integration exists");
    assert!(integration.contains("current_latest_quality_gate"));
    let coverage = std::fs::read_to_string("scripts/release/coverage.sh").expect("coverage exists");
    assert!(coverage.contains("TEST-24.4.4"));
}

fn blocked_inputs() -> CurrentLatestQualityInputs {
    let mut inputs = ready_inputs("no known release-blocking defects under declared gates");
    inputs.source_inventory_status = "ready-with-blockers".to_string();
    inputs.source_inventory_unclassified_count = 318;
    inputs.matrix_status = "ready-with-blockers".to_string();
    inputs.matrix_unclassified_count = 318;
    inputs.golden_corpus_status = "ready-with-blockers".to_string();
    inputs.golden_corpus_blocker_count = 432;
    inputs.stress_status = "blocked".to_string();
    inputs.current_target_claim_allowed = false;
    inputs.external_authority_status = "blocked".to_string();
    inputs.external_authority_blocker_count = 8;
    inputs.publication_ready = PublicationReadiness::CredentialBlocked;
    inputs
}

fn local_ready_external_blocked_inputs() -> CurrentLatestQualityInputs {
    let mut inputs = ready_inputs("no known release-blocking defects under declared gates");
    inputs.external_authority_status = "blocked".to_string();
    inputs.external_authority_blocker_count = 8;
    inputs.publication_ready = PublicationReadiness::CredentialBlocked;
    inputs
}

fn ready_inputs(wording: &str) -> CurrentLatestQualityInputs {
    CurrentLatestQualityInputs {
        adapter_verification_status: "ready".to_string(),
        source_inventory_status: "ready".to_string(),
        source_inventory_unclassified_count: 0,
        matrix_status: "ready".to_string(),
        matrix_unclassified_count: 0,
        golden_corpus_status: "ready".to_string(),
        golden_corpus_blocker_count: 0,
        regression_status: "ready".to_string(),
        stress_status: "ready".to_string(),
        property_status: "ready".to_string(),
        runtime_smoke_status: "ready".to_string(),
        external_authority_status: "ready".to_string(),
        external_authority_blocker_count: 0,
        publication_ready: PublicationReadiness::Ready,
        current_target_status: "ready".to_string(),
        current_target_claim_allowed: true,
        local_stable_allowed: true,
        requested_claim_wording: wording.to_string(),
        source_artifacts: source_artifacts(),
    }
}

fn source_artifacts() -> Vec<String> {
    vec![
        "target/release-gates/current-latest-target.json".to_string(),
        "target/release-gates/current-latest-source-inventory.json".to_string(),
        "target/release-gates/current-latest-matrix.json".to_string(),
        "target/release-gates/current-latest-golden-corpus.json".to_string(),
        "target/release-gates/release-candidate.json".to_string(),
        "target/release-gates/perfect-refactor-claim.json".to_string(),
        "target/release-gates/external-authority-blockers.json".to_string(),
        "target/release-gates/publication-authority.json".to_string(),
    ]
}
