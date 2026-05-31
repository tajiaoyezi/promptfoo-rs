use std::path::Path;
use std::process::Command;

use promptfoo_rs::compatibility::fixtures::FixtureCorpus;
use promptfoo_rs::compatibility::matrix::CapabilityMatrix;
use promptfoo_rs::compatibility::provider_assertion::{
    resolve_provider_request_response_fixture, validate_p0_provider_module_burndown,
    validate_provider_fixture_burndown, write_provider_fixture_burndown,
    ProviderModuleResolutionKind,
};
use serde_json::Value;

#[test]
fn test_19_3_1_non_external_provider_modules_have_dedicated_fixtures() {
    /* TEST-19.3.1 */
    for (item_id, fixture_id) in [
        (
            "provider:src-providers-anthropic-completion",
            "p0-provider-anthropic-completion",
        ),
        (
            "provider:src-providers-httpmultipart",
            "p0-provider-http-multipart",
        ),
        (
            "provider:src-providers-openai-completion",
            "p0-provider-openai-completion",
        ),
        (
            "provider:src-providers-openai-embedding",
            "p0-provider-openai-embedding",
        ),
        (
            "provider:src-providers-openai-image",
            "p0-provider-openai-image",
        ),
        (
            "provider:src-providers-openai-moderation",
            "p0-provider-openai-moderation",
        ),
        (
            "provider:src-providers-openai-responses",
            "p0-provider-openai-responses",
        ),
        (
            "provider:src-providers-openai-transcription",
            "p0-provider-openai-transcription",
        ),
        (
            "provider:src-providers-openai-video",
            "p0-provider-openai-video",
        ),
    ] {
        let resolution = resolve_provider_request_response_fixture(item_id);
        assert_eq!(
            resolution.kind,
            ProviderModuleResolutionKind::FixtureCovered,
            "{resolution:#?}"
        );
        assert_eq!(resolution.fixture_ids, vec![fixture_id.to_string()]);
        assert!(resolution.verification.starts_with("fixture:"), "{resolution:#?}");
        assert!(!resolution.requires_external_authority, "{resolution:#?}");
        assert!(resolution.reason.contains("request/response"), "{resolution:#?}");
    }

    let codex = resolve_provider_request_response_fixture("provider:src-providers-openai-codex-sdk");
    assert_eq!(codex.kind, ProviderModuleResolutionKind::ExternalBlocker);
    assert!(codex.requires_external_authority, "{codex:#?}");
    assert!(codex.reason.contains("Codex"), "{codex:#?}");
}

#[test]
fn test_19_3_2_dedicated_provider_fixtures_use_no_real_secrets() {
    /* TEST-19.3.2 */
    let matrix = load_matrix();
    let fixtures = load_fixtures(&matrix);
    let report = validate_p0_provider_module_burndown(&matrix, &fixtures);
    let fixture_report = validate_provider_fixture_burndown(&report);

    assert!(
        fixture_report.fixtures_requiring_real_secrets.is_empty(),
        "{fixture_report:#?}"
    );
    for fixture_id in fixture_report
        .resolved_by_fixture
        .iter()
        .flat_map(|resolution| resolution.fixture_ids.iter())
        .filter(|fixture_id| fixture_id.contains("completion")
            || fixture_id.contains("embedding")
            || fixture_id.contains("image")
            || fixture_id.contains("moderation")
            || fixture_id.contains("responses")
            || fixture_id.contains("transcription")
            || fixture_id.contains("video")
            || fixture_id.contains("multipart"))
    {
        let record = fixtures
            .records()
            .iter()
            .find(|record| record.manifest.id == *fixture_id)
            .unwrap_or_else(|| panic!("{fixture_id} should be a fixture manifest"));
        assert!(record.manifest.required_env.is_empty(), "{record:#?}");
        assert!(
            record
                .manifest
                .expected_outputs
                .contains(&"request-json".to_string()),
            "{record:#?}"
        );
        assert!(
            record
                .manifest
                .expected_outputs
                .contains(&"response-json".to_string()),
            "{record:#?}"
        );
        assert!(
            record
                .manifest
                .expected_outputs
                .contains(&"redaction-evidence".to_string()),
            "{record:#?}"
        );
    }
}

#[test]
fn test_19_3_3_longtail_report_updates_provider_fixture_burndown_counts() {
    /* TEST-19.3.3 */
    let output = Command::new(git_bash())
        .args([
            "-lc",
            "LONGTAIL_SKIP_UNIT_TEST=1 bash scripts/release/longtail-classification.sh",
        ])
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
    assert_eq!(burndown["resolved_by_fixture_count"], 22);
    assert_eq!(burndown["new_dedicated_request_response_fixture_count"], 9);
    assert_eq!(burndown["remaining_blocker_count"], 15);
    assert_eq!(burndown["external_authority_blocker_count"], 15);
    assert_eq!(burndown["generic_blocker_count"], 0);
    assert_eq!(report["p0_release_blocker_count"], 15);

    let remaining = report["p0_release_blockers"]
        .as_array()
        .expect("remaining blockers should be listed");
    for resolved in [
        "provider:src-providers-openai-completion",
        "provider:src-providers-openai-embedding",
        "provider:src-providers-openai-responses",
        "provider:src-providers-httpmultipart",
    ] {
        assert!(
            !remaining.iter().any(|item| item["item_id"] == resolved),
            "{resolved} should no longer be a release blocker"
        );
    }

    let path = std::env::temp_dir().join(format!(
        "promptfoo-rs-provider-fixture-burndown-{}.json",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&path);
    let matrix = load_matrix();
    let fixtures = load_fixtures(&matrix);
    let report = validate_p0_provider_module_burndown(&matrix, &fixtures);
    let fixture_report = validate_provider_fixture_burndown(&report);
    write_provider_fixture_burndown(&fixture_report, Path::new(&path))
        .expect("provider fixture burndown report should write");
    let json: Value =
        serde_json::from_str(&std::fs::read_to_string(&path).expect("report should be readable"))
            .expect("report should be valid json");
    let _ = std::fs::remove_file(&path);
    assert_eq!(json["resolved_by_fixture_count"], 22);
}

#[test]
fn test_19_3_4_docs_distinguish_fixture_covered_from_external_modules() {
    /* TEST-19.3.4 */
    let matrix =
        std::fs::read_to_string("docs/compatibility/matrix.md").expect("matrix should exist");
    let audit = std::fs::read_to_string(
        "docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md",
    )
    .expect("audit should exist");

    for docs in [matrix, audit] {
        assert!(docs.contains("Task 19.3"), "{docs}");
        assert!(docs.contains("fixture-covered provider modules"), "{docs}");
        assert!(docs.contains("external-authority modules"), "{docs}");
        assert!(!docs.contains("all provider modules complete"), "{docs}");
    }
}

fn load_matrix() -> CapabilityMatrix {
    CapabilityMatrix::from_json_file(Path::new("compatibility/matrix/items.json"))
        .expect("item-level matrix should load")
}

fn load_fixtures(matrix: &CapabilityMatrix) -> FixtureCorpus {
    FixtureCorpus::load(Path::new("compatibility/fixtures"), matrix)
        .expect("fixture corpus should load")
}

fn git_bash() -> &'static str {
    if cfg!(windows) {
        r"C:\Program Files\Git\bin\bash.exe"
    } else {
        "bash"
    }
}
