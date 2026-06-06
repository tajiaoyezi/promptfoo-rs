use std::path::Path;

use promptfoo_rs::release::{
    load_authority_decision_manifest, validate_authority_decisions,
    write_authority_decision_gate_report,
};
use serde_json::{json, Value};

#[test]
fn test_43_1_1_every_decision_item_has_exactly_one_manifest_row() {
    /* TEST-43.1.1 */
    let packet = load_unblock_packet();
    let manifest = load_tracked_manifest();
    let report = validate_authority_decisions(&packet, &manifest);

    assert_eq!(report.schema, "promptfoo-rs.authority-decisions-gate.v1");
    assert_eq!(
        report.required_decision_count,
        packet["decision_items"]
            .as_array()
            .map(|items| items.len())
            .unwrap_or(0),
        "{report:#?}"
    );
    assert_eq!(
        report.manifest_row_count, report.required_decision_count,
        "{report:#?}"
    );
    assert!(report.missing_manifest_rows.is_empty(), "{report:#?}");
    assert!(report.extra_manifest_rows.is_empty(), "{report:#?}");
    assert!(report.duplicate_manifest_rows.is_empty(), "{report:#?}");

    for item in packet["decision_items"]
        .as_array()
        .expect("decision_items array")
    {
        let item_id = item["item_id"].as_str().expect("item_id");
        let matches = manifest["rows"]
            .as_array()
            .expect("manifest rows")
            .iter()
            .filter(|row| row["item_id"] == item_id)
            .count();
        assert_eq!(matches, 1, "{item_id}: {manifest:#?}");
    }
}

#[test]
fn test_43_1_2_unresolved_and_mock_evidence_keep_perfect_refactor_blocked() {
    /* TEST-43.1.2 */
    let packet = load_unblock_packet();
    let manifest = load_tracked_manifest();
    let report = validate_authority_decisions(&packet, &manifest);

    assert!(!report.perfect_refactor_decision_ready(), "{report:#?}");
    assert_eq!(
        report.unresolved_count, report.required_decision_count,
        "{report:#?}"
    );
    assert_eq!(report.ready_row_count, 0, "{report:#?}");
    assert!(
        report
            .blockers
            .iter()
            .any(|blocker| blocker.contains("unresolved")),
        "{report:#?}"
    );

    let mut mock_manifest = manifest.clone();
    let mock_item_id = mock_manifest["rows"][0]["item_id"]
        .as_str()
        .expect("item_id")
        .to_string();
    mock_manifest["rows"][0] = json!({
        "item_id": mock_item_id,
        "decision_state": "evidence-provided",
        "evidence_references": [{
            "kind": "artifact-path",
            "reference": "target/mock-only/local-fixture.json"
        }]
    });
    let mock_report = validate_authority_decisions(&packet, &mock_manifest);
    assert!(
        !mock_report.perfect_refactor_decision_ready(),
        "{mock_report:#?}"
    );
    assert!(
        mock_report.mock_only_evidence_rows.contains(&mock_item_id),
        "{mock_report:#?}"
    );

    let runtime_script = std::fs::read_to_string("scripts/release/runtime-smoke.sh")
        .expect("runtime smoke script should exist");
    assert!(runtime_script.contains("authority-decisions.sh"));
    assert!(runtime_script.contains("authority_decisions"));
}

#[test]
fn test_43_1_3_waiver_rows_require_owner_date_scope_expiration_rationale_and_release_impact() {
    /* TEST-43.1.3 */
    let packet = load_unblock_packet();
    let item_id = packet["decision_items"][0]["item_id"]
        .as_str()
        .expect("item_id")
        .to_string();

    let incomplete = json!({
        "schema": "promptfoo-rs.authority-decisions.v1",
        "rows": [{
            "item_id": item_id,
            "decision_state": "waived-with-boundary",
            "waiver": {
                "owner": "maintainer",
                "approval_date": "2026-06-06"
            }
        }]
    });
    let incomplete_report = validate_authority_decisions(&packet, &incomplete);
    assert!(
        incomplete_report.invalid_waiver_rows.contains(&item_id),
        "{incomplete_report:#?}"
    );
    assert!(
        !incomplete_report.perfect_refactor_decision_ready(),
        "{incomplete_report:#?}"
    );

    let mut complete_one = load_tracked_manifest();
    let rows = complete_one["rows"].as_array_mut().expect("rows");
    rows[0] = json!({
        "item_id": item_id,
        "decision_state": "waived-with-boundary",
        "waiver": {
            "owner": "maintainer",
            "approval_date": "2026-06-06",
            "scope": "current-latest config authority only",
            "expiration_or_review_date": "2026-12-06",
            "rationale": "External service contract pending; keep frozen-baseline wording",
            "release_impact": "Blocks perfect-refactor claim for this item until review date"
        }
    });
    let partial_report = validate_authority_decisions(&packet, &complete_one);
    assert!(
        partial_report.invalid_waiver_rows.is_empty()
            || !partial_report.invalid_waiver_rows.contains(&item_id),
        "{partial_report:#?}"
    );
    assert!(
        !partial_report.perfect_refactor_decision_ready(),
        "one waived row cannot clear aggregate readiness while others remain unresolved: {partial_report:#?}"
    );
}

#[test]
fn test_43_1_4_manifest_stores_no_real_secrets() {
    /* TEST-43.1.4 */
    let packet = load_unblock_packet();
    let manifest = load_tracked_manifest();
    let report = validate_authority_decisions(&packet, &manifest);

    assert!(report.secret_bearing_rows.is_empty(), "{report:#?}");

    let mut secret_manifest = manifest.clone();
    let secret_item_id = secret_manifest["rows"][0]["item_id"]
        .as_str()
        .expect("item_id")
        .to_string();
    secret_manifest["rows"][0] = json!({
        "item_id": secret_item_id,
        "decision_state": "evidence-provided",
        "evidence_references": [{
            "kind": "approval-id",
            "reference": "sk-live-secret-token-123"
        }]
    });
    let secret_report = validate_authority_decisions(&packet, &secret_manifest);
    assert!(
        !secret_report.secret_bearing_rows.is_empty(),
        "{secret_report:#?}"
    );
    assert!(
        !secret_report.perfect_refactor_decision_ready(),
        "{secret_report:#?}"
    );

    let serialized = serde_json::to_string(&manifest).expect("manifest json");
    for forbidden in ["sk-live-secret", "Bearer token", "api_key="] {
        assert!(
            !serialized.contains(forbidden),
            "tracked manifest must not contain secrets: {forbidden}"
        );
    }

    let output = std::env::temp_dir().join(format!(
        "promptfoo-rs-authority-decisions-gate-{}.json",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&output);
    write_authority_decision_gate_report(&report, Path::new(&output))
        .expect("gate report should write");
    let gate_json: Value = serde_json::from_str(
        &std::fs::read_to_string(&output).expect("gate report should be readable"),
    )
    .expect("gate report should be valid json");
    let _ = std::fs::remove_file(&output);
    assert_eq!(gate_json["status"], "blocked");
    assert_eq!(gate_json["perfect_refactor_decision_ready"], false);
}

fn load_unblock_packet() -> Value {
    serde_json::from_str(
        &std::fs::read_to_string("target/release-gates/perfect-refactor-unblock-packet.json")
            .expect("unblock packet should exist"),
    )
    .expect("unblock packet should be valid json")
}

fn load_tracked_manifest() -> Value {
    load_authority_decision_manifest(Path::new("docs/compatibility/authority-decisions.json"))
        .expect("tracked authority decision manifest should load")
}
