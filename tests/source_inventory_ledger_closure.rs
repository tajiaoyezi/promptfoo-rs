use std::path::Path;

use promptfoo_rs::compatibility::inventory::{
    build_source_accounting_ledger, write_source_accounting_ledger, FrozenSourceReference,
    SourceInventoryExtractor,
};
use promptfoo_rs::compatibility::matrix::{CapabilityMatrix, CapabilityRow};

fn synthetic_source() -> FrozenSourceReference {
    FrozenSourceReference::new(
        "0.121.13",
        "refs/tags/0.121.13",
        "4860e990c7e9a2f8f677173fb92cf9867b34d03f",
        "sha512-DBPSixUophzcD7S7lML6SqVwnVtrhK5A3HsZ03IG9Xrw0t24r5imG7nLj+YMb0vlAjbdFtE7yFG+rsqDpfYp6g==",
        "git ls-tree -r --name-only refs/tags/0.121.13",
    )
    .with_source_files(vec![
        "src/providers/openai/chat.ts",
        "src/assertions/answerRelevance.ts",
        "src/app/src/App.tsx",
        "examples/basic/promptfooconfig.yaml",
        "src/config/default.ts",
    ])
}

fn explicit_matrix() -> CapabilityMatrix {
    CapabilityMatrix {
        rows: vec![CapabilityRow {
            capability: "provider:src-providers-openai-chat".to_string(),
            level: "P0".to_string(),
            target_status: "native".to_string(),
            verification: "fixture:provider:src-providers-openai-chat".to_string(),
            owner: "provider-runtime".to_string(),
            notes: "reason: explicit native fixture row from test".to_string(),
        }],
    }
}

#[test]
fn test_18_1_1_source_accounting_ledger_represents_every_extracted_item() {
    /* TEST-18.1.1 */
    let extracted =
        SourceInventoryExtractor::extract(&synthetic_source()).expect("source should extract");
    let ledger = build_source_accounting_ledger(&extracted, &explicit_matrix());

    assert_eq!(ledger.source_extracted_item_count, extracted.items.len());
    assert_eq!(ledger.ledger_item_count, extracted.items.len());
    assert!(ledger.unrepresented_items().is_empty(), "{ledger:#?}");
    assert!(ledger
        .rows
        .iter()
        .any(|row| row.item_id == "provider:src-providers-openai-chat" && !row.generated));
    assert!(ledger
        .rows
        .iter()
        .any(|row| { row.item_id == "assertion:src-assertions-answerrelevance" && row.generated }));
}

#[test]
fn test_18_1_2_generated_p0_rows_remain_release_blockers() {
    /* TEST-18.1.2 */
    let extracted =
        SourceInventoryExtractor::extract(&synthetic_source()).expect("source should extract");
    let ledger = build_source_accounting_ledger(&extracted, &CapabilityMatrix { rows: vec![] });

    let blockers = ledger.p0_blockers();
    assert!(
        blockers.contains(&"provider:src-providers-openai-chat".to_string()),
        "{ledger:#?}"
    );
    assert!(
        blockers.contains(&"config:src-config-default".to_string()),
        "{ledger:#?}"
    );
    assert!(ledger.rows.iter().any(|row| {
        row.item_id == "provider:src-providers-openai-chat"
            && row.generated
            && row.verification == "blocker:provider:src-providers-openai-chat"
            && row.target_status == "blocked"
            && row.reason.contains("generated P0 accounting row")
    }));
}

#[test]
fn test_18_1_3_generated_non_p0_rows_have_snapshot_or_registration_evidence() {
    /* TEST-18.1.3 */
    let extracted =
        SourceInventoryExtractor::extract(&synthetic_source()).expect("source should extract");
    let ledger = build_source_accounting_ledger(&extracted, &CapabilityMatrix { rows: vec![] });

    let assertion = ledger
        .rows
        .iter()
        .find(|row| row.item_id == "assertion:src-assertions-answerrelevance")
        .expect("assertion ledger row should exist");
    assert_eq!(assertion.level, "P1");
    assert_eq!(
        assertion.verification,
        "snapshot:assertion:src-assertions-answerrelevance"
    );
    assert_eq!(assertion.target_status, "later");
    assert!(assertion.reason.contains("promptfoo@0.121.13:"));

    let example = ledger
        .rows
        .iter()
        .find(|row| row.item_id == "example:examples-basic-promptfooconfig")
        .expect("example ledger row should exist");
    assert_eq!(example.level, "P2");
    assert_eq!(
        example.verification,
        "registration:example:examples-basic-promptfooconfig"
    );
}

#[test]
fn test_18_1_4_source_accounting_ledger_writes_release_gate_json() {
    /* TEST-18.1.4 */
    let extracted =
        SourceInventoryExtractor::extract(&synthetic_source()).expect("source should extract");
    let ledger = build_source_accounting_ledger(&extracted, &explicit_matrix());
    let path = std::env::temp_dir().join(format!(
        "promptfoo-rs-source-inventory-ledger-{}.json",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&path);

    write_source_accounting_ledger(&ledger, Path::new(&path)).expect("ledger should write json");
    let json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&path).expect("ledger should be readable"))
            .expect("ledger should be valid json");
    let _ = std::fs::remove_file(&path);

    assert_eq!(json["schema"], "promptfoo-rs.source-inventory-ledger.v1");
    assert_eq!(json["unrepresented_item_count"], 0);
    assert!(
        json["p0_blocker_count"]
            .as_u64()
            .expect("p0 blocker count should be numeric")
            >= 1
    );
}
