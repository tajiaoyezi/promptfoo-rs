use std::path::Path;

use promptfoo_rs::compatibility::inventory::CapabilityInventory;
use promptfoo_rs::compatibility::matrix::{
    expand_matrix_from_inventory, matrix_release_blockers, validate_no_silent_omissions,
    CapabilityMatrix, CapabilityRow, MatrixPolicy,
};

#[test]
fn test_11_3_1_every_inventory_item_has_matrix_row_metadata() {
    /* TEST-11.3.1 */
    let inventory = CapabilityInventory::from_json_file(Path::new(
        "compatibility/inventory/upstream-items.json",
    ))
    .expect("inventory json should parse");
    let matrix = CapabilityMatrix::from_json_file(Path::new("compatibility/matrix/items.json"))
        .expect("item-level matrix json should parse");

    let report = validate_no_silent_omissions(&inventory, &matrix);

    assert!(
        report.missing_matrix_rows.is_empty(),
        "missing rows: {report:#?}"
    );
    assert!(report.rows_missing_level.is_empty(), "{report:#?}");
    assert!(report.rows_missing_status.is_empty(), "{report:#?}");
    assert!(report.rows_missing_verification.is_empty(), "{report:#?}");
    assert!(report.rows_missing_owner.is_empty(), "{report:#?}");
    assert_eq!(matrix.rows.len(), inventory.items.len());
}

#[test]
fn test_11_3_2_level_specific_rules_require_fixture_snapshot_or_reason() {
    /* TEST-11.3.2 */
    let inventory = CapabilityInventory::from_json_file(Path::new(
        "compatibility/inventory/upstream-items.json",
    ))
    .expect("inventory json should parse");

    let matrix = expand_matrix_from_inventory(&inventory, &MatrixPolicy::default());
    let report = validate_no_silent_omissions(&inventory, &matrix);

    assert!(
        report.p0_rows_missing_fixture_or_blocker.is_empty(),
        "{report:#?}"
    );
    assert!(
        report.p1_rows_missing_snapshot_plan.is_empty(),
        "{report:#?}"
    );
    assert!(
        report.p2_rows_missing_reason_or_target.is_empty(),
        "{report:#?}"
    );
    assert!(matrix_release_blockers(&report).is_empty(), "{report:#?}");
}

#[test]
fn test_11_3_3_aggregate_rows_do_not_hide_item_level_omissions() {
    /* TEST-11.3.3 */
    let inventory = CapabilityInventory::from_json_file(Path::new(
        "compatibility/inventory/upstream-items.json",
    ))
    .expect("inventory json should parse");
    let mut matrix = expand_matrix_from_inventory(&inventory, &MatrixPolicy::default());
    let omitted = inventory.items[0].stable_id.clone();
    matrix.rows.retain(|row| row.capability != omitted);
    matrix.rows.push(CapabilityRow {
        capability: "Other documented providers".to_string(),
        level: "P1/P2".to_string(),
        target_status: "later".to_string(),
        verification: "aggregate placeholder".to_string(),
        owner: "compatibility".to_string(),
        notes: "aggregate rows cannot satisfy item-level coverage".to_string(),
    });

    let report = validate_no_silent_omissions(&inventory, &matrix);
    let blockers = matrix_release_blockers(&report);

    assert!(report.missing_matrix_rows.contains(&omitted), "{report:#?}");
    assert!(blockers
        .iter()
        .any(|blocker| blocker.item_id == omitted && blocker.reason.contains("silent omission")));
}
