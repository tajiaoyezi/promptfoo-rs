use std::path::Path;

use promptfoo_rs::release::{
    apply_publication_evidence, load_publication_evidence_manifest, validate_publication_evidence,
};
use serde_json::Value;

#[test]
fn test_44_2_1_tracked_manifest_maps_every_publication_channel() {
    /* TEST-44.2.1 */
    let authority = load_publication_authority();
    let manifest = load_tracked_manifest();
    let report = validate_publication_evidence(&authority, &manifest);

    assert_eq!(
        report.manifest_row_count, report.required_channel_count,
        "{report:#?}"
    );
    assert!(report.missing_manifest_rows.is_empty(), "{report:#?}");
}

#[test]
fn test_44_2_2_no_channel_is_published_from_local_dry_run_evidence() {
    /* TEST-44.2.2 */
    let authority = load_publication_authority();
    let manifest = load_tracked_manifest();
    let report = validate_publication_evidence(&authority, &manifest);

    assert!(!report.publication_ready(), "{report:#?}");
    assert_eq!(report.published_channel_count, 1, "{report:#?}");
    assert_eq!(report.blocked_channel_count, 5, "{report:#?}");
    assert!(report.dry_run_only_published_rows.is_empty(), "{report:#?}");
}

#[test]
fn test_44_2_3_v1_defers_non_github_channels_while_github_awaits_real_release() {
    /* TEST-44.2.3 */
    let authority = load_publication_authority();
    let manifest = load_tracked_manifest();
    let application = apply_publication_evidence(&manifest, &authority);

    assert!(!application.publication_ready, "{application:#?}");
    assert_eq!(
        application.published_channels(),
        vec!["github-releases".to_string()],
        "{application:#?}"
    );
    assert_eq!(application.deferred_channels.len(), 5, "{application:#?}");
    for channel in [
        "cargo",
        "docker",
        "github-action",
        "homebrew",
        "npm-wrapper",
    ] {
        assert!(
            application.deferred_channels.contains(&channel.to_string()),
            "{application:#?}"
        );
    }

    let github = manifest["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .find(|row| row["channel"] == "github-releases")
        .expect("github-releases row");
    assert_eq!(github["publication_state"], "published");
    assert!(
        github["artifact_url"]
            .as_str()
            .unwrap_or_default()
            .starts_with("https://github.com/tajiaoyezi/promptfoo-rs/releases/download/v0.1.1/"),
        "{github:#?}"
    );
    assert!(
        github["credential_authority_reference"]
            .as_str()
            .unwrap_or_default()
            .contains("approval:v1-github-releases-only"),
        "{github:#?}"
    );
}

#[test]
fn test_44_2_4_manifest_contains_no_publish_tokens() {
    /* TEST-44.2.4 */
    let manifest = load_tracked_manifest();
    let serialized = serde_json::to_string(&manifest).expect("manifest json");
    for forbidden in [
        "npm publish token",
        "crates.io publish token",
        "ghp_",
        "sk-",
    ] {
        assert!(
            !serialized.contains(forbidden),
            "publication manifest must not contain secrets: {forbidden}"
        );
    }
}

fn load_publication_authority() -> Value {
    serde_json::from_str(
        &std::fs::read_to_string("target/release-gates/publication-authority.json")
            .expect("publication authority should exist"),
    )
    .expect("publication authority should be valid json")
}

fn load_tracked_manifest() -> Value {
    load_publication_evidence_manifest(Path::new("docs/compatibility/publication-evidence.json"))
        .expect("tracked publication evidence manifest should load")
}
