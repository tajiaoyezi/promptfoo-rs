use std::path::Path;
use std::process::Command;

use promptfoo_rs::release::{
    build_current_latest_quality_report, CurrentLatestQualityInputs, PublicationReadiness,
};
use serde_json::Value;

#[test]
fn test_50_1_1_frozen_baseline_golden_corpus_splits_active_and_audit_blockers() {
    /* TEST-50.1.1 */
    let status = Command::new("bash")
        .args(["scripts/release/current-latest-golden-corpus.sh"])
        .status()
        .expect("golden corpus script should run");
    assert!(status.success(), "golden corpus script should succeed");

    let corpus_path = Path::new("target/release-gates/current-latest-golden-corpus.json");
    let corpus: Value = serde_json::from_str(
        &std::fs::read_to_string(corpus_path).expect("golden corpus readable"),
    )
    .expect("golden corpus json");
    assert_eq!(
        corpus["blocker_count"].as_u64(),
        Some(24),
        "audit blocker_count should remain 24: {corpus:#?}"
    );
    assert_eq!(
        corpus["active_blocker_count"].as_u64(),
        Some(0),
        "active_blocker_count should be zero after waiver alignment: {corpus:#?}"
    );
    assert_eq!(
        corpus["waived_blocker_count"].as_u64(),
        Some(24),
        "waived_blocker_count should be 24: {corpus:#?}"
    );
    assert!(
        corpus["active_blockers"]
            .as_array()
            .is_some_and(|rows| rows.is_empty()),
        "active_blockers should be empty when all audit blockers are waived: {corpus:#?}"
    );
}

#[test]
fn test_50_1_3_quality_gate_uses_golden_active_blocker_count() {
    /* TEST-50.1.3 */
    let report = build_current_latest_quality_report(CurrentLatestQualityInputs {
        adapter_verification_status: "ready".to_string(),
        source_inventory_status: "ready".to_string(),
        source_inventory_unclassified_count: 0,
        matrix_status: "ready".to_string(),
        matrix_unclassified_count: 0,
        golden_corpus_status: "ready".to_string(),
        golden_corpus_blocker_count: 24,
        golden_corpus_active_blocker_count: Some(0),
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
        requested_claim_wording: "no known release-blocking defects under declared gates"
            .to_string(),
        source_artifacts: vec!["target/release-gates/current-latest-golden-corpus.json".to_string()],
    });

    assert!(
        !report
            .blockers
            .iter()
            .any(|blocker| blocker.category == "golden-corpus"),
        "quality gate should not block on audit-only golden blockers: {report:#?}"
    );
    assert!(report.local_current_latest_ready, "{report:#?}");
}

#[test]
fn test_50_1_4_golden_corpus_script_emits_active_blocker_fields() {
    /* TEST-50.1.4 */
    let status = Command::new("bash")
        .args(["scripts/release/current-latest-golden-corpus.sh"])
        .status()
        .expect("golden corpus script should run");
    assert!(status.success(), "golden corpus script should succeed");

    let corpus: Value = serde_json::from_str(
        &std::fs::read_to_string("target/release-gates/current-latest-golden-corpus.json")
            .expect("golden corpus readable"),
    )
    .expect("golden corpus json");
    assert!(corpus.get("active_blocker_count").is_some(), "{corpus:#?}");
    assert!(corpus.get("waived_blocker_count").is_some(), "{corpus:#?}");
    assert!(corpus["active_blockers"].is_array(), "{corpus:#?}");
    assert!(corpus["waived_blockers"].is_array(), "{corpus:#?}");
}
