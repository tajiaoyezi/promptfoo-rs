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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParityPolicy {
    pub docs_link: String,
}

impl Default for ParityPolicy {
    fn default() -> Self {
        Self {
            docs_link: "docs/compatibility/matrix.md".to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LongtailClass {
    Native,
    Bridge,
    Unsupported,
    Later,
    Blocked,
}

impl LongtailClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Bridge => "bridge",
            Self::Unsupported => "unsupported",
            Self::Later => "later",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LongtailClassification {
    pub item_id: String,
    pub class: LongtailClass,
    pub reason: String,
    pub owner: String,
    pub verification: String,
}

pub type ProviderClassification = LongtailClassification;
pub type AssertionClassification = LongtailClassification;
pub type RedteamClassification = LongtailClassification;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GapClass {
    Later,
    Unsupported,
    Blocked,
}

impl GapClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Later => "later",
            Self::Unsupported => "unsupported",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompatibilityError {
    item_id: String,
    class: GapClass,
    message: String,
    exit_code: i32,
}

impl CompatibilityError {
    pub fn exit_code(&self) -> i32 {
        self.exit_code
    }

    pub fn item_id(&self) -> &str {
        &self.item_id
    }

    pub fn class(&self) -> GapClass {
        self.class
    }
}

impl std::fmt::Display for CompatibilityError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for CompatibilityError {}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LongtailParityReport {
    pub classified_item_count: usize,
    pub classified_by_category: BTreeMap<String, usize>,
    pub missing_classification: Vec<String>,
    pub rows_missing_owner: Vec<String>,
    pub rows_missing_verification: Vec<String>,
    pub rows_missing_reason: Vec<String>,
    pub unresolved_rows: Vec<String>,
    pub p0_missing_fixture_or_blocker: Vec<String>,
    pub p0_release_blocker_count: usize,
    pub p1_missing_snapshot_plan: Vec<String>,
    pub p2_or_later_missing_reason: Vec<String>,
    pub script_boundary_gaps: Vec<String>,
    pub script_boundaries: Vec<ScriptBoundaryPolicy>,
}

impl LongtailParityReport {
    pub fn script_boundary_for(&self, runtime: &str) -> Option<&ScriptBoundaryPolicy> {
        self.script_boundaries
            .iter()
            .find(|boundary| boundary.runtime == runtime)
    }
}

pub fn classify_provider_item(
    item: &InventoryItem,
    policy: &ParityPolicy,
) -> ProviderClassification {
    classify_inventory_item(item, "provider", policy)
}

pub fn classify_assertion_item(
    item: &InventoryItem,
    policy: &ParityPolicy,
) -> AssertionClassification {
    classify_inventory_item(item, "assertion", policy)
}

pub fn classify_redteam_item(item: &InventoryItem, policy: &ParityPolicy) -> RedteamClassification {
    classify_inventory_item(item, "redteam", policy)
}

pub fn compatibility_gap_error(item_id: &str, class: GapClass, reason: &str) -> CompatibilityError {
    let reason = redact_sensitive_assignments(reason);
    CompatibilityError {
        item_id: item_id.to_string(),
        class,
        message: format!(
            "{item_id}: {} compatibility gap; reason: {reason}; docs: docs/compatibility/matrix.md",
            class.as_str()
        ),
        exit_code: 1,
    }
}

pub fn validate_longtail_classification(
    matrix: &CapabilityMatrix,
    fixtures: &FixtureCorpus,
) -> LongtailParityReport {
    let mut report = LongtailParityReport {
        script_boundaries: script_boundary_policies(),
        ..LongtailParityReport::default()
    };

    for row in matrix
        .rows
        .iter()
        .filter(|row| is_longtail_row(&row.capability))
    {
        report.classified_item_count += 1;
        let category = row
            .capability
            .split_once(':')
            .map(|(category, _)| category.to_string())
            .unwrap_or_else(|| "<unknown>".to_string());
        *report.classified_by_category.entry(category).or_default() += 1;

        let Some(class) = longtail_class_from_status(&row.target_status) else {
            report.missing_classification.push(row.capability.clone());
            continue;
        };

        if row.owner.trim().is_empty() {
            report.rows_missing_owner.push(row.capability.clone());
        }
        if row.verification.trim().is_empty() {
            report
                .rows_missing_verification
                .push(row.capability.clone());
        }
        if row
            .target_status
            .to_ascii_lowercase()
            .contains("unresolved")
            || row
                .notes
                .to_ascii_lowercase()
                .contains("task-17.4 classification")
        {
            report.unresolved_rows.push(row.capability.clone());
        }

        if requires_reason(row, class) && !row_has_reason_for_class(row, class) {
            report.rows_missing_reason.push(row.capability.clone());
        }

        if row.level == "P0"
            && !fixtures.has_p0_fixture_for(&row.capability)
            && !row.verification.starts_with("blocker:")
        {
            report
                .p0_missing_fixture_or_blocker
                .push(row.capability.clone());
        }
        if row.level == "P0" && row.verification.starts_with("blocker:") {
            report.p0_release_blocker_count += 1;
        }
        if row.level == "P1" && !row.verification.starts_with("snapshot:") {
            report.p1_missing_snapshot_plan.push(row.capability.clone());
        }
        if (row.level == "P2" || matches!(class, LongtailClass::Later))
            && !row_has_reason_for_class(row, class)
        {
            report
                .p2_or_later_missing_reason
                .push(row.capability.clone());
        }
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
            || !(notes.contains("later:")
                || notes.contains("unsupported:")
                || notes.contains("blocked:"))
            || !(target_status.contains("later")
                || target_status.contains("unsupported")
                || target_status.contains("blocked"))
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

fn classify_inventory_item(
    item: &InventoryItem,
    domain: &str,
    policy: &ParityPolicy,
) -> LongtailClassification {
    let status = item.status.to_ascii_lowercase();
    let owner = item.owner_hint.to_ascii_lowercase();
    let name = item.name.to_ascii_lowercase();
    let class = if status == "blocked" {
        LongtailClass::Blocked
    } else if status == "unsupported" {
        LongtailClass::Unsupported
    } else if status == "later" || status == "unresolved" {
        LongtailClass::Later
    } else if owner == "script-bridge"
        || ["javascript", "typescript", "python", "shell", "ruby"]
            .iter()
            .any(|runtime| name.contains(runtime))
    {
        LongtailClass::Bridge
    } else if item.level_hint == "P2"
        || (domain == "redteam" && item.level_hint != "P0")
        || item.unresolved_reason.is_some()
    {
        LongtailClass::Later
    } else {
        LongtailClass::Native
    };

    let reason = match class {
        LongtailClass::Native => format!(
            "native implementation evidence from {}",
            item.source_reference
        ),
        LongtailClass::Bridge => format!(
            "script bridge default-deny compatibility path; docs: {}",
            policy.docs_link
        ),
        LongtailClass::Unsupported | LongtailClass::Later | LongtailClass::Blocked => {
            item.unresolved_reason.clone().unwrap_or_else(|| {
                format!(
                    "{} source item requires explicit compatibility registration",
                    item.stable_id
                )
            })
        }
    };
    let verification = match (item.level_hint.as_str(), class) {
        ("P0", LongtailClass::Blocked) => format!("blocker:{}", item.stable_id),
        ("P0", _) => format!("fixture:{}", item.stable_id),
        ("P1", _) => format!("snapshot:{}", item.stable_id),
        ("P2", _) => format!("registration:{}", item.stable_id),
        _ => format!("blocker:{}", item.stable_id),
    };

    LongtailClassification {
        item_id: item.stable_id.clone(),
        class,
        reason,
        owner: item.owner_hint.clone(),
        verification,
    }
}

fn longtail_class_from_status(status: &str) -> Option<LongtailClass> {
    let normalized = status.trim().to_ascii_lowercase();
    if normalized.contains("native") {
        Some(LongtailClass::Native)
    } else if normalized.contains("bridge") {
        Some(LongtailClass::Bridge)
    } else if normalized.contains("unsupported") {
        Some(LongtailClass::Unsupported)
    } else if normalized.contains("later") {
        Some(LongtailClass::Later)
    } else if normalized.contains("blocked") {
        Some(LongtailClass::Blocked)
    } else {
        None
    }
}

fn is_longtail_row(capability: &str) -> bool {
    capability.starts_with("provider:")
        || capability.starts_with("assertion:")
        || capability.starts_with("redteam-plugin:")
        || capability.starts_with("redteam-strategy:")
}

fn requires_reason(row: &CapabilityRow, class: LongtailClass) -> bool {
    row.level == "P2"
        || row.verification.starts_with("blocker:")
        || matches!(
            class,
            LongtailClass::Unsupported | LongtailClass::Later | LongtailClass::Blocked
        )
}

fn row_has_reason_for_class(row: &CapabilityRow, class: LongtailClass) -> bool {
    let notes = row.notes.to_ascii_lowercase();
    if !notes.contains("reason:") {
        return false;
    }
    match class {
        LongtailClass::Native | LongtailClass::Bridge => true,
        LongtailClass::Unsupported => notes.contains("unsupported:"),
        LongtailClass::Later => notes.contains("later:"),
        LongtailClass::Blocked => {
            notes.contains("blocked:") || row.verification.starts_with("blocker:")
        }
    }
}

fn redact_sensitive_assignments(input: &str) -> String {
    input
        .split_whitespace()
        .map(|token| {
            let Some((key, _value)) = token.split_once('=') else {
                return token.to_string();
            };
            let key_lower = key.to_ascii_lowercase();
            if key_lower.contains("key")
                || key_lower.contains("token")
                || key_lower.contains("secret")
            {
                format!("{key}=[REDACTED]")
            } else {
                token.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
