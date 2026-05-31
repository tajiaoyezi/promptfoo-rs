use std::path::Path;

use promptfoo_rs::compatibility::fixtures::{
    fixture_count_by_priority, validate_p0_fixture_corpus, Priority, ProviderMocking,
};
use promptfoo_rs::compatibility::matrix::CapabilityMatrix;

#[test]
fn test_12_1_1_repository_tracks_at_least_50_p0_fixtures() {
    /* TEST-12.1.1 */
    let matrix = CapabilityMatrix::from_json_file(Path::new("compatibility/matrix/items.json"))
        .expect("item-level matrix should load");
    let report = validate_p0_fixture_corpus(Path::new("compatibility/fixtures"), &matrix);

    assert!(
        report.tracked_fixture_count >= 50,
        "tracked fixture count too low: {report:#?}"
    );
    assert!(
        fixture_count_by_priority(&report, Priority::P0) >= 50,
        "{report:#?}"
    );
}

#[test]
fn test_12_1_2_every_fixture_has_metadata_and_matrix_linkage() {
    /* TEST-12.1.2 */
    let matrix = CapabilityMatrix::from_json_file(Path::new("compatibility/matrix/items.json"))
        .expect("item-level matrix should load");
    let report = validate_p0_fixture_corpus(Path::new("compatibility/fixtures"), &matrix);

    assert!(report.invalid_fixtures.is_empty(), "{report:#?}");
    assert!(report.matrix_links_missing.is_empty(), "{report:#?}");
    for record in &report.fixtures {
        assert!(!record.manifest.id.trim().is_empty());
        assert!(
            is_s2v_test_id(&record.manifest.test_id),
            "{} has invalid test_id {}",
            record.path.display(),
            record.manifest.test_id
        );
        assert!(!record.manifest.matrix_item_ids.is_empty());
        assert!(!record.manifest.expected_outputs.is_empty());
        assert!(!record.manifest.normalization_rules.is_empty());
    }
}

fn is_s2v_test_id(test_id: &str) -> bool {
    let Some(number) = test_id.strip_prefix("TEST-") else {
        return false;
    };
    let mut parts = number.split('.');
    match (parts.next(), parts.next(), parts.next(), parts.next()) {
        (Some(major), Some(minor), Some(case), None) => [major, minor, case]
            .into_iter()
            .all(|part| !part.is_empty() && part.chars().all(|ch| ch.is_ascii_digit())),
        _ => false,
    }
}

#[test]
fn test_12_1_3_fixtures_use_mock_or_recorded_providers_without_real_secrets() {
    /* TEST-12.1.3 */
    let matrix = CapabilityMatrix::from_json_file(Path::new("compatibility/matrix/items.json"))
        .expect("item-level matrix should load");
    let report = validate_p0_fixture_corpus(Path::new("compatibility/fixtures"), &matrix);

    assert!(
        report.fixtures_requiring_real_secrets.is_empty(),
        "{report:#?}"
    );
    for record in &report.fixtures {
        assert!(matches!(
            record.manifest.provider_mocking,
            ProviderMocking::Mock | ProviderMocking::Recorded
        ));
        assert!(record.manifest.blocks_stable_release);
    }
}
