use serde::{Deserialize, Serialize};

use crate::compatibility::fixtures::{FixtureCorpus, ProviderMocking};
use crate::compatibility::inventory::{CapabilityInventory, InventoryItem};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompatibilityLevel {
    P0,
    P1,
    P2,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryItem {
    pub id: String,
    pub level: CompatibilityLevel,
    pub notes: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedteamRegistry {
    pub plugins: Vec<RegistryItem>,
    pub strategies: Vec<RegistryItem>,
}

impl RedteamRegistry {
    pub fn core_defaults() -> Self {
        Self {
            plugins: vec![
                item(
                    "prompt-injection",
                    CompatibilityLevel::P0,
                    "core jailbreak coverage",
                ),
                item(
                    "harmful-content",
                    CompatibilityLevel::P1,
                    "snapshot-only policy coverage",
                ),
                item(
                    "custom-policy",
                    CompatibilityLevel::P2,
                    "P2 reason: custom policy registry is project-specific",
                ),
            ],
            strategies: vec![
                item(
                    "jailbreak",
                    CompatibilityLevel::P0,
                    "single-turn jailbreak strategy",
                ),
                item(
                    "multi-turn",
                    CompatibilityLevel::P1,
                    "recorded multi-turn snapshot",
                ),
                item(
                    "agentic-chain",
                    CompatibilityLevel::P2,
                    "P2 reason: long-running agentic chains are deferred",
                ),
            ],
        }
    }

    pub fn plugins_by_level(&self, level: CompatibilityLevel) -> Vec<&RegistryItem> {
        self.plugins
            .iter()
            .filter(|plugin| plugin.level == level)
            .collect()
    }

    pub fn strategies_by_level(&self, level: CompatibilityLevel) -> Vec<&RegistryItem> {
        self.strategies
            .iter()
            .filter(|strategy| strategy.level == level)
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RedteamInventoryCoverage {
    items: Vec<RedteamCoverageItem>,
}

impl RedteamInventoryCoverage {
    pub fn from_registry(registry: &RedteamRegistry, inventory: &CapabilityInventory) -> Self {
        let items = inventory
            .items
            .iter()
            .filter(|item| is_redteam_category(&item.category))
            .map(|item| coverage_item(registry, item))
            .collect();
        Self { items }
    }

    pub fn plugin_items(&self) -> Vec<&RedteamCoverageItem> {
        self.items
            .iter()
            .filter(|item| item.category == "redteam-plugin")
            .collect()
    }

    pub fn strategy_items(&self) -> Vec<&RedteamCoverageItem> {
        self.items
            .iter()
            .filter(|item| item.category == "redteam-strategy")
            .collect()
    }

    pub fn status_for(&self, stable_id: &str) -> Option<&str> {
        self.item(stable_id)
            .map(|item| item.registry_status.as_str())
    }

    pub fn reason_for(&self, stable_id: &str) -> Option<&str> {
        self.item(stable_id).and_then(|item| item.reason.as_deref())
    }

    pub fn item(&self, stable_id: &str) -> Option<&RedteamCoverageItem> {
        self.items.iter().find(|item| item.stable_id == stable_id)
    }

    pub fn items(&self) -> &[RedteamCoverageItem] {
        &self.items
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RedteamCoverageItem {
    pub stable_id: String,
    pub category: String,
    pub name: String,
    pub level: String,
    pub registry_status: String,
    pub reason: Option<String>,
    pub source_reference: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RedteamParityReport {
    pub missing_matrix_rows: Vec<String>,
    pub missing_registry_status: Vec<String>,
    pub silent_omissions: Vec<String>,
    pub p0_missing_fixture_or_blocker: Vec<String>,
    pub p0_fixtures_requiring_real_secrets: Vec<String>,
    pub unsafe_fixture_content: Vec<String>,
    pub p0_redteam_fixture_count: usize,
    pub p2_or_later_missing_reason: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GapClass {
    Later,
    Unsupported,
    Blocked,
}

pub fn validate_redteam_parity(
    coverage: &RedteamInventoryCoverage,
    fixtures: &FixtureCorpus,
) -> RedteamParityReport {
    let mut report = RedteamParityReport {
        p0_fixtures_requiring_real_secrets: fixtures.fixtures_requiring_real_secrets(),
        p0_redteam_fixture_count: fixtures.p0_fixture_item_count_for_prefix("redteam-"),
        ..RedteamParityReport::default()
    };

    for item in coverage.items() {
        if item.stable_id.trim().is_empty() {
            report
                .missing_matrix_rows
                .push("<empty redteam item>".to_string());
        }
        if item.registry_status.trim().is_empty() {
            report.missing_registry_status.push(item.stable_id.clone());
        }
        if item.registry_status == "missing" {
            report.silent_omissions.push(item.stable_id.clone());
        }
        if item.level == "P0"
            && !fixtures.has_p0_fixture_for(&item.stable_id)
            && item.registry_status != "blocked"
        {
            report
                .p0_missing_fixture_or_blocker
                .push(item.stable_id.clone());
        }
        if matches!(
            item.registry_status.as_str(),
            "later" | "unsupported" | "blocked"
        ) && item.reason.as_deref().unwrap_or("").trim().is_empty()
        {
            report
                .p2_or_later_missing_reason
                .push(item.stable_id.clone());
        }
    }

    for record in fixtures.records() {
        let is_redteam = record
            .manifest
            .matrix_item_ids
            .iter()
            .any(|id| id.starts_with("redteam-"));
        if is_redteam
            && (!record.manifest.blocks_stable_release
                || record.manifest.provider_mocking != ProviderMocking::Mock
                || record.manifest.expected_outputs.is_empty())
        {
            report
                .unsafe_fixture_content
                .push(record.manifest.id.clone());
        }
    }
    for invalid in fixtures.invalid_fixtures() {
        if invalid.contains("redteam") {
            report.unsafe_fixture_content.push(invalid.clone());
        }
    }

    report
}

pub fn redteam_gap_user_message(item: &InventoryItem, classification: GapClass) -> String {
    let class = match classification {
        GapClass::Later => "later",
        GapClass::Unsupported => "unsupported",
        GapClass::Blocked => "blocked",
    };
    format!(
        "redteam item {} is classified as {class} in the compatibility matrix; promptfoo-rs will not silently execute unsupported redteam behavior. Source: {}",
        item.stable_id, item.source_reference
    )
}

fn item(id: &str, level: CompatibilityLevel, notes: &str) -> RegistryItem {
    RegistryItem {
        id: id.to_string(),
        level,
        notes: notes.to_string(),
    }
}

fn coverage_item(registry: &RedteamRegistry, item: &InventoryItem) -> RedteamCoverageItem {
    let registry_item = registry_lookup(registry, &item.category, &item.name);
    let (registry_status, reason) = match registry_item {
        Some(registry_item) => match registry_item.level {
            CompatibilityLevel::P0 => ("native".to_string(), Some(registry_item.notes.clone())),
            CompatibilityLevel::P1 => ("snapshot".to_string(), Some(registry_item.notes.clone())),
            CompatibilityLevel::P2 => (
                "later".to_string(),
                Some(format!("later: {}", registry_item.notes)),
            ),
        },
        None if item.level_hint == "P0" => (
            "blocked".to_string(),
            Some(format!(
                "P0 redteam item requires fixture or blocker before stable release: {}",
                item.source_reference
            )),
        ),
        None => (
            "later".to_string(),
            Some(format!(
                "{} redteam item registered as later until native registry behavior is implemented: {}",
                item.level_hint, item.source_reference
            )),
        ),
    };

    RedteamCoverageItem {
        stable_id: item.stable_id.clone(),
        category: item.category.clone(),
        name: item.name.clone(),
        level: item.level_hint.clone(),
        registry_status,
        reason,
        source_reference: item.source_reference.clone(),
    }
}

fn registry_lookup<'a>(
    registry: &'a RedteamRegistry,
    category: &str,
    name: &str,
) -> Option<&'a RegistryItem> {
    match category {
        "redteam-plugin" => registry.plugins.iter().find(|item| item.id == name),
        "redteam-strategy" => registry.strategies.iter().find(|item| item.id == name),
        _ => None,
    }
}

fn is_redteam_category(category: &str) -> bool {
    matches!(category, "redteam-plugin" | "redteam-strategy")
}
