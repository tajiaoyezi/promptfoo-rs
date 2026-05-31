use std::path::Path;

use promptfoo_rs::compatibility::inventory::{
    build_source_accounting_ledger, classify_generated_source_accounting_row,
    classify_non_app_config_source_row, validate_core_config_source_burndown,
    write_core_config_source_burndown, FrozenSourceReference, InventoryItem,
    SourceExtractedInventory, SourceInventoryCounts,
};
use promptfoo_rs::compatibility::matrix::CapabilityMatrix;

#[test]
fn test_19_2_1_runtime_config_rows_have_fixture_evidence() {
    /* TEST-19.2.1 */
    for source_reference in [
        "promptfoo@0.121.13:src/commands/config.ts",
        "promptfoo@0.121.13:src/configTypes.ts",
        "promptfoo@0.121.13:src/util/config/load.ts",
        "promptfoo@0.121.13:src/util/config/default.ts",
    ] {
        let decision = classify_non_app_config_source_row(&generated_config_row(source_reference));

        assert_eq!(decision.classification, "native-fixture", "{decision:#?}");
        assert_eq!(decision.level, "P0", "{decision:#?}");
        assert_eq!(decision.target_status, "native", "{decision:#?}");
        assert_eq!(decision.owner, "config-loader", "{decision:#?}");
        assert!(decision.local_runtime_parity, "{decision:#?}");
        assert!(!decision.external_authority_required, "{decision:#?}");
        assert!(decision.verification.starts_with("fixture:config:"), "{decision:#?}");
        assert!(
            decision
                .fixture_path
                .as_deref()
                .unwrap_or_default()
                .starts_with("compatibility/fixtures/config/"),
            "{decision:#?}"
        );
        assert!(decision.reason.contains("promptfooconfig"), "{decision:#?}");
        assert!(
            !decision
                .reason
                .contains("generated P0 accounting row requires"),
            "{decision:#?}"
        );
    }
}

#[test]
fn test_19_2_2_external_config_rows_remain_explicit_blockers() {
    /* TEST-19.2.2 */
    for source_reference in [
        "promptfoo@0.121.13:src/globalConfig/cloud.ts",
        "promptfoo@0.121.13:src/server/config/serverConfig.ts",
        "promptfoo@0.121.13:src/tracing/otelConfig.ts",
    ] {
        let decision = classify_non_app_config_source_row(&generated_config_row(source_reference));

        assert_eq!(decision.classification, "external-blocker", "{decision:#?}");
        assert_eq!(decision.level, "P0", "{decision:#?}");
        assert_eq!(decision.target_status, "blocked", "{decision:#?}");
        assert_eq!(decision.owner, "external-authority", "{decision:#?}");
        assert!(!decision.local_runtime_parity, "{decision:#?}");
        assert!(decision.external_authority_required, "{decision:#?}");
        assert!(decision.verification.starts_with("blocker:config:"), "{decision:#?}");
        assert!(decision.reason.contains("external"), "{decision:#?}");
        assert!(
            !decision
                .reason
                .contains("generated P0 accounting row requires"),
            "{decision:#?}"
        );
    }
}

#[test]
fn test_19_2_3_burndown_report_counts_specific_config_decisions() {
    /* TEST-19.2.3 */
    let ledger = build_source_accounting_ledger(
        &inventory_with_items(vec![
            config_item(
                "config:src-util-config-load",
                "promptfoo@0.121.13:src/util/config/load.ts",
            ),
            config_item(
                "config:src-redteam-plugins-policy-evals-promptfooconfig",
                "promptfoo@0.121.13:src/redteam/plugins/policy/evals/promptfooconfig.yaml",
            ),
            config_item(
                "config:src-codescan-config-schema",
                "promptfoo@0.121.13:src/codeScan/config/schema.ts",
            ),
            config_item(
                "config:src-globalconfig-cloud",
                "promptfoo@0.121.13:src/globalConfig/cloud.ts",
            ),
            config_item(
                "config:src-app-vite-config",
                "promptfoo@0.121.13:src/app/vite.config.ts",
            ),
        ]),
        &CapabilityMatrix { rows: vec![] },
    );
    let report = validate_core_config_source_burndown(&ledger);

    assert_eq!(report.schema, "promptfoo-rs.core-config-source-burndown.v1");
    assert_eq!(report.non_app_config_total, 4, "{report:#?}");
    assert_eq!(report.non_app_config_fixture_covered_count, 2, "{report:#?}");
    assert_eq!(report.non_app_config_external_blocker_count, 1, "{report:#?}");
    assert_eq!(
        report.non_app_config_auxiliary_registration_count, 1,
        "{report:#?}"
    );
    assert_eq!(report.non_app_config_generic_blocker_count, 0, "{report:#?}");
    assert!(report.decisions.iter().any(|decision| {
        decision.item_id == "config:src-util-config-load"
            && decision.classification == "native-fixture"
    }));
    assert!(report.decisions.iter().any(|decision| {
        decision.item_id == "config:src-globalconfig-cloud"
            && decision.classification == "external-blocker"
            && decision.external_authority_required
    }));
}

#[test]
fn test_19_2_4_no_non_app_config_row_remains_generic_generated_blocker() {
    /* TEST-19.2.4 */
    let ledger = build_source_accounting_ledger(
        &inventory_with_items(vec![
            config_item(
                "config:src-util-config-load",
                "promptfoo@0.121.13:src/util/config/load.ts",
            ),
            config_item(
                "config:src-server-config-serverconfig",
                "promptfoo@0.121.13:src/server/config/serverConfig.ts",
            ),
        ]),
        &CapabilityMatrix { rows: vec![] },
    );
    let report = validate_core_config_source_burndown(&ledger);

    assert_eq!(report.non_app_config_generic_blocker_count, 0, "{report:#?}");
    assert!(report.decisions.iter().all(|decision| {
        !decision
            .reason
            .contains("generated P0 accounting row requires")
    }));

    let path = std::env::temp_dir().join(format!(
        "promptfoo-rs-core-config-source-burndown-{}.json",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&path);
    write_core_config_source_burndown(&report, Path::new(&path)).expect("report should write");
    let json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&path).expect("report should be readable"))
            .expect("report should be valid json");
    let _ = std::fs::remove_file(&path);

    assert_eq!(json["schema"], "promptfoo-rs.core-config-source-burndown.v1");
    assert_eq!(json["non_app_config_generic_blocker_count"], 0);
    assert!(json["decisions"].is_array());

    let script = std::fs::read_to_string("scripts/release/source-inventory-evidence.sh")
        .expect("source inventory script should exist");
    assert!(script.contains("core_config_source_burndown"), "{script}");
    assert!(
        script.contains("non_app_config_fixture_covered_count"),
        "{script}"
    );
    assert!(
        script.contains("non_app_config_generic_blocker_count"),
        "{script}"
    );
}

fn generated_config_row(source_reference: &str) -> promptfoo_rs::compatibility::inventory::SourceAccountingRow {
    classify_generated_source_accounting_row(&config_item(
        &InventoryItem::stable_id("config", source_reference.trim_start_matches("promptfoo@0.121.13:")),
        source_reference,
    ))
}

fn config_item(item_id: &str, source_reference: &str) -> InventoryItem {
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
            source_files: vec!["src/util/config/load.ts".to_string()],
        },
        extraction_timestamp: "2026-05-31T00:00:00Z".to_string(),
        source_counts: SourceInventoryCounts::default(),
        items,
    }
}
