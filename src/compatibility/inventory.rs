use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityInventory {
    pub items: Vec<InventoryItem>,
}

impl CapabilityInventory {
    pub fn from_json_file(path: &Path) -> Result<Self, InventoryError> {
        let json = fs::read_to_string(path).map_err(InventoryError::Read)?;
        serde_json::from_str(&json).map_err(|error| InventoryError::Parse(error.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventoryItem {
    pub stable_id: String,
    pub category: String,
    pub name: String,
    pub source_reference: String,
    pub level_hint: String,
    pub status: String,
    pub owner_hint: String,
    pub unresolved_reason: Option<String>,
}

impl InventoryItem {
    pub fn stable_id(category: &str, name: &str) -> String {
        let category = slug(category);
        let name = slug(name);
        format!("{category}:{name}")
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamSnapshot {
    pub source_ref: String,
    pub items: Vec<SnapshotItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotItem {
    pub category: String,
    pub name: String,
    pub source_reference: String,
    pub level_hint: String,
    pub status: String,
    pub owner_hint: String,
    pub unresolved_reason: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InventoryReport {
    pub missing_categories: Vec<String>,
    pub items_missing_metadata: Vec<String>,
    pub unresolved_items: Vec<String>,
    pub release_blocking_unresolved: usize,
}

impl InventoryReport {
    pub fn is_complete(&self) -> bool {
        self.missing_categories.is_empty()
            && self.items_missing_metadata.is_empty()
            && self.release_blocking_unresolved == 0
    }
}

#[derive(Debug)]
pub enum InventoryError {
    Read(std::io::Error),
    Parse(String),
}

pub fn extract_upstream_inventory(
    snapshot: &UpstreamSnapshot,
) -> Result<CapabilityInventory, InventoryError> {
    let items = snapshot
        .items
        .iter()
        .map(|item| InventoryItem {
            stable_id: InventoryItem::stable_id(&item.category, &item.name),
            category: item.category.clone(),
            name: item.name.clone(),
            source_reference: if item.source_reference.trim().is_empty() {
                snapshot.source_ref.clone()
            } else {
                item.source_reference.clone()
            },
            level_hint: item.level_hint.clone(),
            status: item.status.clone(),
            owner_hint: item.owner_hint.clone(),
            unresolved_reason: item.unresolved_reason.clone(),
        })
        .collect();
    Ok(CapabilityInventory { items })
}

pub fn validate_inventory_completeness(inventory: &CapabilityInventory) -> InventoryReport {
    let required_categories = [
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

    let mut missing_categories = Vec::new();
    for category in required_categories {
        if !inventory.items.iter().any(|item| item.category == category) {
            missing_categories.push(category.to_string());
        }
    }

    let mut items_missing_metadata = Vec::new();
    let mut unresolved_items = Vec::new();

    for item in &inventory.items {
        let expected_id = InventoryItem::stable_id(&item.category, &item.name);
        let missing_metadata = item.stable_id != expected_id
            || item.source_reference.trim().is_empty()
            || !matches!(item.level_hint.as_str(), "P0" | "P1" | "P2")
            || item.status.trim().is_empty()
            || item.owner_hint.trim().is_empty();
        if missing_metadata {
            items_missing_metadata.push(item.stable_id.clone());
        }
        if item.status == "unresolved" || item.unresolved_reason.is_some() {
            unresolved_items.push(item.stable_id.clone());
        }
    }

    InventoryReport {
        missing_categories,
        items_missing_metadata,
        release_blocking_unresolved: unresolved_items.len(),
        unresolved_items,
    }
}

fn slug(value: &str) -> String {
    value
        .trim()
        .to_ascii_lowercase()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
