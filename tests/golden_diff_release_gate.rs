use promptfoo_rs::compatibility::diff::{
    classify_diff, DiffClass, DiffFinding,
};
use promptfoo_rs::compatibility::matrix::{CapabilityMatrix, CapabilityRow};
use promptfoo_rs::compatibility::normalize::NormalizedArtifact;
use promptfoo_rs::compatibility::release_gate::{
    evaluate_release_gate, ReleaseGateStatus,
};
use serde_json::json;

fn normalized(payload: serde_json::Value) -> NormalizedArtifact {
    NormalizedArtifact {
        fixture_name: "release-gate".to_string(),
        payload,
        applied_rules: vec!["timestamp".to_string(), "latency".to_string()],
    }
}

fn matrix_row(capability: &str, level: &str) -> CapabilityRow {
    CapabilityRow {
        capability: capability.to_string(),
        level: level.to_string(),
        target_status: "native".to_string(),
        verification: "golden diff".to_string(),
        owner: "leafiellune".to_string(),
        notes: "registered by TEST-6.2".to_string(),
    }
}

#[test]
fn test_6_2_1_diff_classification_covers_all_release_gate_classes() {
    let cases = vec![
        (
            normalized(json!({ "value": "same" })),
            normalized(json!({ "value": "same" })),
            DiffClass::Matching,
        ),
        (
            normalized(json!({ "value": "upstream spelling" })),
            normalized(json!({
                "value": "rust spelling",
                "compatibility": {
                    "classification": "intentional-difference",
                    "reason": "documented Rust-native message wording"
                }
            })),
            DiffClass::IntentionalDifference,
        ),
        (
            normalized(json!({ "cloud": "upload" })),
            normalized(json!({
                "cloud": "disabled",
                "compatibility": {
                    "classification": "unsupported",
                    "reason": "cloud/share is P2 no-upload"
                }
            })),
            DiffClass::Unsupported,
        ),
        (
            normalized(json!({ "provider": "long-tail" })),
            normalized(json!({
                "provider": "deferred",
                "compatibility": {
                    "classification": "later",
                    "reason": "P2 provider inventory deferred"
                }
            })),
            DiffClass::Later,
        ),
        (
            normalized(json!({ "warning": "ambiguous upstream docs" })),
            normalized(json!({
                "warning": "ambiguous upstream docs",
                "compatibility": {
                    "classification": "upstream-ambiguous",
                    "reason": "promptfoo 0.121.13 docs omit this edge"
                }
            })),
            DiffClass::UpstreamAmbiguous,
        ),
        (
            normalized(json!({ "summary": { "failed": 0 } })),
            normalized(json!({ "summary": { "failed": 1 } })),
            DiffClass::Bug,
        ),
    ];

    let classes: Vec<DiffClass> = cases
        .into_iter()
        .map(|(upstream, rs, expected)| {
            let findings = classify_diff(&upstream, &rs);
            assert_eq!(findings.len(), 1);
            assert_eq!(findings[0].class, expected);
            findings[0].class
        })
        .collect();

    assert_eq!(
        classes,
        vec![
            DiffClass::Matching,
            DiffClass::IntentionalDifference,
            DiffClass::Unsupported,
            DiffClass::Later,
            DiffClass::UpstreamAmbiguous,
            DiffClass::Bug,
        ]
    );
}

#[test]
fn test_6_2_2_p0_bug_and_unclassified_diff_block_stable_release() {
    let matrix = CapabilityMatrix {
        rows: vec![
            matrix_row("Eval runner", "P0"),
            matrix_row("Model-graded assertions", "P1"),
        ],
    };
    let findings = vec![
        DiffFinding::bug("Eval runner", "$.summary.failed", "failed count drift"),
        DiffFinding::unclassified(
            "Eval runner",
            "$.results[0].metadata",
            "metadata drift lacks classification",
        ),
        DiffFinding::intentional_difference(
            "Model-graded assertions",
            "$.score",
            "recorded grader fixture is non-deterministic",
        ),
    ];

    let summary = evaluate_release_gate(&matrix, &findings);

    assert_eq!(summary.status, ReleaseGateStatus::Blocked);
    assert_eq!(summary.blocking_findings.len(), 2);
    assert!(summary
        .blocking_findings
        .iter()
        .any(|finding| finding.message.contains("failed count drift")));
    assert!(summary
        .blocking_findings
        .iter()
        .any(|finding| finding.class == DiffClass::Unclassified));
}

#[test]
fn test_6_2_3_p1_snapshots_and_p2_registration_enter_gate_summary() {
    let matrix = CapabilityMatrix {
        rows: vec![
            matrix_row("Model-graded assertions", "P1"),
            matrix_row("promptfoo cloud/share", "P2"),
            matrix_row("Eval runner", "P0"),
        ],
    };
    let findings = vec![
        DiffFinding::intentional_difference(
            "Model-graded assertions",
            "$.score",
            "P1 snapshot documented",
        ),
        DiffFinding::unsupported(
            "promptfoo cloud/share",
            "$.upload",
            "P2 no-upload registration documented",
        ),
    ];

    let summary = evaluate_release_gate(&matrix, &findings);

    assert_eq!(summary.status, ReleaseGateStatus::Ready);
    assert_eq!(summary.p1_snapshot_total, 1);
    assert_eq!(summary.p1_snapshot_covered, 1);
    assert_eq!(summary.p2_registration_total, 1);
    assert_eq!(summary.p2_registered, 1);
    assert!(summary
        .notes
        .contains(&"P1 snapshot coverage: 1/1".to_string()));
    assert!(summary
        .notes
        .contains(&"P2 registration coverage: 1/1".to_string()));
}
