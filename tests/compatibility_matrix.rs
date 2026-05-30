use std::path::Path;

use promptfoo_rs::compatibility::matrix::{
    validate_matrix_completeness, CapabilityMatrix,
};

#[test]
fn test_1_2_1_matrix_covers_required_capability_domains() {
    let matrix = CapabilityMatrix::from_markdown(Path::new("docs/compatibility/matrix.md"))
        .expect("TEST-1.2.1 matrix should parse");
    let report = validate_matrix_completeness(&matrix);

    assert!(report.missing_domains.is_empty(), "{report:#?}");
    for required in [
        "CLI",
        "config",
        "provider",
        "assertion",
        "redteam",
        "MCP",
        "scan",
        "output",
        "Node API",
        "cloud/share",
    ] {
        assert!(
            matrix.covers_domain(required),
            "expected matrix to cover {required}"
        );
    }
}

#[test]
fn test_1_2_2_each_capability_has_level_status_verification_and_owner() {
    let matrix = CapabilityMatrix::from_markdown(Path::new("docs/compatibility/matrix.md"))
        .expect("TEST-1.2.2 matrix should parse");
    let report = validate_matrix_completeness(&matrix);

    assert!(report.rows_missing_level.is_empty(), "{report:#?}");
    assert!(report.rows_missing_target_status.is_empty(), "{report:#?}");
    assert!(report.rows_missing_verification.is_empty(), "{report:#?}");
    assert!(report.rows_missing_owner.is_empty(), "{report:#?}");
}

#[test]
fn test_1_2_3_p2_known_gaps_have_reason() {
    let matrix = CapabilityMatrix::from_markdown(Path::new("docs/compatibility/matrix.md"))
        .expect("TEST-1.2.3 matrix should parse");
    let report = validate_matrix_completeness(&matrix);

    assert!(report.p2_rows_missing_reason.is_empty(), "{report:#?}");
}
