# Task 20.1: source-provider-accounting-reconciliation

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 20 — cross-ledger-perfect-claim-closure
**Dependencies**: task-19.3-provider-request-response-fixture-burndown, task-19.4-external-authority-blocker-waiver-gate

## 1. Background

Phase 19 后 `longtail-classification.json` 已证明 22 个 provider module rows 有 fixture evidence、15 个仍为 external-authority blockers；但 `source-inventory-evidence.json.remaining_p0_blockers` 仍把 37 个 provider source rows 全部计为 P0 accounting blockers，导致 `p0_accounting_blocker_count=44`。这是跨 gate 口径不一致，不是新增实现缺口。依据 Phase 19 §9 Artifact evidence、ADR-009、docs/compatibility/matrix.md Task 19.3/19.4 段。

## 2. Goal

让 source accounting ledger 消费 provider burndown evidence：fixture-covered provider rows 以 item-level fixture evidence 出账，external-authority provider rows 保留 blocker，最终 source accounting blocker count 与 provider/config gate 一致。

## 3. Scope

### In Scope

- `src/compatibility/inventory.rs`
- `src/compatibility/provider_assertion.rs`
- `scripts/release/source-inventory-evidence.sh`
- `scripts/release/longtail-classification.sh`
- `tests/source_provider_accounting_reconciliation.rs`
- `docs/compatibility/matrix.md`
- `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`

### Out Of Scope

- 不删除 provider source rows。
- 不把 external-authority provider rows 标为 ready。
- 不改变 provider fixture 语义或新增真实服务调用。

## 4. Users / Actors

- Release reviewer：需要 source accounting 和 provider burndown blocker count 一致。
- Compatibility maintainer：需要知道 provider rows 是 fixture-covered 还是 external-authority。
- Future implementer：需要剩余 22 blockers 的最小真实边界。

## 5. Behavior Contract

Source accounting evidence 必须在 provider rows 上复用 provider burndown classification。`fixture-covered` provider rows 必须保留 item id、source reference、fixture verification、owner 和 reason，但不得进入 `remaining_p0_blockers`。`external-authority` provider rows 必须继续进入 `remaining_p0_blockers` 并引用 external authority gate。不得通过删除 rows、降级 P0 或隐藏 blockers 降低 count。

### 5.1 Required Reading

- docs/specs/tasks/task-19.3-provider-request-response-fixture-burndown.md
- docs/specs/tasks/task-19.4-external-authority-blocker-waiver-gate.md
- docs/specs/phases/phase-19-source-accounting-native-burndown.md
- docs/compatibility/matrix.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`promptfoo_rs::compatibility::inventory`、`promptfoo_rs::compatibility::provider_assertion`、`serde_json`、`std::fs`、`std::path::Path`。
- Tooling commands：adapter §Commands Install / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke。

### 5.3 函数签名

- `classify_provider_source_accounting_row(row: &SourceAccountingRow, provider_report: &ProviderModuleBurndownReport) -> ProviderSourceAccountingDecision`
- `validate_provider_source_accounting_reconciliation(ledger: &SourceAccountingLedger, provider_report: &ProviderModuleBurndownReport) -> ProviderSourceAccountingReconciliationReport`
- `write_provider_source_accounting_reconciliation(report: &ProviderSourceAccountingReconciliationReport, path: &Path) -> Result<(), InventoryError>`

## 6. Acceptance Criteria

- [x] **AC1** (ADR-009): every provider source row that provider burndown resolved by fixture is represented with fixture verification and excluded from `remaining_p0_blockers`.
- [x] **AC2** (Phase 19): provider external-authority rows remain release-blocking with item-level required decision evidence.
- [x] **AC3** (PRD §Success Metrics): `source-inventory-evidence.json.p0_accounting_blocker_count` equals 22 and `remaining_p0_blockers` contains exactly 7 config external blockers + 15 provider external-authority blockers.
- [x] **AC4** (docs/audits): audit and compatibility docs explain the cross-ledger reconciliation without claiming external-authority completion.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-20.1.1 | TEST-20.1.1 | tests/source_provider_accounting_reconciliation.rs | install, typecheck, unit-test, integration, build | Done |
| AC2 | SCEN-20.1.1 | TEST-20.1.2 | tests/source_provider_accounting_reconciliation.rs | install, typecheck, unit-test, coverage, build | Done |
| AC3 | SCEN-20.1.1 | TEST-20.1.3 | tests/source_provider_accounting_reconciliation.rs | install, typecheck, unit-test, runtime-smoke, build | Done |
| AC4 | SCEN-20.1.1 | TEST-20.1.4 | tests/source_provider_accounting_reconciliation.rs | install, typecheck, unit-test, e2e, build | Done |

## 8. Risks

- Provider rows may be double-counted if source accounting and longtail classification drift.
- Lowering blocker count by deleting source rows would hide upstream surface and violate Phase 18.
- External-authority rows still block perfect-refactor completion.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Integration tests**: adapter §Commands Integration tests
- **E2E tests**: adapter §Commands E2E tests
- **Coverage**: adapter §Commands Coverage
- **Build**: adapter §Commands Build
- **Runtime smoke**: adapter §Commands Runtime smoke

## 10. Completion Notes

- **完成日期**：2026-05-31
- **改动文件**：
  - `tests/source_provider_accounting_reconciliation.rs`
  - `src/compatibility/inventory.rs`
  - `scripts/release/source-inventory-evidence.sh`
  - `docs/compatibility/matrix.md`
  - `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`
  - `docs/s2v-adapter.md`
  - `docs/specs/phases/phase-20-cross-ledger-perfect-claim-closure.md`
  - `docs/specs/tasks/task-20.1-source-provider-accounting-reconciliation.md`
- **commit 列表**：
  - `8d0158d` `test(compatibility): add SCEN-20.1.1 provider source accounting RED tests`
  - `2d47382` `feat(compatibility): reconcile provider source accounting with burndown evidence`
- **§9 Verification 结果**：
  - install: PASS — helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - typecheck: PASS — helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS — helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-20.1.1 ~ TEST-20.1.4 通过。
  - integration: PASS — `bash scripts/release/integration.sh` 通过，包含 source inventory/provider reconciliation 相关 gate。
  - e2e: PASS — `bash scripts/release/e2e.sh` 通过。
  - build: PASS — helper 执行 `cargo build --workspace`、viewer/npm `pnpm build` 通过。
  - coverage: PASS — `bash scripts/release/coverage.sh` 通过；`s2v_coverage_threshold_guard` 通过。
  - runtime-smoke: PASS — `bash scripts/release/runtime-smoke.sh` 通过；`source-inventory-evidence.json` reports `p0_accounting_blocker_count=22`, 7 config blockers, 15 provider blockers, `provider_source_accounting_reconciliation.resolved_provider_fixture_count=22`, `provider_external_authority_count=15`, `provider_source_generic_blocker_count=0`。
- **剩余风险 / 未做项**：15 个 provider external-authority blockers 和 7 个 config external blockers 仍保留为真实边界；本 task 只消除 source/provider 口径重复计数，不解除外部授权、账号、凭据或 publication/current-upstream blocker。
- **下游 task 影响**：task 20.2 可以基于统一后的 `p0_accounting_blocker_count=22` 构建 perfect-refactor claim contract，并明确区分 local stable gate 与 perfect-refactor completion。
