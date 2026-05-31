use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::inventory::{
    CapabilityInventory, CompatibilityEvidenceError, InventoryError, InventoryItem,
};
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderModuleResolutionKind {
    FixtureCovered,
    ExternalBlocker,
    Blocked,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderModuleResolution {
    pub item_id: String,
    pub source_reference: String,
    pub kind: ProviderModuleResolutionKind,
    pub reason: String,
    pub verification: String,
    pub fixture_ids: Vec<String>,
    pub docs_link: String,
    pub requires_external_authority: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderModuleBurndownReport {
    pub initial_blocker_count: usize,
    pub resolved_by_fixture_count: usize,
    pub new_dedicated_request_response_fixture_count: usize,
    pub remaining_blocker_count: usize,
    pub external_authority_blocker_count: usize,
    pub generic_blocker_count: usize,
    pub resolved_by_fixture: Vec<ProviderModuleResolution>,
    pub remaining_blockers: Vec<ProviderModuleResolution>,
    pub fixtures_requiring_real_secrets: Vec<String>,
}

pub type LongtailClassificationReport = ProviderModuleBurndownReport;
pub type ProviderFixtureBurndownReport = ProviderModuleBurndownReport;

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

pub fn provider_module_blocker_rows(matrix: &CapabilityMatrix) -> Vec<CapabilityRow> {
    matrix
        .rows
        .iter()
        .filter(|row| {
            row.capability.starts_with("provider:src-providers-")
                && row.level == "P0"
                && row.verification.starts_with("blocker:")
        })
        .cloned()
        .collect()
}

pub fn resolve_provider_module_fixture(
    row: &CapabilityRow,
    fixtures: &FixtureCorpus,
) -> ProviderModuleResolution {
    let item_id = row.capability.clone();
    let source_reference = source_reference_from_notes(row);
    let fixture_ids = available_fixture_ids(provider_module_fixture_ids(&item_id), fixtures);
    if !fixture_ids.is_empty() {
        let verification = format!("fixture:{}", fixture_ids.join("+"));
        return ProviderModuleResolution {
            item_id: item_id.clone(),
            source_reference: source_reference.clone(),
            kind: ProviderModuleResolutionKind::FixtureCovered,
            reason: format!(
                "aggregate provider fixture evidence ({}) covers {item_id}; source: {source_reference}",
                fixture_ids.join(", ")
            ),
            verification,
            fixture_ids,
            docs_link: provider_module_docs_link(),
            requires_external_authority: false,
        };
    }

    let (reason, requires_external_authority) =
        explicit_provider_module_blocker_reason(&item_id, &source_reference);
    let kind = if requires_external_authority {
        ProviderModuleResolutionKind::ExternalBlocker
    } else {
        ProviderModuleResolutionKind::Blocked
    };
    ProviderModuleResolution {
        item_id: item_id.clone(),
        source_reference,
        kind,
        reason,
        verification: format!("blocker:{item_id}"),
        fixture_ids: Vec::new(),
        docs_link: provider_module_docs_link(),
        requires_external_authority,
    }
}

pub fn resolve_provider_request_response_fixture(item_id: &str) -> ProviderModuleResolution {
    let source_reference = source_reference_from_provider_item_id(item_id);
    let fixture_ids = dedicated_request_response_fixture_ids(item_id)
        .iter()
        .map(|fixture_id| (*fixture_id).to_string())
        .collect::<Vec<_>>();
    if !fixture_ids.is_empty() {
        return ProviderModuleResolution {
            item_id: item_id.to_string(),
            source_reference: source_reference.clone(),
            kind: ProviderModuleResolutionKind::FixtureCovered,
            reason: format!(
                "dedicated request/response fixture evidence ({}) covers {item_id}; source: {source_reference}",
                fixture_ids.join(", ")
            ),
            verification: format!("fixture:{}", fixture_ids.join("+")),
            fixture_ids,
            docs_link: provider_module_docs_link(),
            requires_external_authority: false,
        };
    }

    let (reason, requires_external_authority) =
        explicit_provider_module_blocker_reason(item_id, &source_reference);
    ProviderModuleResolution {
        item_id: item_id.to_string(),
        source_reference,
        kind: if requires_external_authority {
            ProviderModuleResolutionKind::ExternalBlocker
        } else {
            ProviderModuleResolutionKind::Blocked
        },
        reason,
        verification: format!("blocker:{item_id}"),
        fixture_ids: Vec::new(),
        docs_link: provider_module_docs_link(),
        requires_external_authority,
    }
}

pub fn validate_p0_provider_module_burndown(
    matrix: &CapabilityMatrix,
    fixtures: &FixtureCorpus,
) -> ProviderModuleBurndownReport {
    let mut report = ProviderModuleBurndownReport {
        fixtures_requiring_real_secrets: fixtures.fixtures_requiring_real_secrets(),
        ..ProviderModuleBurndownReport::default()
    };
    let blocker_rows = provider_module_blocker_rows(matrix);
    report.initial_blocker_count = blocker_rows.len();

    for row in &blocker_rows {
        let resolution = resolve_provider_module_fixture(row, fixtures);
        match resolution.kind {
            ProviderModuleResolutionKind::FixtureCovered => {
                report.resolved_by_fixture.push(resolution);
            }
            ProviderModuleResolutionKind::ExternalBlocker => {
                report.remaining_blockers.push(resolution);
            }
            ProviderModuleResolutionKind::Blocked => {
                report.remaining_blockers.push(resolution);
            }
        }
    }

    report.resolved_by_fixture_count = report.resolved_by_fixture.len();
    report.new_dedicated_request_response_fixture_count = report
        .resolved_by_fixture
        .iter()
        .filter(|resolution| {
            resolution
                .fixture_ids
                .iter()
                .any(|fixture_id| is_dedicated_request_response_fixture_id(fixture_id))
        })
        .count();
    report.remaining_blocker_count = report.remaining_blockers.len();
    report.external_authority_blocker_count = report
        .remaining_blockers
        .iter()
        .filter(|resolution| resolution.requires_external_authority)
        .count();
    report.generic_blocker_count = report
        .remaining_blockers
        .iter()
        .filter(|resolution| !resolution.requires_external_authority)
        .count();
    report
}

pub fn validate_provider_fixture_burndown(
    report: &LongtailClassificationReport,
) -> ProviderFixtureBurndownReport {
    report.clone()
}

pub fn write_provider_fixture_burndown(
    report: &ProviderFixtureBurndownReport,
    path: &Path,
) -> Result<(), CompatibilityEvidenceError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(InventoryError::Write)?;
    }
    let json = serde_json::to_string_pretty(report)
        .map_err(|error| InventoryError::Parse(error.to_string()))?;
    fs::write(path, format!("{json}\n")).map_err(InventoryError::Write)
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

fn provider_module_docs_link() -> String {
    "docs/compatibility/matrix.md#p0-provider-module-burndown".to_string()
}

fn provider_module_fixture_ids(item_id: &str) -> &'static [&'static str] {
    match item_id {
        "provider:src-providers-anthropic-defaults"
        | "provider:src-providers-anthropic-generic"
        | "provider:src-providers-anthropic-messages"
        | "provider:src-providers-anthropic-types"
        | "provider:src-providers-anthropic-util" => &["p0-provider-anthropic-message"],
        "provider:src-providers-anthropic-completion" => &["p0-provider-anthropic-completion"],
        "provider:src-providers-http" => &["p0-provider-http-get", "p0-provider-http-post"],
        "provider:src-providers-httptransforms" => &["p0-provider-http-transform"],
        "provider:src-providers-httpmultipart" => &["p0-provider-http-multipart"],
        "provider:src-providers-ollama" => &["p0-provider-ollama-chat"],
        "provider:src-providers-openai-chat"
        | "provider:src-providers-openai-index"
        | "provider:src-providers-openai-types" => &["p0-provider-openai-chat"],
        "provider:src-providers-openai-completion" => &["p0-provider-openai-completion"],
        "provider:src-providers-openai-defaults" => {
            &["p0-provider-openai-env", "p0-provider-openai-headers"]
        }
        "provider:src-providers-openai-embedding" => &["p0-provider-openai-embedding"],
        "provider:src-providers-openai-image" => &["p0-provider-openai-image"],
        "provider:src-providers-openai-moderation" => &["p0-provider-openai-moderation"],
        "provider:src-providers-openai-responses" => &["p0-provider-openai-responses"],
        "provider:src-providers-openai-transcription" => &["p0-provider-openai-transcription"],
        "provider:src-providers-openai-util" => &[
            "p0-provider-openai-chat",
            "p0-provider-openai-env",
            "p0-provider-openai-headers",
        ],
        "provider:src-providers-openai-video" => &["p0-provider-openai-video"],
        _ => &[],
    }
}

fn dedicated_request_response_fixture_ids(item_id: &str) -> &'static [&'static str] {
    match item_id {
        "provider:src-providers-anthropic-completion" => &["p0-provider-anthropic-completion"],
        "provider:src-providers-httpmultipart" => &["p0-provider-http-multipart"],
        "provider:src-providers-openai-completion" => &["p0-provider-openai-completion"],
        "provider:src-providers-openai-embedding" => &["p0-provider-openai-embedding"],
        "provider:src-providers-openai-image" => &["p0-provider-openai-image"],
        "provider:src-providers-openai-moderation" => &["p0-provider-openai-moderation"],
        "provider:src-providers-openai-responses" => &["p0-provider-openai-responses"],
        "provider:src-providers-openai-transcription" => &["p0-provider-openai-transcription"],
        "provider:src-providers-openai-video" => &["p0-provider-openai-video"],
        _ => &[],
    }
}

fn is_dedicated_request_response_fixture_id(fixture_id: &str) -> bool {
    matches!(
        fixture_id,
        "p0-provider-anthropic-completion"
            | "p0-provider-http-multipart"
            | "p0-provider-openai-completion"
            | "p0-provider-openai-embedding"
            | "p0-provider-openai-image"
            | "p0-provider-openai-moderation"
            | "p0-provider-openai-responses"
            | "p0-provider-openai-transcription"
            | "p0-provider-openai-video"
    )
}

fn available_fixture_ids(ids: &[&str], fixtures: &FixtureCorpus) -> Vec<String> {
    ids.iter()
        .filter(|id| {
            fixtures
                .records()
                .iter()
                .any(|record| record.manifest.id == **id)
        })
        .map(|id| (*id).to_string())
        .collect()
}

fn source_reference_from_notes(row: &CapabilityRow) -> String {
    row.notes
        .split("source:")
        .nth(1)
        .map(str::trim)
        .filter(|source| !source.is_empty())
        .unwrap_or("unknown source reference")
        .to_string()
}

fn source_reference_from_provider_item_id(item_id: &str) -> String {
    let path = item_id
        .strip_prefix("provider:")
        .unwrap_or(item_id)
        .replace('-', "/");
    format!("promptfoo@0.121.13:{path}.ts")
}

fn explicit_provider_module_blocker_reason(
    item_id: &str,
    source_reference: &str,
) -> (String, bool) {
    let lower = item_id.to_ascii_lowercase();
    let (reason, requires_external_authority) = if lower.contains("claudecodeauth") {
        (
            "Anthropic Claude Code auth requires real local credential flow and product authority before native parity can be claimed",
            true,
        )
    } else if lower.contains("codex") {
        (
            "OpenAI Codex provider modules require external product authority and private SDK/server credential confirmation before native parity can be claimed",
            true,
        )
    } else if lower.contains("billing") {
        (
            "OpenAI billing module requires account-level credentials and billing authority; no local mock may be treated as published parity",
            true,
        )
    } else if lower.contains("chatkit") {
        (
            "OpenAI ChatKit modules require product authority and browser/session fixture confirmation before native parity can be claimed",
            true,
        )
    } else if lower.contains("agents") {
        (
            "OpenAI Agents SDK and tracing modules require dedicated SDK/trace fixtures plus product contract review",
            true,
        )
    } else if lower.contains("realtime") {
        (
            "OpenAI realtime module requires a dedicated streaming protocol fixture and service contract confirmation",
            true,
        )
    } else if lower.contains("assistant") {
        (
            "OpenAI Assistants module requires a stateful API fixture and account-authorized behavior review",
            true,
        )
    } else {
        (
            "Provider module needs a dedicated request/response fixture before aggregate provider evidence can prove per-module parity",
            false,
        )
    };
    (
        format!("{reason}; source: {source_reference}"),
        requires_external_authority,
    )
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
