use std::collections::BTreeMap;

use super::inventory::{CapabilityInventory, InventoryItem};
use super::matrix::{CapabilityMatrix, CapabilityRow};

pub use super::fixtures::FixtureCorpus;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProviderParityRegistry {
    items: Vec<ParityInventoryItem>,
}

impl ProviderParityRegistry {
    pub fn from_inventory(inventory: &CapabilityInventory) -> Self {
        Self {
            items: parity_items_for_category(inventory, "provider"),
        }
    }

    pub fn items(&self) -> &[ParityInventoryItem] {
        &self.items
    }

    pub fn p0_item_ids(&self) -> Vec<String> {
        p0_item_ids(&self.items)
    }

    pub fn item(&self, stable_id: &str) -> Option<&ParityInventoryItem> {
        self.items.iter().find(|item| item.stable_id == stable_id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssertionParityRegistry {
    items: Vec<ParityInventoryItem>,
}

impl AssertionParityRegistry {
    pub fn from_inventory(inventory: &CapabilityInventory) -> Self {
        Self {
            items: parity_items_for_category(inventory, "assertion"),
        }
    }

    pub fn items(&self) -> &[ParityInventoryItem] {
        &self.items
    }

    pub fn p0_item_ids(&self) -> Vec<String> {
        p0_item_ids(&self.items)
    }

    pub fn item(&self, stable_id: &str) -> Option<&ParityInventoryItem> {
        self.items.iter().find(|item| item.stable_id == stable_id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParityInventoryItem {
    pub stable_id: String,
    pub level: String,
    pub status: String,
    pub owner: String,
    gap_reason: Option<String>,
}

impl ParityInventoryItem {
    pub fn gap_reason(&self) -> Option<&str> {
        self.gap_reason.as_deref()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ParityReport {
    pub provider_matrix_gaps: Vec<String>,
    pub assertion_matrix_gaps: Vec<String>,
    pub provider_rows_missing_metadata: Vec<String>,
    pub assertion_rows_missing_metadata: Vec<String>,
    pub p2_rows_missing_reason: Vec<String>,
    pub p0_missing_fixture_or_blocker: Vec<String>,
    pub p0_fixtures_requiring_real_secrets: Vec<String>,
    pub unclassified_p0_blockers: Vec<String>,
    pub p0_provider_fixture_count: usize,
    pub p0_assertion_fixture_count: usize,
    pub script_boundary_gaps: Vec<String>,
    pub script_boundaries: Vec<ScriptBoundaryPolicy>,
    target_status_by_item: BTreeMap<String, String>,
}

impl ParityReport {
    pub fn target_status_for(&self, item_id: &str) -> Option<&str> {
        self.target_status_by_item.get(item_id).map(String::as_str)
    }

    pub fn script_boundary_for(&self, runtime: &str) -> Option<&ScriptBoundaryPolicy> {
        self.script_boundaries
            .iter()
            .find(|boundary| boundary.runtime == runtime)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScriptBoundaryPolicy {
    pub runtime: String,
    pub default_deny: bool,
    pub explicit_allow_required: bool,
    pub timeout_required: bool,
    pub env_allowlist_required: bool,
    pub redaction_required: bool,
}

pub fn validate_provider_assertion_parity(
    matrix: &CapabilityMatrix,
    fixtures: &FixtureCorpus,
) -> ParityReport {
    let mut report = ParityReport {
        p0_fixtures_requiring_real_secrets: fixtures.fixtures_requiring_real_secrets(),
        p0_provider_fixture_count: fixtures.p0_fixture_item_count_for_prefix("provider:"),
        p0_assertion_fixture_count: fixtures.p0_fixture_item_count_for_prefix("assertion:"),
        script_boundaries: script_boundary_policies(),
        ..ParityReport::default()
    };

    let provider_rows = rows_for_prefix(matrix, "provider:");
    let assertion_rows = rows_for_prefix(matrix, "assertion:");
    if provider_rows.is_empty() {
        report
            .provider_matrix_gaps
            .push("provider:* matrix rows missing".to_string());
    }
    if assertion_rows.is_empty() {
        report
            .assertion_matrix_gaps
            .push("assertion:* matrix rows missing".to_string());
    }

    for row in provider_rows {
        validate_row(row, fixtures, &mut report, RowDomain::Provider);
    }
    for row in assertion_rows {
        validate_row(row, fixtures, &mut report, RowDomain::Assertion);
    }

    for boundary in &report.script_boundaries {
        if !(boundary.default_deny
            && boundary.explicit_allow_required
            && boundary.timeout_required
            && boundary.env_allowlist_required
            && boundary.redaction_required)
        {
            report.script_boundary_gaps.push(boundary.runtime.clone());
        }
    }

    report
}

fn validate_row(
    row: &CapabilityRow,
    fixtures: &FixtureCorpus,
    report: &mut ParityReport,
    domain: RowDomain,
) {
    report
        .target_status_by_item
        .insert(row.capability.clone(), row.target_status.clone());

    if row.level.trim().is_empty()
        || row.target_status.trim().is_empty()
        || row.verification.trim().is_empty()
        || row.owner.trim().is_empty()
    {
        match domain {
            RowDomain::Provider => report
                .provider_rows_missing_metadata
                .push(row.capability.clone()),
            RowDomain::Assertion => report
                .assertion_rows_missing_metadata
                .push(row.capability.clone()),
        }
    }

    if row.level == "P0"
        && !fixtures.has_p0_fixture_for(&row.capability)
        && !row.verification.starts_with("blocker:")
    {
        report
            .p0_missing_fixture_or_blocker
            .push(row.capability.clone());
    }
    if row.level == "P0"
        && row.verification.starts_with("blocker:")
        && !row.notes.to_ascii_lowercase().contains("reason:")
    {
        report.unclassified_p0_blockers.push(row.capability.clone());
    }
    if row.level == "P2" {
        let notes = row.notes.to_ascii_lowercase();
        let target_status = row.target_status.to_ascii_lowercase();
        if !notes.contains("reason:")
            || !(notes.contains("later:") || notes.contains("unsupported:"))
            || !(target_status.contains("later") || target_status.contains("unsupported"))
        {
            report.p2_rows_missing_reason.push(row.capability.clone());
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RowDomain {
    Provider,
    Assertion,
}

fn parity_items_for_category(
    inventory: &CapabilityInventory,
    category: &str,
) -> Vec<ParityInventoryItem> {
    inventory
        .items
        .iter()
        .filter(|item| item.category == category)
        .map(parity_item)
        .collect()
}

fn parity_item(item: &InventoryItem) -> ParityInventoryItem {
    ParityInventoryItem {
        stable_id: item.stable_id.clone(),
        level: item.level_hint.clone(),
        status: item.status.clone(),
        owner: item.owner_hint.clone(),
        gap_reason: item.unresolved_reason.clone(),
    }
}

fn p0_item_ids(items: &[ParityInventoryItem]) -> Vec<String> {
    items
        .iter()
        .filter(|item| item.level == "P0")
        .map(|item| item.stable_id.clone())
        .collect()
}

fn rows_for_prefix<'a>(matrix: &'a CapabilityMatrix, prefix: &str) -> Vec<&'a CapabilityRow> {
    matrix
        .rows
        .iter()
        .filter(|row| row.capability.starts_with(prefix))
        .collect()
}

fn script_boundary_policies() -> Vec<ScriptBoundaryPolicy> {
    ["javascript", "typescript", "python", "shell", "ruby"]
        .into_iter()
        .map(|runtime| ScriptBoundaryPolicy {
            runtime: runtime.to_string(),
            default_deny: true,
            explicit_allow_required: true,
            timeout_required: true,
            env_allowlist_required: true,
            redaction_required: true,
        })
        .collect()
}
