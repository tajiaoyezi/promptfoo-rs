# Task 14.2: redteam-plugin-strategy-parity

**Status**: In Progress
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 14 — provider-assertion-redteam-parity
**Dependencies**: task-14.1-provider-assertion-inventory-parity

## 1. Background

Audit found local redteam registry has 3 plugin defaults and 3 strategy defaults, while upstream has a large plugin/strategy surface. PRD forbids silent omissions. Basis: PRD §Compatibility Matrix, ADR-009, docs/audits/promptfoo-upstream-inventory-gap-2026-05-30.md.

## 2. Goal

For every upstream redteam plugin/strategy inventory item, implement core behavior or register P1/P2/later/unsupported with fixture or reason evidence.

## 3. Scope

### In Scope

- src/redteam/
- compatibility/fixtures/redteam/
- tests/redteam_plugin_strategy_parity.rs
- docs/compatibility/matrix.md
- docs/compatibility/redteam.md

### Out Of Scope

- Real harmful generation against external services; use mock/recorded evaluators.
- Provider transport internals outside redteam needs.

## 4. Users / Actors

- Security redteam team: needs plugin/strategy coverage transparency.
- Enterprise compliance team: needs policy and safety boundaries documented.

## 5. Behavior Contract

Every redteam plugin/strategy inventory item must be represented in matrix and registry coverage report. P0 core flows require fixtures; P2/later rows require reason and user-visible behavior.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/tasks/task-11.2-item-level-capability-inventory.md
- docs/audits/promptfoo-upstream-inventory-gap-2026-05-30.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`serde_json`、内部模块 `redteam`、`compatibility::matrix`、`compatibility::fixtures`。

### 5.3 函数签名

- `RedteamInventoryCoverage::from_registry(registry: &RedteamRegistry, inventory: &CapabilityInventory) -> RedteamInventoryCoverage`
- `validate_redteam_parity(coverage: &RedteamInventoryCoverage, fixtures: &FixtureCorpus) -> RedteamParityReport`
- `redteam_gap_user_message(item: &InventoryItem, classification: GapClass) -> String`

## 6. Acceptance Criteria

- [ ] **AC1** (PRD §Compatibility Matrix): every redteam plugin/strategy inventory item has matrix row and registry coverage status.
- [ ] **AC2** (PRD §Core Capabilities): P0 redteam fixtures execute with mock target/evaluator and enter golden diff gate.
- [ ] **AC3** (ADR-009): P2/later redteam rows have reasons, user-visible messages, and no silent omissions.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-14.2.1 | TEST-14.2.1 | tests/redteam_plugin_strategy_parity.rs | install, typecheck, unit-test, manual | Not Started |
| AC2 | SCEN-14.2.1 | TEST-14.2.2 | tests/redteam_plugin_strategy_parity.rs | install, typecheck, unit-test, manual | Not Started |
| AC3 | SCEN-14.2.1 | TEST-14.2.3 | tests/redteam_plugin_strategy_parity.rs | install, typecheck, unit-test, manual | Not Started |

## 8. Risks

- Some redteam content is safety-sensitive; fixtures should use minimal mock prompts and policy labels.
- Legal/brand-sensitive plugins may require explicit human approval before native implementation.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Manual**: review redteam coverage report for missing items and unsafe fixture content.

## 10. Completion Notes

- **完成日期**：<TBD-after-impl>
- **改动文件**：<TBD-after-impl>
- **commit 列表**：<TBD-after-impl>
- **§9 Verification 结果**：<TBD-after-impl>
- **剩余风险 / 未做项**：<TBD-after-impl>
- **下游 task 影响**：<TBD-after-impl>
