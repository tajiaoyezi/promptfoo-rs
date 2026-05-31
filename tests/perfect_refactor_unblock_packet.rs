use std::path::Path;

use promptfoo_rs::release::{
    build_perfect_refactor_claim_contract, build_perfect_refactor_unblock_packet,
    validate_perfect_refactor_unblock_packet, write_perfect_refactor_unblock_packet,
    PerfectRefactorClaimInputs, PerfectRefactorUnblockInputs, PerfectRefactorUnblockItem,
    PublicationReadiness,
};
use serde_json::Value;

#[test]
fn test_22_1_1_packet_keeps_claim_false_with_remaining_blockers() {
    /* TEST-22.1.1 */
    let packet = build_perfect_refactor_unblock_packet(blocked_inputs());
    let validation = validate_perfect_refactor_unblock_packet(&packet);

    assert_eq!(
        packet.schema,
        "promptfoo-rs.perfect-refactor-unblock-packet.v1"
    );
    assert_eq!(packet.status, "blocked", "{packet:#?}");
    assert!(!packet.perfect_refactor_claim_allowed, "{packet:#?}");
    assert!(!packet.auto_resolvable, "{packet:#?}");
    assert_eq!(packet.source_p0_accounting_blocker_count, 22, "{packet:#?}");
    assert_eq!(packet.external_authority_blocker_count, 21, "{packet:#?}");
    assert!(packet.required_user_decision_count >= 29, "{packet:#?}");
    assert!(!validation.ready, "{validation:#?}");
    assert_eq!(
        validation.blocked_count,
        packet.decision_items.len(),
        "{validation:#?}"
    );
}

#[test]
fn test_22_1_2_provider_source_blockers_are_deduplicated_by_external_authority() {
    /* TEST-22.1.2 */
    let packet = build_perfect_refactor_unblock_packet(blocked_inputs());
    let provider_id = "provider:src-providers-openai-agents";
    let provider_items = packet
        .decision_items
        .iter()
        .filter(|item| item.item_id == provider_id)
        .collect::<Vec<_>>();
    let source_only_provider_items = packet
        .decision_items
        .iter()
        .filter(|item| item.item_id == provider_id && item.category == "source-accounting")
        .collect::<Vec<_>>();

    assert_eq!(provider_items.len(), 1, "{packet:#?}");
    assert!(source_only_provider_items.is_empty(), "{packet:#?}");
    assert!(packet.decision_items.iter().any(|item| {
        item.item_id == "config:src-globalconfig-cloud" && item.category == "source-accounting"
    }));
}

#[test]
fn test_22_1_3_publication_items_require_external_evidence_not_dry_run() {
    /* TEST-22.1.3 */
    let packet = build_perfect_refactor_unblock_packet(blocked_inputs());
    let cargo = packet
        .decision_items
        .iter()
        .find(|item| item.item_id == "publication:cargo")
        .expect("cargo publication blocker should be listed");

    assert_eq!(cargo.category, "publication-authority");
    assert_eq!(cargo.required_actor, "release maintainer");
    assert!(
        cargo.required_evidence.contains("credentials"),
        "{cargo:#?}"
    );
    assert!(
        cargo.required_evidence.contains("legal/brand approval"),
        "{cargo:#?}"
    );
    assert!(
        cargo.required_evidence.contains("external URL/digest"),
        "{cargo:#?}"
    );
    assert!(!cargo.auto_resolvable, "{cargo:#?}");
    assert!(
        !cargo.required_evidence.contains("dry-run is enough"),
        "{cargo:#?}"
    );
}

#[test]
fn test_22_1_4_current_upstream_rebaseline_requirement_stays_visible() {
    /* TEST-22.1.4 */
    let packet = build_perfect_refactor_unblock_packet(blocked_inputs());
    let rebaseline = packet
        .decision_items
        .iter()
        .find(|item| item.item_id == "current-upstream:rebaseline")
        .expect("current upstream rebaseline blocker should be listed");

    assert_eq!(rebaseline.category, "current-upstream");
    assert!(
        rebaseline.required_evidence.contains("same-ref"),
        "{rebaseline:#?}"
    );
    assert!(
        rebaseline
            .source_artifact
            .ends_with("upstream-distribution-target.json"),
        "{rebaseline:#?}"
    );
    assert!(!rebaseline.auto_resolvable, "{rebaseline:#?}");
}

#[test]
fn test_22_1_5_runtime_smoke_release_candidate_and_docs_expose_unblock_packet() {
    /* TEST-22.1.5 */
    let packet = build_perfect_refactor_unblock_packet(blocked_inputs());
    let path = std::env::temp_dir().join(format!(
        "promptfoo-rs-perfect-refactor-unblock-packet-{}.json",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&path);
    write_perfect_refactor_unblock_packet(&packet, Path::new(&path))
        .expect("unblock packet should write");
    let json: Value =
        serde_json::from_str(&std::fs::read_to_string(&path).expect("packet should be readable"))
            .expect("packet should be valid json");
    let _ = std::fs::remove_file(&path);
    assert_eq!(json["status"], "blocked");
    assert_eq!(json["auto_resolvable"], false);
    assert!(json["decision_items"].is_array());

    let runtime_script =
        std::fs::read_to_string("scripts/release/runtime-smoke.sh").expect("runtime smoke exists");
    assert!(runtime_script.contains("perfect-refactor-unblock-packet.sh"));
    assert!(runtime_script.contains("perfect_refactor_unblock_packet"));

    for docs_path in [
        "docs/release.md",
        "docs/compatibility/matrix.md",
        "docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md",
    ] {
        let docs = std::fs::read_to_string(docs_path).expect("docs should exist");
        assert!(docs.contains("Task 22.1"), "{docs_path}: {docs}");
        assert!(
            docs.contains("perfect-refactor-unblock-packet.json"),
            "{docs_path}: {docs}"
        );
        assert!(
            docs.contains("auto_resolvable=false"),
            "{docs_path}: {docs}"
        );
    }
}

fn blocked_inputs() -> PerfectRefactorUnblockInputs {
    let claim = build_perfect_refactor_claim_contract(PerfectRefactorClaimInputs {
        local_stable_allowed: true,
        published: false,
        source_p0_accounting_blocker_count: 22,
        current_perfect_claim_allowed: false,
        publication_ready: PublicationReadiness::CredentialBlocked,
        external_authority_status: "blocked".to_string(),
        external_authority_blocker_count: 21,
        source_artifacts: source_artifacts(),
    });
    PerfectRefactorUnblockInputs {
        claim,
        source_p0_blockers: source_p0_blockers(),
        external_authority_items: external_authority_items(),
        current_upstream_rebaseline_required: true,
        source_artifacts: source_artifacts(),
    }
}

fn source_p0_blockers() -> Vec<String> {
    let mut blockers = vec![
        "config:src-globalconfig-cloud".to_string(),
        "provider:src-providers-openai-agents".to_string(),
        "provider:src-providers-openai-billing".to_string(),
    ];
    for index in 0..19 {
        blockers.push(format!("config:test-external-{index}"));
    }
    blockers
}

fn external_authority_items() -> Vec<PerfectRefactorUnblockItem> {
    let mut items = vec![
        external_item(
            "provider:src-providers-openai-agents",
            "provider-authority",
            "product owner",
            "Product authority approval and live product contract evidence",
        ),
        external_item(
            "provider:src-providers-openai-billing",
            "provider-authority",
            "account owner",
            "Account credentials and billing authority evidence",
        ),
        publication_item("publication:cargo", "Cargo"),
        publication_item("publication:github-releases", "GitHub Releases"),
        publication_item("publication:npm-wrapper", "npm wrapper"),
        publication_item("publication:docker", "Docker"),
        publication_item("publication:homebrew", "Homebrew"),
        publication_item("publication:github-action", "GitHub Action"),
    ];
    for index in 0..13 {
        items.push(external_item(
            &format!("provider:test-external-{index}"),
            "provider-authority",
            "product owner",
            "Product authority approval and service contract evidence",
        ));
    }
    items
}

fn external_item(
    item_id: &str,
    category: &str,
    required_actor: &str,
    required_evidence: &str,
) -> PerfectRefactorUnblockItem {
    PerfectRefactorUnblockItem {
        item_id: item_id.to_string(),
        category: category.to_string(),
        authority_type: "product-authority".to_string(),
        required_actor: required_actor.to_string(),
        required_evidence: required_evidence.to_string(),
        source_artifact: "target/release-gates/external-authority-blockers.json".to_string(),
        source_reference: Some(format!("promptfoo@0.121.13:{item_id}")),
        safe_local_fallback: "Keep local fixture or dry-run evidence only".to_string(),
        release_impact: "Blocks perfect-refactor claim until external evidence exists".to_string(),
        auto_resolvable: false,
    }
}

fn publication_item(item_id: &str, label: &str) -> PerfectRefactorUnblockItem {
    PerfectRefactorUnblockItem {
        item_id: item_id.to_string(),
        category: "publication-authority".to_string(),
        authority_type: "publication-authority".to_string(),
        required_actor: "release maintainer".to_string(),
        required_evidence: format!(
            "{label} publication requires credentials, release authority, legal/brand approval, and external URL/digest evidence"
        ),
        source_artifact: "target/release-gates/publication-authority.json".to_string(),
        source_reference: Some(format!(
            "target/release-gates/publication-authority.json#{}",
            item_id.strip_prefix("publication:").unwrap_or(item_id)
        )),
        safe_local_fallback: "Keep dry-run installability evidence only".to_string(),
        release_impact: format!("{label} published=false; public availability remains blocked"),
        auto_resolvable: false,
    }
}

fn source_artifacts() -> Vec<String> {
    vec![
        "target/release-gates/source-inventory-evidence.json".to_string(),
        "target/release-gates/external-authority-blockers.json".to_string(),
        "target/release-gates/publication-authority.json".to_string(),
        "target/release-gates/upstream-distribution-target.json".to_string(),
        "target/release-gates/perfect-refactor-claim.json".to_string(),
    ]
}
