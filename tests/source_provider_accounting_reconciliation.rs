use std::path::Path;
use std::process::Command;

use promptfoo_rs::compatibility::inventory::{
    classify_provider_source_accounting_row, validate_provider_source_accounting_reconciliation,
    write_provider_source_accounting_reconciliation, ProviderSourceAccountingDecision,
    SourceAccountingLedger, SourceAccountingRow,
};
use promptfoo_rs::compatibility::provider_assertion::{
    ProviderModuleBurndownReport, ProviderModuleResolution, ProviderModuleResolutionKind,
};
use serde_json::Value;

#[test]
fn test_20_1_1_fixture_covered_provider_rows_leave_remaining_source_blockers() {
    /* TEST-20.1.1 */
    let ledger = source_ledger(vec![
        provider_blocker_row("provider:src-providers-openai-completion"),
        provider_blocker_row("provider:src-providers-openai-codex-sdk"),
        config_blocker_row("config:src-globalconfig-cloud"),
    ]);
    let provider_report = provider_report();

    let report = validate_provider_source_accounting_reconciliation(&ledger, &provider_report);

    assert_eq!(
        report.schema,
        "promptfoo-rs.provider-source-accounting-reconciliation.v1"
    );
    assert_eq!(report.provider_source_total, 2, "{report:#?}");
    assert_eq!(report.resolved_provider_fixture_count, 1, "{report:#?}");
    assert_eq!(report.provider_external_authority_count, 1, "{report:#?}");
    assert_eq!(
        report.provider_source_generic_blocker_count, 0,
        "{report:#?}"
    );
    assert_eq!(report.source_p0_accounting_blocker_count, 2, "{report:#?}");
    assert_eq!(
        report.remaining_source_p0_blockers,
        vec![
            "config:src-globalconfig-cloud".to_string(),
            "provider:src-providers-openai-codex-sdk".to_string(),
        ],
        "{report:#?}"
    );
    assert!(report.resolved_provider_source_rows.iter().any(|decision| {
        decision.item_id == "provider:src-providers-openai-completion"
            && decision.local_fixture_covered
            && !decision.release_blocking
    }));
}

#[test]
fn test_20_1_2_provider_external_authority_rows_remain_blocking() {
    /* TEST-20.1.2 */
    let provider_report = provider_report();
    let fixture_decision = classify_provider_source_accounting_row(
        &provider_blocker_row("provider:src-providers-openai-completion"),
        &provider_report,
    );
    assert_fixture_decision(&fixture_decision);

    let external_decision = classify_provider_source_accounting_row(
        &provider_blocker_row("provider:src-providers-openai-codex-sdk"),
        &provider_report,
    );
    assert_eq!(
        external_decision.classification, "external-authority-provider",
        "{external_decision:#?}"
    );
    assert_eq!(
        external_decision.target_status, "blocked",
        "{external_decision:#?}"
    );
    assert_eq!(
        external_decision.owner, "external-authority",
        "{external_decision:#?}"
    );
    assert!(
        external_decision.external_authority_required,
        "{external_decision:#?}"
    );
    assert!(external_decision.release_blocking, "{external_decision:#?}");
    assert!(
        external_decision
            .verification
            .starts_with("blocker:provider:"),
        "{external_decision:#?}"
    );
    assert!(
        external_decision.reason.contains("external"),
        "{external_decision:#?}"
    );
}

#[test]
fn test_20_1_3_source_inventory_evidence_reconciles_provider_burndown_counts() {
    /* TEST-20.1.3 */
    let output = Command::new(git_bash())
        .args(["-lc", "bash scripts/release/source-inventory-evidence.sh"])
        .output()
        .expect("source inventory evidence script should execute");
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let evidence: Value = serde_json::from_str(
        &std::fs::read_to_string("target/release-gates/source-inventory-evidence.json")
            .expect("source evidence should exist"),
    )
    .expect("source evidence should be valid JSON");
    let reconciliation = &evidence["provider_source_accounting_reconciliation"];
    assert_eq!(reconciliation["resolved_provider_fixture_count"], 22);
    assert_eq!(reconciliation["provider_external_authority_count"], 15);
    assert_eq!(reconciliation["provider_source_generic_blocker_count"], 0);
    assert_eq!(evidence["p0_accounting_blocker_count"], 22);

    let remaining = evidence["remaining_p0_blockers"]
        .as_array()
        .expect("remaining P0 blockers should be listed");
    let config_remaining = remaining
        .iter()
        .filter(|item| item.as_str().unwrap_or_default().starts_with("config:"))
        .count();
    let provider_remaining = remaining
        .iter()
        .filter(|item| item.as_str().unwrap_or_default().starts_with("provider:"))
        .count();
    assert_eq!(config_remaining, 7, "{remaining:#?}");
    assert_eq!(provider_remaining, 15, "{remaining:#?}");
    assert!(!remaining
        .iter()
        .any(|item| item == "provider:src-providers-openai-completion"));
    assert!(remaining
        .iter()
        .any(|item| item == "provider:src-providers-openai-codex-sdk"));
}

#[test]
fn test_20_1_4_docs_explain_cross_ledger_reconciliation_boundary() {
    /* TEST-20.1.4 */
    let matrix =
        std::fs::read_to_string("docs/compatibility/matrix.md").expect("matrix should exist");
    let audit = std::fs::read_to_string(
        "docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md",
    )
    .expect("audit should exist");

    for docs in [matrix, audit] {
        assert!(docs.contains("Task 20.1"), "{docs}");
        assert!(docs.contains("cross-ledger"), "{docs}");
        assert!(docs.contains("fixture-covered provider rows"), "{docs}");
        assert!(docs.contains("p0_accounting_blocker_count=22"), "{docs}");
        assert!(!docs.contains("all provider modules complete"), "{docs}");
    }
}

fn assert_fixture_decision(decision: &ProviderSourceAccountingDecision) {
    assert_eq!(
        decision.classification, "fixture-covered-provider",
        "{decision:#?}"
    );
    assert_eq!(decision.target_status, "native", "{decision:#?}");
    assert_eq!(decision.owner, "provider-runtime", "{decision:#?}");
    assert!(decision.local_fixture_covered, "{decision:#?}");
    assert!(!decision.external_authority_required, "{decision:#?}");
    assert!(!decision.release_blocking, "{decision:#?}");
    assert!(
        decision.verification.starts_with("fixture:"),
        "{decision:#?}"
    );
    assert!(decision.reason.contains("fixture"), "{decision:#?}");
}

fn provider_report() -> ProviderModuleBurndownReport {
    ProviderModuleBurndownReport {
        initial_blocker_count: 2,
        resolved_by_fixture_count: 1,
        new_dedicated_request_response_fixture_count: 1,
        remaining_blocker_count: 1,
        external_authority_blocker_count: 1,
        generic_blocker_count: 0,
        resolved_by_fixture: vec![ProviderModuleResolution {
            item_id: "provider:src-providers-openai-completion".to_string(),
            source_reference: "promptfoo@0.121.13:src/providers/openai/completion.ts".to_string(),
            kind: ProviderModuleResolutionKind::FixtureCovered,
            reason: "dedicated request/response fixture evidence covers provider row".to_string(),
            verification: "fixture:p0-provider-openai-completion".to_string(),
            fixture_ids: vec!["p0-provider-openai-completion".to_string()],
            docs_link: "docs/compatibility/matrix.md#p0-provider-module-burndown".to_string(),
            requires_external_authority: false,
        }],
        remaining_blockers: vec![ProviderModuleResolution {
            item_id: "provider:src-providers-openai-codex-sdk".to_string(),
            source_reference: "promptfoo@0.121.13:src/providers/openai/codexSdk.ts".to_string(),
            kind: ProviderModuleResolutionKind::ExternalBlocker,
            reason: "OpenAI Codex provider modules require external product authority".to_string(),
            verification: "blocker:provider:src-providers-openai-codex-sdk".to_string(),
            fixture_ids: vec![],
            docs_link: "docs/compatibility/matrix.md#p0-provider-module-burndown".to_string(),
            requires_external_authority: true,
        }],
        fixtures_requiring_real_secrets: vec![],
    }
}

fn source_ledger(rows: Vec<SourceAccountingRow>) -> SourceAccountingLedger {
    let p0_blocker_count = rows
        .iter()
        .filter(|row| row.level == "P0" && row.verification.starts_with("blocker:"))
        .count();
    SourceAccountingLedger {
        schema: "promptfoo-rs.source-inventory-ledger.v1".to_string(),
        source_extracted_item_count: rows.len(),
        ledger_item_count: rows.len(),
        unrepresented_item_count: 0,
        p0_blocker_count,
        rows,
        unrepresented_items: vec![],
    }
}

fn provider_blocker_row(item_id: &str) -> SourceAccountingRow {
    SourceAccountingRow {
        item_id: item_id.to_string(),
        category: "provider".to_string(),
        source_reference: format!(
            "promptfoo@0.121.13:{}",
            item_id
                .trim_start_matches("provider:")
                .replace("src-providers-", "src/providers/")
                .replace('-', "/")
        ),
        level: "P0".to_string(),
        target_status: "blocked".to_string(),
        owner: "provider-runtime".to_string(),
        verification: format!("blocker:{item_id}"),
        reason: "generated P0 accounting row requires native fixture, bridge fixture, or explicit waiver".to_string(),
        generated: true,
    }
}

fn config_blocker_row(item_id: &str) -> SourceAccountingRow {
    SourceAccountingRow {
        item_id: item_id.to_string(),
        category: "config".to_string(),
        source_reference: "promptfoo@0.121.13:src/globalConfig/cloud.ts".to_string(),
        level: "P0".to_string(),
        target_status: "blocked".to_string(),
        owner: "external-authority".to_string(),
        verification: format!("blocker:{item_id}"),
        reason: "external cloud config requires account authority".to_string(),
        generated: true,
    }
}

fn git_bash() -> &'static str {
    if cfg!(windows) {
        r"C:\Program Files\Git\bin\bash.exe"
    } else {
        "bash"
    }
}

#[allow(dead_code)]
fn write_temp_reconciliation(
    report: &promptfoo_rs::compatibility::inventory::ProviderSourceAccountingReconciliationReport,
) -> Value {
    let path = std::env::temp_dir().join(format!(
        "promptfoo-rs-provider-source-accounting-{}.json",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&path);
    write_provider_source_accounting_reconciliation(report, Path::new(&path))
        .expect("provider source accounting reconciliation should write");
    let json: Value =
        serde_json::from_str(&std::fs::read_to_string(&path).expect("report should be readable"))
            .expect("report should be valid json");
    let _ = std::fs::remove_file(&path);
    json
}
