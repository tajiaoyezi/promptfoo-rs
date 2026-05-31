use std::path::Path;

use promptfoo_rs::compatibility::fixtures::FixtureCorpus;
use promptfoo_rs::compatibility::inventory::{CapabilityInventory, InventoryItem};
use promptfoo_rs::compatibility::matrix::CapabilityMatrix;
use promptfoo_rs::redteam::registry::{
    redteam_gap_user_message, validate_redteam_parity, GapClass, RedteamInventoryCoverage,
    RedteamRegistry,
};

#[test]
fn test_14_2_1_redteam_inventory_items_have_matrix_and_registry_coverage_status() {
    /* TEST-14.2.1 */
    let inventory = load_inventory();
    let matrix = load_matrix();
    let fixtures = load_fixtures(&matrix);
    let coverage =
        RedteamInventoryCoverage::from_registry(&RedteamRegistry::core_defaults(), &inventory);
    let report = validate_redteam_parity(&coverage, &fixtures);

    assert!(
        coverage.plugin_items().len() >= 120,
        "Phase 17 source-extracted redteam plugin inventory should include long-tail rows: {coverage:#?}"
    );
    assert!(
        coverage.strategy_items().len() >= 30,
        "Phase 17 source-extracted redteam strategy inventory should include long-tail rows: {coverage:#?}"
    );
    assert_eq!(
        coverage.status_for("redteam-plugin:prompt-injection"),
        Some("native")
    );
    assert_eq!(coverage.status_for("redteam-plugin:medical"), Some("later"));
    assert_eq!(
        coverage.status_for("redteam-strategy:agentic-chain"),
        Some("later")
    );
    assert_eq!(
        coverage.status_for("redteam-plugin:src-redteam-plugins-aegis"),
        Some("later")
    );
    assert!(report.missing_matrix_rows.is_empty(), "{report:#?}");
    assert!(report.missing_registry_status.is_empty(), "{report:#?}");
    assert!(report.silent_omissions.is_empty(), "{report:#?}");
}

#[test]
fn test_14_2_2_p0_redteam_fixtures_are_mocked_release_blocking_artifacts() {
    /* TEST-14.2.2 */
    let inventory = load_inventory();
    let matrix = load_matrix();
    let fixtures = load_fixtures(&matrix);
    let coverage =
        RedteamInventoryCoverage::from_registry(&RedteamRegistry::core_defaults(), &inventory);
    let report = validate_redteam_parity(&coverage, &fixtures);

    assert!(
        report.p0_missing_fixture_or_blocker.is_empty(),
        "{report:#?}"
    );
    assert!(
        report.p0_fixtures_requiring_real_secrets.is_empty(),
        "{report:#?}"
    );
    assert!(report.unsafe_fixture_content.is_empty(), "{report:#?}");
    assert_eq!(report.p0_redteam_fixture_count, 4, "{report:#?}");

    for expected in [
        "redteam-plugin:prompt-injection",
        "redteam-plugin:harmful-content",
        "redteam-strategy:jailbreak",
        "redteam-strategy:multi-turn",
    ] {
        assert!(
            fixtures.has_p0_fixture_for(expected),
            "{expected} should have release-blocking mock redteam fixture"
        );
    }
}

#[test]
fn test_14_2_3_redteam_later_rows_have_reasons_and_user_visible_messages() {
    /* TEST-14.2.3 */
    let inventory = load_inventory();
    let matrix = load_matrix();
    let fixtures = load_fixtures(&matrix);
    let coverage =
        RedteamInventoryCoverage::from_registry(&RedteamRegistry::core_defaults(), &inventory);
    let report = validate_redteam_parity(&coverage, &fixtures);

    assert!(report.p2_or_later_missing_reason.is_empty(), "{report:#?}");
    for item_id in ["redteam-plugin:medical", "redteam-strategy:agentic-chain"] {
        let item = inventory_item(&inventory, item_id);
        let message = redteam_gap_user_message(item, GapClass::Later);
        assert!(message.contains(item_id), "{message}");
        assert!(message.contains("later"), "{message}");
        assert!(message.contains("compatibility matrix"), "{message}");
        assert!(
            coverage
                .reason_for(item_id)
                .expect("later item must have reason")
                .contains("later"),
            "{coverage:#?}"
        );
    }
}

fn load_inventory() -> CapabilityInventory {
    CapabilityInventory::from_json_file(Path::new("compatibility/inventory/upstream-items.json"))
        .expect("upstream inventory should load")
}

fn load_matrix() -> CapabilityMatrix {
    CapabilityMatrix::from_json_file(Path::new("compatibility/matrix/items.json"))
        .expect("item-level matrix should load")
}

fn load_fixtures(matrix: &CapabilityMatrix) -> FixtureCorpus {
    FixtureCorpus::load(Path::new("compatibility/fixtures"), matrix)
        .expect("fixture corpus should load")
}

fn inventory_item<'a>(inventory: &'a CapabilityInventory, item_id: &str) -> &'a InventoryItem {
    inventory
        .items
        .iter()
        .find(|item| item.stable_id == item_id)
        .expect("inventory item should exist")
}
