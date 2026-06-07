use std::process::Command;

use promptfoo_rs::compatibility::inventory::{
    evaluate_current_claim_policy, FrozenSourceReference, TargetMode,
};
use promptfoo_rs::release::{
    build_perfect_refactor_claim_contract, build_perfect_refactor_unblock_packet,
    PerfectRefactorClaimInputs, PerfectRefactorUnblockInputs, PublicationReadiness,
};
use serde_json::Value;

const PRODUCT_BASELINE_SHA: &str = "4805856060d026521794d4e69decb938155580ad";
const CURRENT_HEAD_SHA: &str = "c54a30668ad8319d76c20ae96e6680ad6c51a2c6";
const LS_REMOTE: &str = "\
c54a30668ad8319d76c20ae96e6680ad6c51a2c6\tHEAD
4805856060d026521794d4e69decb938155580ad\trefs/tags/0.121.15
4860e990c7e9a2f8f677173fb92cf9867b34d03f\trefs/tags/0.121.13
";

#[test]
fn test_49_1_1_product_baseline_policy_records_adr012_freeze_without_rebaseline() {
    /* TEST-49.1.1 */
    let observation =
        promptfoo_rs::compatibility::inventory::CurrentUpstreamObservation::from_ls_remote(
            LS_REMOTE,
        )
        .expect("ls-remote should parse");
    let frozen = FrozenSourceReference::new(
        "0.121.15",
        "refs/tags/0.121.15",
        PRODUCT_BASELINE_SHA,
        "sha512-product-baseline",
        "compatibility/inventory/current-latest-target.json",
    );
    let policy = evaluate_current_claim_policy(&frozen, &observation, TargetMode::ProductBaseline);

    assert_eq!(policy.target_mode, TargetMode::ProductBaseline);
    assert!(policy.product_baseline_frozen, "{policy:#?}");
    assert!(!policy.current_upstream_rebaseline_required, "{policy:#?}");
    assert!(!policy.current_perfect_claim_allowed, "{policy:#?}");
    assert_eq!(
        policy.stable_claim,
        "product-baseline compatibility (ADR-012)"
    );
    assert!(policy.reason.contains("ADR-012"), "{policy:#?}");
    assert_eq!(observation.current_head, CURRENT_HEAD_SHA);
}

#[test]
fn test_49_1_2_product_baseline_frozen_unblock_packet_skips_rebaseline_decision() {
    /* TEST-49.1.2 */
    let mut inputs = product_baseline_unblock_inputs();
    inputs.product_baseline_frozen = true;
    inputs.current_upstream_rebaseline_required = true;
    let packet = build_perfect_refactor_unblock_packet(inputs);

    assert!(!packet.current_upstream_rebaseline_required, "{packet:#?}");
    assert!(packet.product_baseline_frozen, "{packet:#?}");
    assert!(
        !packet
            .decision_items
            .iter()
            .any(|item| item.item_id == "current-upstream:rebaseline"),
        "{packet:#?}"
    );
}

#[test]
fn test_49_2_1_gate_lib_resolves_authority_and_v1_publication_scope() {
    /* TEST-49.2.1 */
    let status = Command::new("node")
        .args([
            "-e",
            r#"
const {
  loadAuthorityDecisions,
  isResolvedAuthorityDecision,
  loadPublicationEvidence,
  v1PublicationScopeReady,
} = require('./scripts/release/product-baseline-gate-lib.cjs');
const { byId } = loadAuthorityDecisions('docs/compatibility/authority-decisions.json');
const { byChannel } = loadPublicationEvidence('docs/compatibility/publication-evidence.json');
const required = ['github-releases','cargo','npm-wrapper','docker','homebrew','github-action'];
if (!isResolvedAuthorityDecision('config:src-globalconfig-cloud', byId)) process.exit(2);
if (!isResolvedAuthorityDecision('publication:github-releases', byId)) process.exit(3);
if (!v1PublicationScopeReady(required, byChannel)) process.exit(4);
console.log('ok');
"#,
        ])
        .current_dir(".")
        .status()
        .expect("node should run");
    assert!(
        status.success(),
        "authority/publication gate lib should resolve v1 scope"
    );
}

#[test]
fn test_49_2_2_tracked_product_baseline_lock_matches_phase48_packet() {
    /* TEST-49.2.2 */
    let target: Value = serde_json::from_str(
        &std::fs::read_to_string("compatibility/inventory/current-latest-target.json")
            .expect("tracked target lock"),
    )
    .expect("target json");
    assert_eq!(target["npm_latest"]["package_version"], "0.121.15");
    assert_eq!(target["npm_latest"]["git_head"], PRODUCT_BASELINE_SHA);
    assert_eq!(
        target["github"]["default_branch_head"].as_str(),
        Some(CURRENT_HEAD_SHA)
    );

    let status = Command::new("node")
        .args([
            "-e",
            r#"
const { loadProductBaselineTarget } = require('./scripts/release/product-baseline-gate-lib.cjs');
const baseline = loadProductBaselineTarget('compatibility/inventory/current-latest-target.json');
if (!baseline || baseline.package_version !== '0.121.15') process.exit(2);
if (baseline.git_commit !== '4805856060d026521794d4e69decb938155580ad') process.exit(3);
console.log('ok');
"#,
        ])
        .current_dir(".")
        .status()
        .expect("node should load product baseline");
    assert!(
        status.success(),
        "gate lib should read Phase 48 product baseline lock"
    );
}

#[test]
fn test_49_2_3_publication_evidence_manifest_counts_v1_deferred_as_resolved() {
    /* TEST-49.2.3 */
    let manifest: Value = serde_json::from_str(
        &std::fs::read_to_string("docs/compatibility/publication-evidence.json")
            .expect("publication evidence manifest"),
    )
    .expect("manifest json");
    let deferred = manifest["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .filter(|row| row["v1_deferred"] == true)
        .count();
    let published = manifest["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .filter(|row| row["publication_state"] == "published")
        .count();
    assert_eq!(published, 1, "{manifest:#}");
    assert_eq!(deferred, 5, "{manifest:#}");
}

fn product_baseline_unblock_inputs() -> PerfectRefactorUnblockInputs {
    let claim = build_perfect_refactor_claim_contract(PerfectRefactorClaimInputs {
        local_stable_allowed: true,
        published: false,
        source_p0_accounting_blocker_count: 22,
        current_perfect_claim_allowed: false,
        publication_ready: PublicationReadiness::CredentialBlocked,
        external_authority_status: "blocked".to_string(),
        external_authority_blocker_count: 21,
        source_artifacts: vec!["target/release-gates/perfect-refactor-claim.json".to_string()],
    });
    PerfectRefactorUnblockInputs {
        claim,
        source_p0_blockers: vec!["config:src-globalconfig-cloud".to_string()],
        external_authority_items: vec![],
        current_upstream_rebaseline_required: true,
        product_baseline_frozen: false,
        resolved_decision_ids: Vec::new(),
        source_artifacts: vec!["target/release-gates/upstream-distribution-target.json".to_string()],
    }
}
