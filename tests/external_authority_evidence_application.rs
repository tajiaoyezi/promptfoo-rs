use std::path::Path;

use promptfoo_rs::release::{
    apply_authority_decisions, load_authority_decision_manifest, validate_authority_decisions,
};
use serde_json::Value;

#[test]
fn test_44_1_1_tracked_manifest_has_evidence_or_waiver_for_every_decision_item() {
    /* TEST-44.1.1 */
    let packet = load_unblock_packet();
    let manifest = load_tracked_manifest();
    let report = validate_authority_decisions(&packet, &manifest);

    assert_eq!(report.unresolved_count, 0, "{report:#?}");
    assert_eq!(report.required_decision_count, 0, "{report:#?}");
    assert_eq!(
        report.ready_row_count, report.manifest_row_count,
        "{report:#?}"
    );
    assert!(report.perfect_refactor_decision_ready(), "{report:#?}");

    for row in manifest["rows"].as_array().expect("rows") {
        let state = row["decision_state"].as_str().unwrap_or_default();
        assert!(
            matches!(state, "evidence-provided" | "waived-with-boundary"),
            "{row:#?}"
        );
    }
}

#[test]
fn test_44_1_2_tracked_manifest_contains_no_secret_like_values() {
    /* TEST-44.1.2 */
    let manifest = load_tracked_manifest();
    let serialized = serde_json::to_string(&manifest).expect("manifest json");
    for forbidden in ["sk-live-", "Bearer token", "api_key=", "npm publish token"] {
        assert!(
            !serialized.contains(forbidden),
            "tracked manifest must not contain secrets: {forbidden}"
        );
    }
}

#[test]
fn test_44_1_3_unresolved_rows_remain_blocking() {
    /* TEST-44.1.3 */
    let packet = load_unblock_packet();
    let mut unresolved_manifest = load_tracked_manifest();
    unresolved_manifest["rows"][0]["decision_state"] = Value::String("unresolved".to_string());
    let report = validate_authority_decisions(&packet, &unresolved_manifest);
    assert!(!report.perfect_refactor_decision_ready(), "{report:#?}");
    assert!(report.unresolved_count >= 1, "{report:#?}");
}

#[test]
fn test_44_1_4_apply_authority_keeps_perfect_refactor_claim_false_with_other_gate_blockers() {
    /* TEST-44.1.4 */
    let manifest = load_tracked_manifest();
    let release_gates = load_release_gates();
    let application = apply_authority_decisions(&manifest, &release_gates);

    assert!(application.authority_decision_ready, "{application:#?}");
    assert!(
        !application.perfect_refactor_claim_allowed,
        "authority decisions alone must not clear aggregate perfect-refactor claim: {application:#?}"
    );
    assert!(
        !application.remaining_blockers().is_empty()
            || !release_gates["perfect_refactor_claim"]["perfect_refactor_claim_allowed"]
                .as_bool()
                .unwrap_or(false),
        "{application:#?}"
    );

    let runtime_script = std::fs::read_to_string("scripts/release/runtime-smoke.sh")
        .expect("runtime smoke script should exist");
    assert!(runtime_script.contains("authority-decisions.sh"));
}

fn load_unblock_packet() -> Value {
    serde_json::from_str(
        &std::fs::read_to_string("target/release-gates/perfect-refactor-unblock-packet.json")
            .expect("unblock packet should exist"),
    )
    .expect("unblock packet should be valid json")
}

fn load_perfect_refactor_claim() -> Value {
    serde_json::from_str(
        &std::fs::read_to_string("target/release-gates/perfect-refactor-claim.json")
            .expect("perfect refactor claim should exist"),
    )
    .expect("perfect refactor claim should parse")
}

fn load_release_gates() -> Value {
    serde_json::json!({
        "perfect_refactor_unblock_packet": load_unblock_packet(),
        "perfect_refactor_claim": load_perfect_refactor_claim(),
    })
}

fn load_tracked_manifest() -> Value {
    load_authority_decision_manifest(Path::new("docs/compatibility/authority-decisions.json"))
        .expect("tracked authority decision manifest should load")
}
