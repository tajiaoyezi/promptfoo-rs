use std::path::Path;

use promptfoo_rs::compatibility::inventory::{
    validate_source_extracted_inventory, write_source_inventory_evidence, FrozenSourceReference,
    SourceInventoryExtractor, SourceInventoryStatus,
};
use promptfoo_rs::compatibility::matrix::{CapabilityMatrix, CapabilityRow};

fn matrix_with_first_row(capability: &str) -> CapabilityMatrix {
    CapabilityMatrix {
        rows: vec![CapabilityRow {
            capability: capability.to_string(),
            level: "P1".to_string(),
            target_status: "native".to_string(),
            verification: format!("snapshot:{capability}"),
            owner: "compatibility".to_string(),
            notes: "source: synthetic frozen source fixture".to_string(),
        }],
    }
}

fn synthetic_source() -> FrozenSourceReference {
    FrozenSourceReference::new(
        "0.121.13",
        "refs/tags/0.121.13",
        "4860e990c7e9a2f8f677173fb92cf9867b34d03f",
        "sha512-DBPSixUophzcD7S7lML6SqVwnVtrhK5A3HsZ03IG9Xrw0t24r5imG7nLj+YMb0vlAjbdFtE7yFG+rsqDpfYp6g==",
        "git ls-tree -r --name-only refs/tags/0.121.13",
    )
    .with_source_files(vec![
        "src/main.ts",
        "src/commands/eval.ts",
        "src/redteam/commands/run.ts",
        "src/providers/openai/chat.ts",
        "src/providers/azure/index.ts",
        "src/assertions/equals.ts",
        "src/assertions/rouge.ts",
        "src/redteam/plugins/harmful.ts",
        "src/redteam/strategies/jailbreak.ts",
        "src/app/src/App.tsx",
        "examples/basic/promptfooconfig.yaml",
        "src/config/default.ts",
        "src/util/output.ts",
    ])
}

#[test]
fn test_17_1_1_baseline_reference_is_frozen_and_refuses_floating_refs() {
    /* TEST-17.1.1 */
    let source = FrozenSourceReference::from_baseline_lock(Path::new(
        "docs/compatibility/baseline.lock.md",
    ))
    .expect("baseline lock should parse into a frozen source reference");

    assert_eq!(source.package_version, "0.121.13");
    assert_eq!(
        source.git_commit,
        "4860e990c7e9a2f8f677173fb92cf9867b34d03f"
    );
    assert!(source.npm_integrity.starts_with("sha512-"));
    assert!(source.acquisition_command.contains("0.121.13"));
    assert!(source.validate_non_floating().is_ok());

    let floating = FrozenSourceReference::new("latest", "main", "HEAD", "", "git ls-tree HEAD");
    let error = floating
        .validate_non_floating()
        .expect_err("floating refs must be rejected");
    assert!(error.to_string().contains("floating"));
}

#[test]
fn test_17_1_2_extractor_reports_source_visible_counts_and_items() {
    /* TEST-17.1.2 */
    let inventory = SourceInventoryExtractor::extract(&synthetic_source())
        .expect("synthetic frozen source should extract");

    assert_eq!(inventory.source_counts.command_related_files, 3);
    assert_eq!(inventory.source_counts.provider_files, 2);
    assert_eq!(inventory.source_counts.assertion_files, 2);
    assert_eq!(inventory.source_counts.redteam_plugin_files, 1);
    assert_eq!(inventory.source_counts.redteam_strategy_files, 1);
    assert_eq!(inventory.source_counts.viewer_app_files, 1);
    assert_eq!(inventory.source_counts.example_files, 1);
    assert!(inventory
        .items
        .iter()
        .any(|item| item.stable_id == "provider:src-providers-openai-chat"));
    assert!(inventory
        .items
        .iter()
        .any(|item| item.stable_id == "redteam-strategy:src-redteam-strategies-jailbreak"));
}

#[test]
fn test_17_1_3_every_extracted_item_has_matrix_row_or_release_blocker() {
    /* TEST-17.1.3 */
    let inventory = SourceInventoryExtractor::extract(&synthetic_source())
        .expect("synthetic frozen source should extract");
    let matrix = matrix_with_first_row(&inventory.items[0].stable_id);
    let report = validate_source_extracted_inventory(&inventory, &matrix);

    assert_eq!(report.status, SourceInventoryStatus::ReadyWithBlockers);
    assert!(report.items_missing_metadata.is_empty(), "{report:#?}");
    assert!(report.silent_omissions.is_empty(), "{report:#?}");
    assert!(
        report.release_blockers.len() >= inventory.items.len() - 1,
        "{report:#?}"
    );
    assert!(report
        .release_blockers
        .iter()
        .any(|blocker| blocker.reason.contains("missing matrix row")));
}

#[test]
fn test_17_1_4_source_inventory_evidence_records_blockers_and_status() {
    /* TEST-17.1.4 */
    let inventory = SourceInventoryExtractor::extract(&synthetic_source())
        .expect("synthetic frozen source should extract");
    let report = validate_source_extracted_inventory(&inventory, &matrix_with_first_row("none"));

    let path = std::env::temp_dir().join(format!(
        "promptfoo-rs-source-inventory-evidence-{}.json",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&path);
    write_source_inventory_evidence(&report, &path)
        .expect("source inventory evidence should be written");
    let json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&path).expect("evidence should be readable"))
            .expect("evidence should be valid json");
    let _ = std::fs::remove_file(&path);

    assert_eq!(json["schema"], "promptfoo-rs.source-inventory-evidence.v2");
    assert_eq!(json["status"], "ready-with-blockers");
    assert!(json["release_blockers"]
        .as_array()
        .expect("release blockers should be an array")
        .len()
        >= inventory.items.len());
}
