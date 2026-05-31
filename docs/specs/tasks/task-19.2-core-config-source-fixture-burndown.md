# Task 19.2: core-config-source-fixture-burndown

**Status**: Done
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

- [x] **AC1** (PRD §Core Capabilities): runtime config source rows have native/bridge fixture evidence tied to promptfooconfig/env/file behavior.
- [x] **AC2** (PRD §Non Goals): cloud/server/telemetry config rows remain explicit unsupported/external blockers and are not counted as local runtime parity.
- [x] **AC3** (ADR-009): `source-inventory-evidence.json` reports non-app config resolved/blocker counts with item ids and reasons.
- [x] **AC4** (Phase 19): no non-app config row remains as generic “generated P0 accounting row requires...” without a specific decision.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-19.2.1 | TEST-19.2.1 | tests/core_config_source_fixture_burndown.rs | install, typecheck, unit-test, integration, build | Done |
| AC2 | SCEN-19.2.1 | TEST-19.2.2 | tests/core_config_source_fixture_burndown.rs | install, typecheck, unit-test, coverage, build | Done |
| AC3 | SCEN-19.2.1 | TEST-19.2.3 | tests/core_config_source_fixture_burndown.rs | install, typecheck, unit-test, runtime-smoke, build | Done |
| AC4 | SCEN-19.2.1 | TEST-19.2.4 | tests/core_config_source_fixture_burndown.rs | install, typecheck, unit-test, e2e, build | Done |

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

- **完成日期**：2026-05-31
- **改动文件**：
  - `tests/core_config_source_fixture_burndown.rs`
  - `tests/viewer_config_source_reclassification.rs`
  - `src/compatibility/inventory.rs`
  - `scripts/release/source-inventory-evidence.sh`
  - `scripts/release/integration.sh`
  - `docs/compatibility/matrix.md`
  - `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`
  - `docs/s2v-adapter.md`
  - `docs/specs/phases/phase-19-source-accounting-native-burndown.md`
  - `docs/specs/tasks/task-19.2-core-config-source-fixture-burndown.md`
- **commit 列表**：
  - `c5736c2` `test(config): add SCEN-19.2.1 core config burndown RED tests`
  - `d9f6b9b` `feat(config): classify non-app config source blockers`
  - `3494728` `refactor(config): align core config burndown error type`
- **§9 Verification 结果**：
  - install: PASS — helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - typecheck: PASS — helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS — helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-19.2.1 ~ TEST-19.2.4 通过。
  - integration: PASS — `bash scripts/release/integration.sh` 通过，包含 `core_config_source_fixture_burndown`。
  - e2e: PASS — `bash scripts/release/e2e.sh` 通过。
  - build: PASS — helper 执行 `cargo build --workspace`、viewer/npm `pnpm build` 通过。
  - coverage: PASS — `bash scripts/release/coverage.sh` 通过；`s2v_coverage_threshold_guard` 通过。
  - runtime-smoke: PASS — `bash scripts/release/runtime-smoke.sh` 通过；`source-inventory-evidence.json` status=`ready-with-blockers`，`p0_accounting_blocker_count=44`，`core_config_source_burndown.non_app_config_total=18`，`non_app_config_fixture_covered_count=8`，`non_app_config_external_blocker_count=7`，`non_app_config_auxiliary_registration_count=3`，`non_app_config_generic_blocker_count=0`。
- **剩余风险 / 未做项**：7 个 cloud/server/telemetry/global config rows 仍是 explicit external blockers；44 个总 P0 source accounting blockers 仍阻止“完美重构”完成，其中 provider module blockers 需 task 19.3/19.4 继续燃尽。
- **下游 task 影响**：task 19.3 可聚焦剩余 provider request/response module blockers；task 19.4 需要把 external-authority config/provider/publication blockers 集中到不可伪造 gate；Phase 19 smoke 应检查 P0=44、non-app config generic blocker=0。
