use std::path::Path;

use promptfoo_rs::compatibility::inventory::CapabilityInventory;
use promptfoo_rs::compatibility::matrix::CapabilityMatrix;
use promptfoo_rs::compatibility::provider_assertion::{
    validate_provider_assertion_parity, AssertionParityRegistry, FixtureCorpus,
    ProviderParityRegistry,
};

#[test]
fn test_14_1_1_provider_and_assertion_inventory_rows_have_classified_matrix_evidence() {
    /* TEST-14.1.1 */
    let inventory = load_inventory();
    let matrix = load_matrix();
    let fixtures = load_fixtures(&matrix);

    let providers = ProviderParityRegistry::from_inventory(&inventory);
    let assertions = AssertionParityRegistry::from_inventory(&inventory);
    let report = validate_provider_assertion_parity(&matrix, &fixtures);

    assert!(
        providers.items().len() >= 200,
        "Phase 17 source-extracted provider inventory should include long-tail rows: {providers:#?}"
    );
    assert!(
        assertions.items().len() >= 50,
        "Phase 17 source-extracted assertion inventory should include long-tail rows: {assertions:#?}"
    );
    for expected in [
        "provider:openai",
        "provider:http",
        "provider:ollama",
        "provider:anthropic",
    ] {
        assert!(providers.p0_item_ids().iter().any(|id| id == expected));
    }
    for expected in [
        "assertion:equals",
        "assertion:contains",
        "assertion:regex",
        "assertion:json",
        "assertion:javascript",
        "assertion:python",
    ] {
        assert!(assertions.p0_item_ids().iter().any(|id| id == expected));
    }

    assert!(report.provider_matrix_gaps.is_empty(), "{report:#?}");
    assert!(report.assertion_matrix_gaps.is_empty(), "{report:#?}");
    assert!(
        report.provider_rows_missing_metadata.is_empty(),
        "{report:#?}"
    );
    assert!(
        report.assertion_rows_missing_metadata.is_empty(),
        "{report:#?}"
    );
    assert!(report.p2_rows_missing_reason.is_empty(), "{report:#?}");

    let dynamic = providers
        .item("provider:dynamic-registry")
        .expect("P2 dynamic provider registry must stay visible");
    assert_eq!(dynamic.level, "P2");
    assert!(dynamic
        .gap_reason()
        .expect("P2 provider needs gap reason")
        .contains("dynamic provider registry"));
    let longtail = providers
        .item("provider:src-providers-ai21")
        .expect("source-extracted long-tail provider should stay visible");
    assert_eq!(longtail.level, "P2");
    assert!(longtail
        .gap_reason()
        .expect("long-tail provider needs explicit later reason")
        .contains("source-extracted long-tail provider"));
}

#[test]
fn test_14_1_2_p0_provider_and_assertion_fixtures_cover_release_blocking_rows() {
    /* TEST-14.1.2 */
    let matrix = load_matrix();
    let fixtures = load_fixtures(&matrix);
    let report = validate_provider_assertion_parity(&matrix, &fixtures);

    assert!(
        report.p0_missing_fixture_or_blocker.is_empty(),
        "{report:#?}"
    );
    assert!(
        report.p0_fixtures_requiring_real_secrets.is_empty(),
        "{report:#?}"
    );
    assert!(report.unclassified_p0_blockers.is_empty(), "{report:#?}");
    assert!(report.p0_provider_fixture_count >= 4, "{report:#?}");
    assert!(report.p0_assertion_fixture_count >= 6, "{report:#?}");

    for expected in [
        "provider:openai",
        "provider:http",
        "provider:ollama",
        "provider:anthropic",
        "assertion:equals",
        "assertion:contains",
        "assertion:regex",
        "assertion:json",
        "assertion:javascript",
        "assertion:python",
    ] {
        assert!(
            fixtures.has_p0_fixture_for(expected),
            "{expected} should have P0 fixture evidence"
        );
    }
}

#[test]
fn test_14_1_3_custom_script_boundaries_cover_default_deny_timeout_allowlist_and_redaction() {
    /* TEST-14.1.3 */
    let matrix = load_matrix();
    let fixtures = load_fixtures(&matrix);
    let report = validate_provider_assertion_parity(&matrix, &fixtures);

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
    assert_eq!(
        report.target_status_for("assertion:javascript"),
        Some("bridge")
    );
    assert_eq!(report.target_status_for("assertion:python"), Some("bridge"));
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
