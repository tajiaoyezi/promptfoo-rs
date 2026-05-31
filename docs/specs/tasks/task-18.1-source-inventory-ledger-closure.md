# Task 18.1: source-inventory-ledger-closure

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 18 — perfect-refactor-blocker-burndown
**Dependencies**: task-17.1-frozen-source-inventory-extractor, task-17.4-longtail-provider-assertion-redteam-classification

## 1. Background

当前审计显示 `target/release-gates/source-inventory-evidence.json` 仍有 2116 个 `missing matrix row for source-extracted item` release blockers。Phase 17 已经证明这些不是 silent omission，但 blocker 仍以“缺矩阵行”形式存在，无法进入逐项 burn-down。依据 docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md §P0 Source inventory、Phase 17 §9 Completion Notes、PRD §Compatibility Matrix、ADR-009。

## 2. Goal

实现 source inventory accounting ledger：每个 source-extracted item 都有 ledger row，包含 level/status/owner/verification/reason/source reference；`missing_matrix_rows` 清零表示没有沉默遗漏，同时 P0 未实现/未 fixture 项继续以 release blocker 明示。

## 3. Scope

### In Scope

- `src/compatibility/inventory.rs`
- `scripts/release/source-inventory-evidence.sh`
- `tests/source_inventory_ledger_closure.rs`
- `target/release-gates/source-inventory-ledger.json`
- `docs/compatibility/matrix.md`
- `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`

### Out Of Scope

- 不在本 task 实现 37 个 P0 provider module 的业务兼容；task 18.2 负责 fixture/native burndown。
- 不改变 frozen baseline；task 18.3 负责 current upstream target mode。
- 不把生成 ledger row 等同于 native parity；ledger 只证明 accounting 完整。

## 4. Users / Actors

- Compatibility reviewer：需要区分 silent missing row 和明确待实现 blocker。
- Release maintainer：需要看到 P0 blocker count，而不是 2116 个不可行动的缺行列表。
- Contributor：需要从 ledger 直接找到每个 source item 的 owner、verification 和下一步处理方式。

## 5. Behavior Contract

`source-inventory-evidence.sh` 必须在 source extraction 后生成 `source-inventory-ledger.json`。ledger 对 source-extracted items 全覆盖；显式 inventory/matrix row 优先，缺显式 row 时生成 accounting row。生成 row 必须有 deterministic verification：P0 缺 fixture/native evidence 生成 `blocker:<item-id>`，P1 生成 `snapshot:<item-id>`，P2 生成 `registration:<item-id>`。`missing_matrix_rows` 只用于真正无法生成 ledger row 的异常；正常情况下为 0。P0 accounting blockers 必须进入 release blockers，防止把缺实现伪装成完成。

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/phases/phase-17-deep-upstream-parity-proof.md
- docs/specs/tasks/task-17.1-frozen-source-inventory-extractor.md
- docs/specs/tasks/task-17.4-longtail-provider-assertion-redteam-classification.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`std::collections::{BTreeMap, BTreeSet}`、`serde::{Deserialize, Serialize}`、内部模块 `compatibility::inventory`、`compatibility::matrix`。
- Tooling commands：adapter §Commands Install / Typecheck / Unit Test / Integration tests / Coverage / Runtime smoke / Build；Git for Windows Bash 执行 release scripts。

### 5.3 函数签名

- `build_source_accounting_ledger(inventory: &SourceExtractedInventory, matrix: &CapabilityMatrix) -> SourceAccountingLedger`
- `SourceAccountingLedger::unrepresented_items(&self) -> Vec<String>`
- `SourceAccountingLedger::p0_blockers(&self) -> Vec<String>`
- `write_source_accounting_ledger(ledger: &SourceAccountingLedger, path: &Path) -> Result<(), InventoryError>`

## 6. Acceptance Criteria

- [x] **AC1** (ADR-009): every source-extracted item gets exactly one ledger row with stable id, category, source reference, level, target status, owner, verification, and reason.
- [x] **AC2** (docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md): `source-inventory-evidence.json` reports `missing_matrix_rows=[]` for generated accounting coverage while still reporting P0 generated blocker rows as release blockers.
- [x] **AC3** (PRD §Compatibility Matrix): generated verification follows P0=`blocker:<item-id>`, P1=`snapshot:<item-id>`, P2=`registration:<item-id>`; generated rows cannot claim `native` unless explicit matrix/inventory evidence already does.
- [x] **AC4** (ADR-007): runtime smoke includes `source-inventory-ledger.json` and release candidate evidence links it so stable release reviewers can audit blocker burn-down.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-18.1.1 | TEST-18.1.1 | tests/source_inventory_ledger_closure.rs | install, typecheck, unit-test, integration, build | Done |
| AC2 | SCEN-18.1.1 | TEST-18.1.2 | tests/source_inventory_ledger_closure.rs | install, typecheck, unit-test, integration, runtime-smoke, build | Done |
| AC3 | SCEN-18.1.1 | TEST-18.1.3 | tests/source_inventory_ledger_closure.rs | install, typecheck, unit-test, coverage, build | Done |
| AC4 | SCEN-18.1.1 | TEST-18.1.4 | tests/source_inventory_ledger_closure.rs | install, typecheck, unit-test, integration, runtime-smoke, build | Done |

## 8. Risks

- Ledger coverage could be misread as implementation coverage; docs and JSON field names must call generated rows `accounting` and keep P0 blockers.
- Category-level generated P1/P2 rows may hide too much if reason is vague; generated reason must include source reference and task owner.
- Runtime smoke could become noisy if source extraction network fails; keep existing baseline failure behavior and do not weaken it.

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
  - `tests/source_inventory_ledger_closure.rs`
  - `src/compatibility/inventory.rs`
  - `scripts/release/source-inventory-evidence.sh`
  - `scripts/release/runtime-smoke.sh`
  - `docs/specs/tasks/task-18.1-source-inventory-ledger-closure.md`
  - `docs/s2v-adapter.md`
  - `docs/compatibility/matrix.md`
  - `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`
- **commit 列表**：
  - `d281624` `test(compatibility): add SCEN-18.1.1 source ledger RED tests`
  - `3cad24a` `feat(compatibility): add source inventory accounting ledger`
- **§9 Verification 结果**：
  - install: PASS — helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - typecheck: PASS — helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS — helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-18.1.1 ~ TEST-18.1.4 通过。
  - integration: PASS — `bash scripts/release/integration.sh` 通过。
  - coverage: PASS — `bash scripts/release/coverage.sh` 通过；`s2v_coverage_threshold_guard` 通过。
  - build: PASS — helper 执行 `cargo build --workspace`、viewer/npm `pnpm build` 通过。
  - runtime-smoke: PASS — `bash scripts/release/runtime-smoke.sh` 通过；`target/release-gates/source-inventory-evidence.json` status=`ready-with-blockers`，source_extracted_item_count=2549，missing_matrix_rows=0，release_blockers=74；`source-inventory-ledger.json` ledger_item_count=2549，generated_matrix_rows_count=2116，p0_blocker_count=111；`release-candidate.json` 包含 ledger artifact path 且 source_inventory.missing_matrix_rows=0。
- **剩余风险 / 未做项**：本 task 清除了 2116 个 missing-matrix-row silent omission blocker，但没有声称 native parity。仍有 111 个 P0 accounting blockers，其中 74 个来自 generated source-accounting P0 rows，37 个来自已显式分类的 P0 provider module blockers；后续 task 18.2 必须继续燃尽 provider module blockers，task 18.3/18.4 处理 current upstream 与 publication authority。
- **下游 task 影响**：task 18.2 可直接从 `source-inventory-ledger.json` 和 `longtail-classification.json` 读取 P0 blocker；task 18.3 可把 frozen/current target mode 与 ledger evidence 绑定；task 18.4 可在 release candidate 中展示 ledger blocker 与 publication blocker 的区别。
