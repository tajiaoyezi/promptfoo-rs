use promptfoo_rs::compatibility::inventory::{
    build_source_accounting_ledger, classify_generated_source_accounting_row,
    is_viewer_config_source_reference, source_accounting_burndown_summary, FrozenSourceReference,
    InventoryItem, SourceExtractedInventory, SourceInventoryCounts,
};
use promptfoo_rs::compatibility::matrix::CapabilityMatrix;

#[test]
fn test_19_1_1_viewer_config_rows_become_p1_viewer_evidence() {
    /* TEST-19.1.1 */
    assert!(is_viewer_config_source_reference(
        "promptfoo@0.121.13:src/app/src/pages/eval/components/ConfigModal.tsx"
    ));
    assert!(is_viewer_config_source_reference(
        "promptfoo@0.121.13:src/app/vite.config.ts"
    ));
    assert!(!is_viewer_config_source_reference(
        "promptfoo@0.121.13:src/util/config/load.ts"
    ));

    let row = classify_generated_source_accounting_row(&viewer_config_item(
        "config:src-app-src-pages-eval-components-configmodal",
        "promptfoo@0.121.13:src/app/src/pages/eval/components/ConfigModal.tsx",
    ));

    assert_eq!(row.level, "P1", "{row:#?}");
    assert_eq!(row.target_status, "later", "{row:#?}");
    assert_eq!(row.owner, "web-viewer", "{row:#?}");
    assert!(row.verification.starts_with("viewer:"), "{row:#?}");
    assert!(row.reason.contains("Local Web viewer"), "{row:#?}");
    assert!(row.reason.contains("P1"), "{row:#?}");
}

#[test]
fn test_19_1_2_non_app_config_rows_remain_p0_blockers() {
    /* TEST-19.1.2 */
    for source_reference in [
        "promptfoo@0.121.13:src/commands/config.ts",
        "promptfoo@0.121.13:src/util/config/load.ts",
        "promptfoo@0.121.13:src/configTypes.ts",
    ] {
        let row = classify_generated_source_accounting_row(&viewer_config_item(
            "config:non-app-runtime-config",
            source_reference,
        ));
        assert_eq!(row.level, "P0", "{row:#?}");
        assert!(
            row.target_status == "blocked" || row.target_status == "native",
            "{row:#?}"
        );
        assert!(
            row.verification.starts_with("blocker:") || row.verification.starts_with("fixture:"),
            "{row:#?}"
        );
        assert!(!row.reason.contains("Local Web viewer"), "{row:#?}");
    }
}

#[test]
fn test_19_1_3_burndown_summary_counts_viewer_reclassification_and_remaining_p0() {
    /* TEST-19.1.3 */
    let inventory = inventory_with_items(vec![
        viewer_config_item(
            "config:src-app-src-pages-eval-components-configmodal",
            "promptfoo@0.121.13:src/app/src/pages/eval/components/ConfigModal.tsx",
        ),
        viewer_config_item(
            "config:src-app-vite-config",
            "promptfoo@0.121.13:src/app/vite.config.ts",
        ),
        viewer_config_item(
            "config:src-globalconfig-cloud",
            "promptfoo@0.121.13:src/globalConfig/cloud.ts",
        ),
    ]);
    let ledger = build_source_accounting_ledger(&inventory, &CapabilityMatrix { rows: vec![] });
    let summary = source_accounting_burndown_summary(&ledger);

    assert_eq!(summary.viewer_config_reclassified_count, 2, "{summary:#?}");
    assert_eq!(summary.p0_accounting_blocker_count, 1, "{summary:#?}");
    assert_eq!(
        summary.remaining_p0_blockers,
        vec!["config:src-globalconfig-cloud".to_string()],
        "{summary:#?}"
    );
    assert_eq!(ledger.p0_blocker_count, 1, "{ledger:#?}");
}

#[test]
fn test_19_1_4_docs_and_script_describe_scope_correction_not_parity_claim() {
    /* TEST-19.1.4 */
    let script = std::fs::read_to_string("scripts/release/source-inventory-evidence.sh")
        .expect("source inventory script should exist");
    assert!(
        script.contains("viewer_config_reclassified_count"),
        "{script}"
    );
    assert!(script.contains("src/app/"), "{script}");

    let matrix =
        std::fs::read_to_string("docs/compatibility/matrix.md").expect("matrix should exist");
    let audit = std::fs::read_to_string(
        "docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md",
    )
    .expect("audit should exist");
    for docs in [matrix, audit] {
        assert!(docs.contains("Task 19.1"), "{docs}");
        assert!(docs.contains("viewer config"), "{docs}");
        assert!(docs.contains("scope correction"), "{docs}");
        assert!(!docs.contains("React UI parity complete"), "{docs}");
    }
}

fn viewer_config_item(item_id: &str, source_reference: &str) -> InventoryItem {
    InventoryItem {
        stable_id: item_id.to_string(),
        category: "config".to_string(),
        name: item_id.to_string(),
        source_reference: source_reference.to_string(),
        level_hint: "P0".to_string(),
        status: "discovered".to_string(),
        owner_hint: "config".to_string(),
        unresolved_reason: Some(
            "source-extracted item was not present in explicit item-level matrix".to_string(),
        ),
    }
}

fn inventory_with_items(items: Vec<InventoryItem>) -> SourceExtractedInventory {
    SourceExtractedInventory {
        baseline: FrozenSourceReference {
            package_version: "0.121.13".to_string(),
            git_ref: "promptfoo@0.121.13".to_string(),
            git_commit: "4860e990c7e9a2f8f677173fb92cf9867b34d03f".to_string(),
            npm_integrity: "sha512-DBPSixUophzcD7S7lML6SqVwnVtrhK5A3HsZ03IG9Xrw0t24r5imG7nLj+YMb0vlAjbdFtE7yFG+rsqDpfYp6g=="
                .to_string(),
            acquisition_command:
                "npm view promptfoo@0.121.13 dist.integrity gitHead --json".to_string(),
            source_files: vec!["src/app/vite.config.ts".to_string()],
        },
        extraction_timestamp: "2026-05-31T00:00:00Z".to_string(),
        source_counts: SourceInventoryCounts::default(),
        items,
    }
}
