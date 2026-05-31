# Task 19.2: core-config-source-fixture-burndown

**Status**: Ready
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 19 — source-accounting-native-burndown
**Dependencies**: task-19.1-viewer-config-source-reclassification

## 1. Background

Task 19.1 只处理 `src/app/**` viewer config 分类。剩余 non-app config rows 包括 `commands/config.ts`、`configTypes.ts`、`util/config/*.ts`、`globalConfig/*.ts`、server/cloud/otel/policy config 等，它们与 promptfooconfig/env/files 或服务配置边界相关，不能被 Local Web viewer P1 规则吞掉。依据 PRD §Core Capabilities / §Compatibility Matrix、ADR-009。

## 2. Goal

为 non-app core config source rows 补 native/bridge fixture evidence 或显式 blocker，清楚区分 runtime config P0、cloud/server/telemetry external blocker、以及可降级的非核心辅助配置。

## 3. Scope

### In Scope

- `src/config/`
- `src/compatibility/inventory.rs`
- `scripts/release/source-inventory-evidence.sh`
- `compatibility/fixtures/config/`
- `tests/core_config_source_fixture_burndown.rs`
- `docs/compatibility/matrix.md`
- `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`

### Out Of Scope

- 不实现 promptfoo cloud/server SaaS 行为。
- 不要求真实 telemetry/backend 服务。
- 不处理 provider module blockers。

## 4. Users / Actors

- CLI user：需要 promptfooconfig/env/file config 行为保持 P0。
- Enterprise reviewer：需要 cloud/server/telemetry 配置不会被伪装成本地 parity。
- Compatibility maintainer：需要每个 non-app config source row 有 fixture 或 blocker。

## 5. Behavior Contract

Non-app config rows 必须进入三类之一：native fixture covered、bridge fixture covered、explicit external/unsupported blocker。`source-inventory-evidence.json` 必须报告 non-app config blocker count 和 resolved count；P0 runtime config rows 不能因分类规则而静默降级。

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/tasks/task-19.1-viewer-config-source-reclassification.md
- docs/compatibility/matrix.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`promptfoo_rs::config`、`promptfoo_rs::compatibility::inventory`、`serde_json`、`std::fs`。
- Tooling commands：adapter §Commands Install / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke。

### 5.3 函数签名

- `classify_non_app_config_source_row(row: &SourceAccountingRow) -> CoreConfigSourceDecision`
- `validate_core_config_source_burndown(ledger: &SourceAccountingLedger) -> CoreConfigSourceBurndownReport`
- `write_core_config_source_burndown(report: &CoreConfigSourceBurndownReport, path: &Path) -> Result<(), CompatibilityEvidenceError>`

## 6. Acceptance Criteria

- [ ] **AC1** (PRD §Core Capabilities): runtime config source rows have native/bridge fixture evidence tied to promptfooconfig/env/file behavior.
- [ ] **AC2** (PRD §Non Goals): cloud/server/telemetry config rows remain explicit unsupported/external blockers and are not counted as local runtime parity.
- [ ] **AC3** (ADR-009): `source-inventory-evidence.json` reports non-app config resolved/blocker counts with item ids and reasons.
- [ ] **AC4** (Phase 19): no non-app config row remains as generic “generated P0 accounting row requires...” without a specific decision.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-19.2.1 | TEST-19.2.1 | tests/core_config_source_fixture_burndown.rs | install, typecheck, unit-test, integration, build | Not Started |
| AC2 | SCEN-19.2.1 | TEST-19.2.2 | tests/core_config_source_fixture_burndown.rs | install, typecheck, unit-test, coverage, build | Not Started |
| AC3 | SCEN-19.2.1 | TEST-19.2.3 | tests/core_config_source_fixture_burndown.rs | install, typecheck, unit-test, runtime-smoke, build | Not Started |
| AC4 | SCEN-19.2.1 | TEST-19.2.4 | tests/core_config_source_fixture_burndown.rs | install, typecheck, unit-test, e2e, build | Not Started |

## 8. Risks

- Some config files are server/cloud-specific and may require unsupported/later classification rather than implementation.
- Fixture names must stay traceable to source item ids.
- Core config behavior changes can affect many CLI tests; keep implementation scoped to evidence/classification unless fixture reveals real bug.

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

- **完成日期**：<TBD-after-impl>
- **改动文件**：<TBD-after-impl>
- **commit 列表**：<TBD-after-impl>
- **§9 Verification 结果**：<TBD-after-impl>
- **剩余风险 / 未做项**：<TBD-after-impl>
- **下游 task 影响**：<TBD-after-impl>
