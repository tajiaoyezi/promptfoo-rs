use std::path::Path;

use promptfoo_rs::release::{
    load_publication_evidence_manifest, validate_publication_evidence,
    write_publication_evidence_gate_report,
};
use serde_json::{json, Value};

#[test]
fn test_43_2_1_every_publication_channel_has_one_manifest_row() {
    /* TEST-43.2.1 */
    let authority = load_publication_authority();
    let manifest = load_tracked_manifest();
    let report = validate_publication_evidence(&authority, &manifest);

    assert_eq!(report.schema, "promptfoo-rs.publication-evidence-gate.v1");
    assert_eq!(
        report.required_channel_count,
        authority["channels"]
            .as_array()
            .map(|rows| rows.len())
            .unwrap_or(0),
        "{report:#?}"
    );
    assert_eq!(
        report.manifest_row_count, report.required_channel_count,
        "{report:#?}"
    );
    assert!(report.missing_manifest_rows.is_empty(), "{report:#?}");
    assert!(report.extra_manifest_rows.is_empty(), "{report:#?}");
    assert!(report.duplicate_manifest_rows.is_empty(), "{report:#?}");

    for channel in authority["channels"].as_array().expect("channels array") {
        let channel_name = channel["channel"].as_str().expect("channel");
        let matches = manifest["rows"]
            .as_array()
            .expect("manifest rows")
            .iter()
            .filter(|row| row["channel"] == channel_name)
            .count();
        assert_eq!(matches, 1, "{channel_name}: {manifest:#?}");
    }
}

#[test]
fn test_43_2_2_dry_run_installability_never_sets_published_true() {
    /* TEST-43.2.2 */
    let authority = load_publication_authority();
    let manifest = load_tracked_manifest();
    let report = validate_publication_evidence(&authority, &manifest);

    assert!(!report.publication_ready(), "{report:#?}");
    assert_eq!(
        report.blocked_channel_count, report.required_channel_count,
        "{report:#?}"
    );
    assert_eq!(report.published_channel_count, 0, "{report:#?}");

    let mut dry_run_manifest = manifest.clone();
    let channel_name = dry_run_manifest["rows"][0]["channel"]
        .as_str()
        .expect("channel")
        .to_string();
    dry_run_manifest["rows"][0] = json!({
        "channel": channel_name,
        "publication_state": "published",
        "authority_owner": "release maintainer",
        "credential_authority_reference": "approval:release-credential-policy-2026-06",
        "legal_brand_approval_reference": "approval:legal-brand-review-2026-06",
        "artifact_url": "target/release-installability/cargo-package-dry-run.json",
        "digest": "sha256:local-dry-run-only",
        "release_notes_reference": "docs/release.md#installability-dry-run",
        "publication_timestamp": "2026-06-06T00:00:00Z",
        "no_upload_provenance": "local dry-run only; no upload command executed"
    });
    let dry_run_report = validate_publication_evidence(&authority, &dry_run_manifest);
    assert!(!dry_run_report.publication_ready(), "{dry_run_report:#?}");
    assert!(
        dry_run_report
            .dry_run_only_published_rows
            .contains(&channel_name),
        "{dry_run_report:#?}"
    );

    let runtime_script = std::fs::read_to_string("scripts/release/runtime-smoke.sh")
        .expect("runtime smoke script should exist");
    assert!(runtime_script.contains("publication-evidence.sh"));
    assert!(runtime_script.contains("publication_evidence"));
}

#[test]
fn test_43_2_3_published_rows_require_url_digest_release_notes_credential_legal_and_timestamp() {
    /* TEST-43.2.3 */
    let authority = load_publication_authority();
    let channel_name = authority["channels"][0]["channel"]
        .as_str()
        .expect("channel")
        .to_string();

    let incomplete = json!({
        "schema": "promptfoo-rs.publication-evidence.v1",
        "rows": [{
            "channel": channel_name,
            "publication_state": "published",
            "authority_owner": "release maintainer",
            "artifact_url": "https://example.com/artifact.tgz"
        }]
    });
    let incomplete_report = validate_publication_evidence(&authority, &incomplete);
    assert!(
        incomplete_report
            .incomplete_published_rows
            .contains(&channel_name),
        "{incomplete_report:#?}"
    );
    assert!(
        !incomplete_report.publication_ready(),
        "{incomplete_report:#?}"
    );

    let mut complete_one = load_tracked_manifest();
    complete_one["rows"][0] = json!({
        "channel": channel_name,
        "publication_state": "published",
        "authority_owner": "release maintainer",
        "credential_authority_reference": "approval:release-credential-policy-2026-06",
        "legal_brand_approval_reference": "approval:legal-brand-review-2026-06",
        "artifact_url": "https://github.com/example/promptfoo-rs/releases/download/v0.1.0/archive.tar.gz",
        "digest": "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        "release_notes_reference": "docs/release.md#v0.1.0",
        "publication_timestamp": "2026-06-06T12:00:00Z",
        "no_upload_provenance": "external release executed with maintainer authorization; artifact URL and digest recorded"
    });
    let partial_report = validate_publication_evidence(&authority, &complete_one);
    assert!(
        !partial_report
            .incomplete_published_rows
            .contains(&channel_name),
        "{partial_report:#?}"
    );
    assert!(
        !partial_report.publication_ready(),
        "one published channel cannot clear aggregate readiness while others remain blocked: {partial_report:#?}"
    );
}

#[test]
fn test_43_2_4_manifest_stores_no_publish_tokens_or_private_credentials() {
    /* TEST-43.2.4 */
    let authority = load_publication_authority();
    let manifest = load_tracked_manifest();
    let report = validate_publication_evidence(&authority, &manifest);

    assert!(report.secret_bearing_rows.is_empty(), "{report:#?}");

    let mut secret_manifest = manifest.clone();
    let channel_name = secret_manifest["rows"][0]["channel"]
        .as_str()
        .expect("channel")
        .to_string();
    secret_manifest["rows"][0] = json!({
        "channel": channel_name,
        "publication_state": "published",
        "authority_owner": "release maintainer",
        "credential_authority_reference": "npm publish token sk-live-secret",
        "legal_brand_approval_reference": "approval:legal-brand-review-2026-06",
        "artifact_url": "https://registry.npmjs.org/promptfoo-rs/-/promptfoo-rs-0.1.0.tgz",
        "digest": "sha512:example",
        "release_notes_reference": "docs/release.md#v0.1.0",
        "publication_timestamp": "2026-06-06T12:00:00Z",
        "no_upload_provenance": "external npm publish with maintainer authorization"
    });
    let secret_report = validate_publication_evidence(&authority, &secret_manifest);
    assert!(
        !secret_report.secret_bearing_rows.is_empty(),
        "{secret_report:#?}"
    );
    assert!(!secret_report.publication_ready(), "{secret_report:#?}");

    let serialized = serde_json::to_string(&manifest).expect("manifest json");
    for forbidden in ["sk-live-secret", "publish token", "api_key="] {
        assert!(
            !serialized.contains(forbidden),
            "tracked manifest must not contain secrets: {forbidden}"
        );
    }

    let output = std::env::temp_dir().join(format!(
        "promptfoo-rs-publication-evidence-gate-{}.json",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&output);
    write_publication_evidence_gate_report(&report, Path::new(&output))
        .expect("gate report should write");
    let gate_json: Value = serde_json::from_str(
        &std::fs::read_to_string(&output).expect("gate report should be readable"),
    )
    .expect("gate report should be valid json");
    let _ = std::fs::remove_file(&output);
    assert_eq!(gate_json["status"], "credential-blocked");
    assert_eq!(gate_json["publication_ready"], false);
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
