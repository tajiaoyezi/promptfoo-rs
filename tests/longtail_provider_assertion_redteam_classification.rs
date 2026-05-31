use std::path::Path;

use promptfoo_rs::compatibility::fixtures::FixtureCorpus;
use promptfoo_rs::compatibility::inventory::{CapabilityInventory, InventoryItem};
use promptfoo_rs::compatibility::matrix::CapabilityMatrix;
use promptfoo_rs::compatibility::provider_assertion::{
    classify_assertion_item, classify_provider_item, classify_redteam_item,
    compatibility_gap_error, validate_longtail_classification, GapClass, LongtailClass,
    ParityPolicy,
};

#[test]
fn test_17_4_1_source_extracted_longtail_rows_are_classified_without_unresolved_gaps() {
    /* TEST-17.4.1 */
    let inventory = load_inventory();
    let matrix = load_matrix();
    let fixtures = load_fixtures(&matrix);
    let report = validate_longtail_classification(&matrix, &fixtures);

    assert!(
        longtail_count(&inventory, "provider") >= 200,
        "provider long-tail source rows must be tracked in inventory"
    );
    assert!(
        longtail_count(&inventory, "assertion") >= 50,
        "assertion long-tail source rows must be tracked in inventory"
    );
    assert!(
        longtail_count(&inventory, "redteam-plugin") >= 120,
        "redteam plugin source rows must be tracked in inventory"
    );
    assert!(
        longtail_count(&inventory, "redteam-strategy") >= 30,
        "redteam strategy source rows must be tracked in inventory"
    );

    assert!(report.missing_classification.is_empty(), "{report:#?}");
    assert!(report.rows_missing_owner.is_empty(), "{report:#?}");
    assert!(report.rows_missing_verification.is_empty(), "{report:#?}");
    assert!(report.unresolved_rows.is_empty(), "{report:#?}");
    assert!(report.rows_missing_reason.is_empty(), "{report:#?}");
}

#[test]
fn test_17_4_2_p0_p1_p2_rows_have_fixture_snapshot_or_blocker_evidence() {
    /* TEST-17.4.2 */
    let matrix = load_matrix();
    let fixtures = load_fixtures(&matrix);
    let report = validate_longtail_classification(&matrix, &fixtures);

    assert!(
        report.p0_missing_fixture_or_blocker.is_empty(),
        "{report:#?}"
    );
    assert!(report.p1_missing_snapshot_plan.is_empty(), "{report:#?}");
    assert!(report.p2_or_later_missing_reason.is_empty(), "{report:#?}");
    assert!(
        report.p0_release_blocker_count > 0,
        "source-extracted P0 provider modules without per-file fixtures must stay release-blocking"
    );
}

#[test]
fn test_17_4_3_script_backed_rows_remain_default_deny_and_redacted() {
    /* TEST-17.4.3 */
    let inventory = load_inventory();
    let matrix = load_matrix();
    let fixtures = load_fixtures(&matrix);
    let report = validate_longtail_classification(&matrix, &fixtures);
    let policy = ParityPolicy::default();

    for runtime in ["javascript", "typescript", "python", "shell", "ruby"] {
        let boundary = report
            .script_boundary_for(runtime)
            .unwrap_or_else(|| panic!("missing script boundary for {runtime}"));
        assert!(boundary.default_deny, "{boundary:#?}");
        assert!(boundary.explicit_allow_required, "{boundary:#?}");
        assert!(boundary.timeout_required, "{boundary:#?}");
        assert!(boundary.env_allowlist_required, "{boundary:#?}");
        assert!(boundary.redaction_required, "{boundary:#?}");
    }
    assert!(report.script_boundary_gaps.is_empty(), "{report:#?}");

    let javascript = inventory_item(&inventory, "assertion:javascript");
    let classification = classify_assertion_item(javascript, &policy);
    assert_eq!(classification.class, LongtailClass::Bridge);
    assert!(classification.reason.contains("script bridge"));

    let error = compatibility_gap_error(
        "assertion:javascript",
        GapClass::Blocked,
        "script bridge denied API_KEY=sk-test TOKEN=secret by default",
    );
    let message = error.to_string();
    assert_eq!(error.exit_code(), 1);
    assert!(message.contains("assertion:javascript"), "{message}");
    assert!(message.contains("blocked"), "{message}");
    assert!(
        message.contains("docs/compatibility/matrix.md"),
        "{message}"
    );
    assert!(!message.contains("sk-test"), "{message}");
    assert!(!message.contains("secret"), "{message}");
}

#[test]
fn test_17_4_4_gap_errors_include_item_class_reason_and_docs_link() {
    /* TEST-17.4.4 */
    let inventory = load_inventory();
    let policy = ParityPolicy::default();

    let provider = first_longtail_item(&inventory, "provider");
    let provider_classification = classify_provider_item(provider, &policy);
    assert!(matches!(
        provider_classification.class,
        LongtailClass::Later | LongtailClass::Blocked | LongtailClass::Unsupported
    ));

    let redteam = first_longtail_item(&inventory, "redteam-plugin");
    let redteam_classification = classify_redteam_item(redteam, &policy);
    assert!(matches!(
        redteam_classification.class,
        LongtailClass::Later | LongtailClass::Blocked | LongtailClass::Unsupported
    ));

    for (item_id, class, reason) in [
        (
            "provider:src-providers-ai21",
            GapClass::Later,
            "requires provider-specific compatibility work",
        ),
        (
            "provider:cloud-upload",
            GapClass::Unsupported,
            "cloud upload is not local-only",
        ),
        (
            "redteam-plugin:src-redteam-plugins-aegis",
            GapClass::Blocked,
            "requires policy review",
        ),
    ] {
        let error = compatibility_gap_error(item_id, class, reason);
        let message = error.to_string();
        assert_eq!(error.exit_code(), 1);
        assert!(message.contains(item_id), "{message}");
        assert!(message.contains(class.as_str()), "{message}");
        assert!(message.contains("reason:"), "{message}");
        assert!(
            message.contains("docs/compatibility/matrix.md"),
            "{message}"
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

fn longtail_count(inventory: &CapabilityInventory, category: &str) -> usize {
    inventory
        .items
        .iter()
        .filter(|item| {
            item.category == category && item.source_reference.starts_with("promptfoo@0.121.13:")
        })
        .count()
}

fn inventory_item<'a>(inventory: &'a CapabilityInventory, item_id: &str) -> &'a InventoryItem {
    inventory
        .items
        .iter()
        .find(|item| item.stable_id == item_id)
        .unwrap_or_else(|| panic!("inventory item {item_id} should exist"))
}

fn first_longtail_item<'a>(
    inventory: &'a CapabilityInventory,
    category: &str,
) -> &'a InventoryItem {
    inventory
        .items
        .iter()
        .find(|item| {
            item.category == category && item.source_reference.starts_with("promptfoo@0.121.13:")
        })
        .unwrap_or_else(|| panic!("long-tail item for {category} should exist"))
}
