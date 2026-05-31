# Task 19.1: viewer-config-source-reclassification

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 19 — source-accounting-native-burndown
**Dependencies**: task-18.1-source-inventory-ledger-closure, task-18.4-publication-authority-release-gate

## 1. Background

Phase 18 让 source inventory silent omissions 变成 explicit ledger rows，但 `target/release-gates/source-inventory-ledger.json` 仍把 74 个 generated `config:*` rows 计入 P0 blockers，其中 56 个来自 `promptfoo@0.121.13:src/app/**`。PRD/compatibility matrix 已把 Local Web viewer 定为 P1，因为目标是本地结果浏览和数据契约，不承诺 upstream React UI 像素级 parity。依据 PRD §Compatibility Matrix、docs/compatibility/matrix.md `Local Web viewer` 行、Phase 18 §9 artifact evidence。

## 2. Goal

将 `src/app/**` viewer config/editor/test 源文件从 generated P0 core config blocker 纠正为 P1 Local Web viewer accounting evidence，同时保证 non-app core config rows 继续保持 P0 blocker，避免弱化 promptfooconfig/env/files 的 P0 兼容承诺。

## 3. Scope

### In Scope

- `src/compatibility/inventory.rs`
- `scripts/release/source-inventory-evidence.sh`
- `tests/viewer_config_source_reclassification.rs`
- `target/release-gates/source-inventory-ledger.json`
- `target/release-gates/source-inventory-evidence.json`
- `docs/compatibility/matrix.md`
- `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`

### Out Of Scope

- 不实现 upstream React UI 组件或像素 parity。
- 不降级 `commands/config.ts`、`util/config/*.ts`、`configTypes.ts`、`globalConfig/*.ts` 等 non-app core config rows。
- 不改 provider module blocker 分类；provider blockers 由 task 19.3/19.4 处理。

## 4. Users / Actors

- Compatibility maintainer：需要区分 viewer UI config 与 CLI/runtime config P0。
- Release reviewer：需要看到 P0 accounting blocker count 的减少来自有依据的分级，不是删除 blocker。
- Future implementer：需要清楚剩余 non-app core config rows 仍要 fixture 或 blocker。

## 5. Behavior Contract

Source accounting ledger 生成 generated rows 时，`source_reference` 匹配 `promptfoo@0.121.13:src/app/**` 且 category=`config` 的项必须被分类为 level=`P1`、target_status=`later` 或 `snapshot-planned`、owner=`web-viewer`、verification 指向 viewer evidence，并写明“Local Web viewer P1；pixel/upstream React UI parity out of scope”。非 `src/app/**` 的 config rows 不得被此规则降级。

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/compatibility/matrix.md
- docs/specs/tasks/task-18.1-source-inventory-ledger-closure.md
- docs/specs/phases/phase-18-perfect-refactor-blocker-burndown.md
- docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`promptfoo_rs::compatibility::inventory::{InventoryItem, SourceAccountingLedger, SourceAccountingRow}`、`serde_json`、`std::fs`。
- Tooling commands：adapter §Commands Install / Typecheck / Unit Test / Integration tests / Coverage / Build / Runtime smoke。

### 5.3 函数签名

- `is_viewer_config_source_reference(source_reference: &str) -> bool`
- `classify_generated_source_accounting_row(item: &InventoryItem) -> SourceAccountingRow`
- `source_accounting_burndown_summary(ledger: &SourceAccountingLedger) -> SourceAccountingBurndownSummary`

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Compatibility Matrix / Local Web viewer): generated `config:*` rows under `src/app/**` become P1 Local Web viewer accounting evidence with owner, verification, and reason.
- [x] **AC2** (PRD §Core Capabilities): non-app config rows remain P0 blocked unless they already have explicit native/bridge fixture evidence; no blanket demotion is allowed.
- [x] **AC3** (Phase 18 §9): release evidence reports viewer-config reclassification counts and reduces P0 accounting blocker count from 111 to the remaining non-app config/provider blockers.
- [x] **AC4** (ADR-009): compatibility matrix and audit explain that this is a scope correction, not a claim of upstream React UI parity.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-19.1.1 | TEST-19.1.1 | tests/viewer_config_source_reclassification.rs | install, typecheck, unit-test, integration, build | Done |
| AC2 | SCEN-19.1.1 | TEST-19.1.2 | tests/viewer_config_source_reclassification.rs | install, typecheck, unit-test, coverage, build | Done |
| AC3 | SCEN-19.1.1 | TEST-19.1.3 | tests/viewer_config_source_reclassification.rs | install, typecheck, unit-test, runtime-smoke, build | Done |
| AC4 | SCEN-19.1.1 | TEST-19.1.4 | tests/viewer_config_source_reclassification.rs | install, typecheck, unit-test, integration, build | Done |

## 8. Risks

- 过宽路径匹配会把 real runtime config 降级；规则必须限定 `:src/app/`。
- reclassification 只能减少错误 P0 accounting，不代表 viewer upstream UI parity 完成。
- Windows path separator 不能影响 source reference 匹配。

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Integration tests**: adapter §Commands Integration tests
- **Coverage**: adapter §Commands Coverage
- **Build**: adapter §Commands Build
- **Runtime smoke**: adapter §Commands Runtime smoke

## 10. Completion Notes

- **完成日期**：2026-05-31
- **改动文件**：
  - `tests/viewer_config_source_reclassification.rs`
  - `src/compatibility/inventory.rs`
  - `scripts/release/source-inventory-evidence.sh`
  - `scripts/release/integration.sh`
  - `docs/compatibility/matrix.md`
  - `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`
  - `docs/s2v-adapter.md`
  - `docs/specs/phases/phase-19-source-accounting-native-burndown.md`
  - `docs/specs/tasks/task-19.1-viewer-config-source-reclassification.md`
- **commit 列表**：
  - `fa5f7b3` `test(compatibility): add SCEN-19.1.1 viewer config reclassification RED tests`
  - `48eef72` `feat(compatibility): reclassify viewer config source blockers`
- **§9 Verification 结果**：
  - install: PASS — helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - typecheck: PASS — helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS — helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-19.1.1 ~ TEST-19.1.4 通过。
  - integration: PASS — `bash scripts/release/integration.sh` 通过，包含 `viewer_config_source_reclassification`。
  - build: PASS — helper 执行 `cargo build --workspace`、viewer/npm `pnpm build` 通过。
  - coverage: PASS — `bash scripts/release/coverage.sh` 通过；`s2v_coverage_threshold_guard` 通过。
  - runtime-smoke: PASS — `bash scripts/release/runtime-smoke.sh` 通过；`target/release-gates/source-inventory-evidence.json` status=`ready-with-blockers`，viewer_config_reclassified_count=56，p0_accounting_blocker_count=55，remaining_p0_blockers=55；`target/release-gates/source-inventory-ledger.json` 同步 viewer_config_reclassified_count=56、p0_blocker_count=55。
- **剩余风险 / 未做项**：本 task 只是 P1 viewer config scope correction，不宣称 upstream React UI parity；55 个 P0 source accounting blockers 仍需 task 19.2/19.3/19.4 继续处理。
- **下游 task 影响**：task 19.2 可聚焦 18 个 non-app core config blockers；task 19.3/19.4 继续处理 provider fixture/external authority blockers；Phase 19 smoke 应检查 source inventory artifact 的 56/55 计数。
