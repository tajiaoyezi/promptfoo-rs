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
    _snapshot: &UpstreamSnapshot,
) -> Result<CapabilityInventory, InventoryError> {
    unimplemented!("task-11.2 RED skeleton: upstream inventory extractor is not implemented")
}

pub fn validate_inventory_completeness(_inventory: &CapabilityInventory) -> InventoryReport {
    unimplemented!("task-11.2 RED skeleton: inventory completeness validator is not implemented")
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
