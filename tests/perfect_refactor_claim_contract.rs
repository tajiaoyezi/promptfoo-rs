use std::path::Path;

use promptfoo_rs::release::{
    build_perfect_refactor_claim_contract, validate_perfect_refactor_claim,
    write_perfect_refactor_claim_contract, PerfectRefactorClaimInputs, PublicationReadiness,
};
use serde_json::Value;

#[test]
fn test_20_2_1_perfect_refactor_claim_stays_false_with_remaining_blockers() {
    /* TEST-20.2.1 */
    let contract = build_perfect_refactor_claim_contract(blocked_inputs());
    let decision = validate_perfect_refactor_claim(&contract);

    assert_eq!(contract.schema, "promptfoo-rs.perfect-refactor-claim.v1");
    assert!(contract.local_stable_allowed, "{contract:#?}");
    assert!(!contract.perfect_refactor_claim_allowed, "{contract:#?}");
    assert!(!contract.local_stable_is_perfect_refactor, "{contract:#?}");
    assert!(!decision.ready, "{decision:#?}");
    assert_eq!(
        decision.blocker_count,
        contract.blockers.len(),
        "{decision:#?}"
    );
    assert!(contract
        .blockers
        .iter()
        .any(|blocker| blocker.category == "source-accounting"));
    assert!(contract
        .blockers
        .iter()
        .any(|blocker| blocker.category == "current-upstream"));
    assert!(contract
        .blockers
        .iter()
        .any(|blocker| blocker.category == "external-authority"));
    assert!(contract
        .blockers
        .iter()
        .any(|blocker| blocker.category == "publication-authority"));
}

#[test]
fn test_20_2_2_local_stable_does_not_imply_publication_or_perfect_claim() {
    /* TEST-20.2.2 */
    let mut inputs = blocked_inputs();
    inputs.local_stable_allowed = true;
    inputs.published = false;
    inputs.publication_ready = PublicationReadiness::CredentialBlocked;

    let contract = build_perfect_refactor_claim_contract(inputs);

    assert!(contract.local_stable_allowed, "{contract:#?}");
    assert!(!contract.published, "{contract:#?}");
    assert!(!contract.perfect_refactor_claim_allowed, "{contract:#?}");
    assert!(contract.blockers.iter().any(|blocker| {
        blocker.category == "publication-authority"
            && blocker.source_artifact == "target/release-gates/publication-authority.json"
    }));
}

#[test]
fn test_20_2_3_claim_blockers_include_source_artifacts_and_runtime_wiring() {
    /* TEST-20.2.3 */
    let contract = build_perfect_refactor_claim_contract(blocked_inputs());

    for blocker in &contract.blockers {
        assert!(!blocker.item_id.trim().is_empty(), "{blocker:#?}");
        assert!(!blocker.category.trim().is_empty(), "{blocker:#?}");
        assert!(
            blocker.source_artifact.starts_with("target/release-gates/"),
            "{blocker:#?}"
        );
        assert!(!blocker.required_decision.trim().is_empty(), "{blocker:#?}");
    }

    let path = std::env::temp_dir().join(format!(
        "promptfoo-rs-perfect-refactor-claim-{}.json",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&path);
    write_perfect_refactor_claim_contract(&contract, Path::new(&path))
        .expect("claim contract should write");
    let json: Value =
        serde_json::from_str(&std::fs::read_to_string(&path).expect("claim should be readable"))
            .expect("claim should be valid json");
    let _ = std::fs::remove_file(&path);
    assert_eq!(json["perfect_refactor_claim_allowed"], false);
    assert!(json["blockers"].is_array());

    let runtime_script =
        std::fs::read_to_string("scripts/release/runtime-smoke.sh").expect("runtime smoke exists");
    assert!(runtime_script.contains("perfect-refactor-claim.json"));
    assert!(runtime_script.contains("perfect_refactor_claim_allowed"));
    assert!(runtime_script.contains("\"perfect_refactor_claim\""));
}

#[test]
fn test_20_2_4_docs_state_local_stable_vs_perfect_refactor_boundary() {
    /* TEST-20.2.4 */
    let release_docs =
        std::fs::read_to_string("docs/release.md").expect("release docs should exist");
    let matrix =
        std::fs::read_to_string("docs/compatibility/matrix.md").expect("matrix should exist");
    let audit = std::fs::read_to_string(
        "docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md",
    )
    .expect("audit should exist");

    for docs in [release_docs, matrix, audit] {
        assert!(docs.contains("Task 20.2"), "{docs}");
        assert!(docs.contains("perfect-refactor-claim.json"), "{docs}");
        assert!(
            docs.contains("perfect_refactor_claim_allowed=false"),
            "{docs}"
        );
        assert!(docs.contains("local stable"), "{docs}");
        assert!(
            !docs.contains("stable_allowed=true means perfect"),
            "{docs}"
        );
    }
}

fn blocked_inputs() -> PerfectRefactorClaimInputs {
    PerfectRefactorClaimInputs {
        local_stable_allowed: true,
        published: false,
        source_p0_accounting_blocker_count: 22,
        current_perfect_claim_allowed: false,
        publication_ready: PublicationReadiness::CredentialBlocked,
        external_authority_status: "blocked".to_string(),
        external_authority_blocker_count: 21,
        source_artifacts: vec![
            "target/release-gates/source-inventory-evidence.json".to_string(),
            "target/release-gates/current-upstream-policy.json".to_string(),
            "target/release-gates/publication-authority.json".to_string(),
            "target/release-gates/external-authority-blockers.json".to_string(),
            "target/release-gates/release-candidate.json".to_string(),
        ],
    }
}
