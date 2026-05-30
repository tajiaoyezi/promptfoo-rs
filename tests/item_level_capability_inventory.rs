use std::collections::BTreeSet;
use std::path::Path;

use promptfoo_rs::compatibility::inventory::{
    extract_upstream_inventory, validate_inventory_completeness, CapabilityInventory,
    InventoryItem, SnapshotItem, UpstreamSnapshot,
};

const REQUIRED_CATEGORIES: &[&str] = &[
    "command",
    "flag",
    "provider",
    "assertion",
    "redteam-plugin",
    "redteam-strategy",
    "output",
    "config",
    "node-api",
    "viewer",
    "release",
];

#[test]
fn test_11_2_1_inventory_covers_required_upstream_surfaces() {
    /* TEST-11.2.1 */
    let inventory = CapabilityInventory::from_json_file(Path::new(
        "compatibility/inventory/upstream-items.json",
    ))
    .expect("inventory json should parse");

    let categories: BTreeSet<_> = inventory
        .items
        .iter()
        .map(|item| item.category.as_str())
        .collect();

    for category in REQUIRED_CATEGORIES {
        assert!(
            categories.contains(category),
            "missing category {category}; present={categories:#?}"
        );
    }
}

#[test]
fn test_11_2_2_every_item_has_stable_id_source_level_status_and_owner() {
    /* TEST-11.2.2 */
    let inventory = CapabilityInventory::from_json_file(Path::new(
        "compatibility/inventory/upstream-items.json",
    ))
    .expect("inventory json should parse");

    for item in &inventory.items {
        assert_eq!(
            item.stable_id,
            InventoryItem::stable_id(&item.category, &item.name)
        );
        assert!(
            !item.source_reference.trim().is_empty(),
            "{} lacks source reference",
            item.stable_id
        );
        assert!(
            matches!(item.level_hint.as_str(), "P0" | "P1" | "P2"),
            "{} has invalid level {}",
            item.stable_id,
            item.level_hint
        );
        assert!(
            !item.status.trim().is_empty(),
            "{} lacks status",
            item.stable_id
        );
        assert!(
            !item.owner_hint.trim().is_empty(),
            "{} lacks owner",
            item.stable_id
        );
    }
}

#[test]
fn test_11_2_3_unresolved_bucket_is_release_blocking() {
    /* TEST-11.2.3 */
    let snapshot = UpstreamSnapshot {
        source_ref: "promptfoo@945fda5d965ed27abb302fe0f0910b7dddea5dde".to_string(),
        items: vec![SnapshotItem {
            category: "provider".to_string(),
            name: "unknown-dynamic-registry-provider".to_string(),
            source_reference: "src/providers/registry.ts".to_string(),
            level_hint: "P2".to_string(),
            status: "unresolved".to_string(),
            owner_hint: "compatibility".to_string(),
            unresolved_reason: Some("dynamic registry entry requires manual review".to_string()),
        }],
    };

    let inventory = extract_upstream_inventory(&snapshot).expect("snapshot should extract");
    let report = validate_inventory_completeness(&inventory);

    assert!(report
        .unresolved_items
        .contains(&"provider:unknown-dynamic-registry-provider".to_string()));
    assert_eq!(report.release_blocking_unresolved, 1);
    assert!(!report.is_complete());
}
