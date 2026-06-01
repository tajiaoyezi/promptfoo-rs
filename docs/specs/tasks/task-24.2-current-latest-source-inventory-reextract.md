# Task 24.2: current-latest-source-inventory-reextract

**Status**: Ready
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 24 — current-latest-perfect-refactor
**Dependencies**: task-24.1-current-latest-upstream-authority-lock

## 1. Background

Current-latest parity cannot reuse the frozen `0.121.13` source inventory as proof. The locked GitHub source target may include commands, providers, assertions, redteam plugins, scanner behavior, viewer/API changes, or examples that were absent from the frozen baseline. 依据用户 2026-06-01 澄清、PRD §Compatibility Matrix / §Compatibility Harness Design、ADR-009、ADR-011。

## 2. Goal

Re-extract source inventory from the locked current-latest target and require every discovered functionality row to be accounted for in the current-latest matrix.

## 3. Scope

### In Scope

- `compatibility/inventory/current-latest-source-inventory.json`
- `compatibility/matrix/current-latest-matrix.json`
- `scripts/release/current-latest-source-inventory.sh`
- `scripts/release/runtime-smoke.sh`
- `src/compatibility/inventory.rs`
- `tests/current_latest_source_inventory.rs`
- `docs/compatibility/matrix.md`
- `docs/compatibility/target-policy.md`
- `test/features/perfect-refactor-parity.feature`

### Out Of Scope

- 不实现 inventory 中发现的所有功能；本 task 只保证无沉默遗漏。
- 不把 P2/later row 当作完成；必须有 reason 或 blocker。
- 不需要真实 provider credentials。

## 4. Users / Actors

- Compatibility reviewer: checks that current-latest rows are complete.
- Implementation agent: uses rows to create parity implementation tasks.
- Release maintainer: blocks claims on unclassified rows.

## 5. Behavior Contract

The extractor must consume the task 24.1 current-latest lock and source snapshot, then produce stable item IDs for CLI commands/flags, config features, providers, assertions, redteam plugins/strategies, outputs, viewer routes, Node API surfaces, examples, and documented workflows. Every row must have source references, category, P0/P1/P2 level, implementation status, verification owner, and blocker/fixture evidence. Any unclassified current-latest row must block the perfect-refactor claim.

### 5.1 Required Reading

- docs/specs/tasks/task-24.1-current-latest-upstream-authority-lock.md
- docs/specs/tasks/task-17.1-frozen-source-inventory-extractor.md
- docs/specs/tasks/task-18.1-source-inventory-ledger-closure.md
- docs/compatibility/matrix.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `serde::{Serialize, Deserialize}`, `serde_json::Value`, `std::collections::{BTreeMap, BTreeSet}`, `promptfoo_rs::compatibility::inventory::{CurrentLatestInventoryRow, CurrentLatestInventoryReport}`.
- Tooling commands: source snapshot acquisition from task 24.1 lock, `bash scripts/release/current-latest-source-inventory.sh`, adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke.

### 5.3 函数签名

- `extract_current_latest_inventory(lock: &CurrentLatestTargetLock, source_root: &Path) -> Result<CurrentLatestInventoryReport, InventoryError>`
- `reconcile_current_latest_matrix(inventory: &CurrentLatestInventoryReport, existing_matrix: &CompatibilityMatrix) -> CurrentLatestMatrixReport`
- Shell contract: `CURRENT_LATEST_SOURCE_ROOT=<path> bash scripts/release/current-latest-source-inventory.sh`

## 6. Acceptance Criteria

- [ ] **AC1** (ADR-011): inventory rows are extracted from the locked current-latest source target, not from frozen baseline files or floating upstream refs.
- [ ] **AC2** (ADR-009): commands, flags, config features, providers, assertions, redteam plugins/strategies, outputs, viewer, Node API, examples, and documented workflows have stable IDs and source references.
- [ ] **AC3** (PRD §Compatibility Matrix): every current-latest row has P0/P1/P2 level, implementation status, verification owner, and fixture/blocker/waiver reason.
- [ ] **AC4** (task-20.2): any unclassified row or row without evidence keeps `perfect_refactor_claim_allowed=false`.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-24.2.1 | TEST-24.2.1 | tests/current_latest_source_inventory.rs | install, typecheck, unit-test, integration, build | Not Started |
| AC2 | SCEN-24.2.1 | TEST-24.2.2 | tests/current_latest_source_inventory.rs | install, lint, typecheck, unit-test, coverage, build | Not Started |
| AC3 | SCEN-24.2.1 | TEST-24.2.3 | tests/current_latest_source_inventory.rs | install, typecheck, unit-test, e2e, runtime-smoke, build | Not Started |
| AC4 | SCEN-24.2.1 | TEST-24.2.4 | tests/current_latest_source_inventory.rs | install, lint, typecheck, unit-test, runtime-smoke, build | Not Started |

## 8. Risks

- Upstream layout can change; extractor must report unknown patterns as blockers.
- A complete inventory may greatly expand implementation scope.
- Generated examples/docs rows can duplicate source rows; reconciliation must deduplicate by stable item ID.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Lint**: adapter §Commands Lint
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Integration tests**: adapter §Commands Integration tests
- **E2E tests**: adapter §Commands E2E tests
- **Coverage**: adapter §Commands Coverage
- **Build**: adapter §Commands Build
- **Runtime smoke**: adapter §Commands Runtime smoke

## 10. Completion Notes

- **完成日期**：<TBD-after-impl>
- **改动文件**：<TBD-after-impl>
- **commit 列表**：<TBD-after-impl>
- **§9 Verification 结果**：<TBD-after-impl>
- **剩余风险 / 未做项**：<TBD-after-impl>
- **下游 task 影响**：<TBD-after-impl>
