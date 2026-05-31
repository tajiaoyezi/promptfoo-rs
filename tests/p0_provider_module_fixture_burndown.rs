use std::path::Path;
use std::process::Command;

use promptfoo_rs::compatibility::fixtures::FixtureCorpus;
use promptfoo_rs::compatibility::matrix::CapabilityMatrix;
use promptfoo_rs::compatibility::provider_assertion::{
    compatibility_gap_error, provider_module_blocker_rows, resolve_provider_module_fixture,
    validate_p0_provider_module_burndown, GapClass, ProviderModuleResolutionKind,
};
use serde_json::Value;

#[test]
fn test_18_2_1_provider_module_blockers_have_fixture_or_explicit_blocker_evidence() {
    /* TEST-18.2.1 */
    let matrix = load_matrix();
    let fixtures = load_fixtures(&matrix);
    let blocker_rows = provider_module_blocker_rows(&matrix);
    let report = validate_p0_provider_module_burndown(&matrix, &fixtures);

    assert_eq!(
        blocker_rows.len(),
        37,
        "task 18.2 starts from the 37 P0 provider module blockers captured by task 17.4"
    );
    assert_eq!(report.initial_blocker_count, blocker_rows.len());
    assert!(
        report.resolved_by_fixture_count > 0,
        "{report:#?}"
    );
    assert!(
        report.remaining_blocker_count < report.initial_blocker_count,
        "{report:#?}"
    );
    assert_eq!(
        report.initial_blocker_count,
        report.resolved_by_fixture_count + report.remaining_blocker_count
    );

    let anthropic_messages = resolution_for(
        &blocker_rows,
        &fixtures,
        "provider:src-providers-anthropic-messages",
    );
    assert_eq!(
        anthropic_messages.kind,
        ProviderModuleResolutionKind::FixtureCovered
    );
    assert!(anthropic_messages
        .fixture_ids
        .contains(&"p0-provider-anthropic-message".to_string()));
    assert!(anthropic_messages
        .reason
        .contains("aggregate provider fixture"));

    let codex_sdk = resolution_for(
        &blocker_rows,
        &fixtures,
        "provider:src-providers-openai-codex-sdk",
    );
    assert_eq!(codex_sdk.kind, ProviderModuleResolutionKind::ExternalBlocker);
    assert!(codex_sdk.requires_external_authority, "{codex_sdk:#?}");
    assert!(codex_sdk.reason.contains("Codex"), "{codex_sdk:#?}");
    assert!(codex_sdk.verification.starts_with("blocker:"));

    for blocker in &report.remaining_blockers {
        assert!(!blocker.item_id.trim().is_empty(), "{blocker:#?}");
        assert!(!blocker.source_reference.trim().is_empty(), "{blocker:#?}");
        assert!(!blocker.reason.trim().is_empty(), "{blocker:#?}");
        assert!(
            blocker.verification.starts_with("blocker:"),
            "{blocker:#?}"
        );
    }
}

#[test]
fn test_18_2_2_provider_module_burndown_uses_no_real_provider_secrets() {
    /* TEST-18.2.2 */
    let matrix = load_matrix();
    let fixtures = load_fixtures(&matrix);
    let report = validate_p0_provider_module_burndown(&matrix, &fixtures);

    assert!(
        report.fixtures_requiring_real_secrets.is_empty(),
        "{report:#?}"
    );
    for resolution in &report.resolved_by_fixture {
        assert!(!resolution.fixture_ids.is_empty(), "{resolution:#?}");
        assert!(
            resolution
                .fixture_ids
                .iter()
                .all(|fixture_id| fixture_id.starts_with("p0-provider-")),
            "{resolution:#?}"
        );
    }
}

#[test]
fn test_18_2_3_longtail_report_lists_remaining_provider_module_blockers() {
    /* TEST-18.2.3 */
    let output = Command::new(git_bash())
        .args(["-lc", "LONGTAIL_SKIP_UNIT_TEST=1 bash scripts/release/longtail-classification.sh"])
        .output()
        .expect("longtail classification script should execute");
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let report: Value = serde_json::from_str(
        &std::fs::read_to_string("target/release-gates/longtail-classification.json")
            .expect("longtail classification report should exist"),
    )
    .expect("longtail classification report should be valid JSON");
    let burndown = &report["p0_provider_module_burndown"];
    assert_eq!(burndown["initial_blocker_count"], 37);
    assert!(
        burndown["resolved_by_fixture_count"].as_u64().unwrap() > 0,
        "{report:#?}"
    );
    assert!(
        burndown["remaining_blocker_count"].as_u64().unwrap() < 37,
        "{report:#?}"
    );
    assert_eq!(
        report["p0_release_blocker_count"],
        burndown["remaining_blocker_count"]
    );

    let remaining = report["p0_release_blockers"]
        .as_array()
        .expect("remaining blockers should be listed");
    assert_eq!(
        remaining.len() as u64,
        burndown["remaining_blocker_count"].as_u64().unwrap()
    );
    assert!(remaining.iter().any(|item| {
        item["item_id"] == "provider:src-providers-openai-codex-sdk"
            && item["reason"].as_str().unwrap_or_default().contains("Codex")
    }));
    assert!(!remaining
        .iter()
        .any(|item| item["item_id"] == "provider:src-providers-openai-chat"));
    for item in remaining {
        assert!(item["item_id"].as_str().unwrap_or_default().starts_with("provider:src-providers-"));
        assert!(!item["reason"].as_str().unwrap_or_default().trim().is_empty());
        assert!(item["verification"].as_str().unwrap_or_default().starts_with("blocker:"));
    }
}

#[test]
fn test_18_2_4_provider_gap_error_includes_item_class_reason_docs_and_exit_code() {
    /* TEST-18.2.4 */
    let error = compatibility_gap_error(
        "provider:src-providers-openai-codex-sdk",
        GapClass::Blocked,
        "Codex provider requires external product authority and CODEX_TOKEN=secret",
    );
    let message = error.to_string();

    assert_eq!(error.exit_code(), 1);
    assert_eq!(error.item_id(), "provider:src-providers-openai-codex-sdk");
    assert_eq!(error.class(), GapClass::Blocked);
    assert!(message.contains("provider:src-providers-openai-codex-sdk"));
    assert!(message.contains("blocked"));
    assert!(message.contains("reason:"));
    assert!(message.contains("Codex provider requires external product authority"));
    assert!(message.contains("docs/compatibility/matrix.md"));
    assert!(!message.contains("secret"), "{message}");
}

fn load_matrix() -> CapabilityMatrix {
    CapabilityMatrix::from_json_file(Path::new("compatibility/matrix/items.json"))
        .expect("item-level matrix should load")
}

fn load_fixtures(matrix: &CapabilityMatrix) -> FixtureCorpus {
    FixtureCorpus::load(Path::new("compatibility/fixtures"), matrix)
        .expect("fixture corpus should load")
}

fn resolution_for(
    rows: &[promptfoo_rs::compatibility::matrix::CapabilityRow],
    fixtures: &FixtureCorpus,
    item_id: &str,
) -> promptfoo_rs::compatibility::provider_assertion::ProviderModuleResolution {
    let row = rows
        .iter()
        .find(|row| row.capability == item_id)
        .unwrap_or_else(|| panic!("{item_id} should be a provider module blocker"));
    resolve_provider_module_fixture(row, fixtures)
}

fn git_bash() -> &'static str {
    if cfg!(windows) {
        r"C:\Program Files\Git\bin\bash.exe"
    } else {
        "bash"
    }
}
