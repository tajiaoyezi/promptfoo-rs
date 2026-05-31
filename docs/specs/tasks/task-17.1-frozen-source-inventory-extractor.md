# Task 17.1: frozen-source-inventory-extractor

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 17 — deep-upstream-parity-proof
**Dependencies**: task-16.3-source-extracted-inventory-real-upstream-smoke

## 1. Background

当前审计确认 `source-inventory-evidence.json` 只证明 npm pack 文件列表与本地 44 个 curated inventory item 有引用，不能证明 frozen upstream tag 的 command/provider/assertion/redteam/plugin/strategy/output/config/viewer/API surface 已完整提取。依据 docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md §P0 Item-level upstream inventory、PRD §Compatibility Matrix、ADR-009。

## 2. Goal

实现 frozen upstream source-level inventory extractor，把 promptfoo 0.121.13 tag/package 中可枚举的能力项转成 machine-readable inventory，并让 matrix/release gate 对 silent omissions fail closed。

## 3. Scope

### In Scope

- `compatibility/inventory/`
- `compatibility/matrix/`
- `src/compatibility/inventory.rs`
- `src/compatibility/matrix.rs`
- `scripts/release/source-inventory-evidence.sh`
- `tests/frozen_source_inventory_extractor.rs`
- `docs/compatibility/matrix.md`

### Out Of Scope

- 不重定 baseline 到 moving upstream `main`；只处理 frozen tag `0.121.13` / commit `4860e990c7e9a2f8f677173fb92cf9867b34d03f`。
- 不在本 task 实现每个能力的业务行为；后续 task 17.2 / 17.4 负责 CLI 与长尾 runtime 行为。
- 不需要真实 provider API key；source extraction 只读公开源码/package。

## 4. Users / Actors

- Release maintainer：需要知道矩阵没有漏掉 upstream source-visible capability。
- Compatibility reviewer：需要用 source counts 和 stable ids 审计新增 rows。
- Contributor：需要看到新增 item 的 owner、priority、verification target，避免实现时猜范围。

## 5. Behavior Contract

Extractor 必须从 frozen upstream source/package 读取 command、flag、provider、assertion、redteam plugin、redteam strategy、output、config、viewer/API、example surfaces，输出 `source-extracted-items.json`、counts、source references、baseline metadata、extraction timestamp。任何 extracted item 缺 stable id、category、name、source reference、level hint、owner hint 或 matrix row 时，release gate 必须给出 release-blocking evidence。

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/compatibility/baseline.lock.md
- docs/compatibility/target-policy.md
- docs/specs/tasks/task-11.2-item-level-capability-inventory.md
- docs/specs/tasks/task-11.3-compatibility-matrix-expansion.md
- docs/specs/tasks/task-16.3-source-extracted-inventory-real-upstream-smoke.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`std::fs`、`std::path::{Path, PathBuf}`、`std::process::Command`、`serde::{Deserialize, Serialize}`、`serde_json`、内部模块 `compatibility::inventory`、`compatibility::matrix`。
- Tooling commands：adapter §Commands Install / Typecheck / Unit Test / Integration tests / Coverage / Runtime smoke / Build；Git for Windows Bash 执行 source extraction script。

### 5.3 函数签名

- `FrozenSourceReference::from_baseline_lock(path: &Path) -> Result<FrozenSourceReference, InventoryError>`
- `SourceInventoryExtractor::extract(source: &FrozenSourceReference) -> Result<SourceExtractedInventory, InventoryError>`
- `validate_source_extracted_inventory(inventory: &SourceExtractedInventory, matrix: &CapabilityMatrix) -> SourceInventoryReport`
- `write_source_inventory_evidence(report: &SourceInventoryReport, path: &Path) -> Result<(), InventoryError>`

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Upstream Baseline Freeze Strategy / ADR-009): extraction evidence records frozen version, commit, npm integrity, source acquisition command, extraction timestamp, and refuses `latest` / `main` / floating refs.
- [x] **AC2** (docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md): extractor reports source-visible counts for command-related files, provider files, assertion files, redteam plugin files, redteam strategy files, viewer/app files, and examples; counts below the audit baseline require release-blocking explanation.
- [x] **AC3** (PRD §Compatibility Matrix): every extracted item has stable id, category, name, source reference, P0/P1/P2 hint, owner hint, and matrix row or blocker record.
- [x] **AC4** (ADR-007 / ADR-009): release gate fails when source inventory evidence is missing, stale relative to baseline lock, contains unresolved rows without reason, or silently drops extracted items.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-17.1.1 | TEST-17.1.1 | tests/frozen_source_inventory_extractor.rs | install, typecheck, unit-test, integration, build, runtime-smoke | Done |
| AC2 | SCEN-17.1.1 | TEST-17.1.2 | tests/frozen_source_inventory_extractor.rs | install, typecheck, unit-test, integration, build, runtime-smoke | Done |
| AC3 | SCEN-17.1.1 | TEST-17.1.3 | tests/frozen_source_inventory_extractor.rs | install, typecheck, unit-test, integration, coverage, build, runtime-smoke | Done |
| AC4 | SCEN-17.1.1 | TEST-17.1.4 | tests/frozen_source_inventory_extractor.rs | install, typecheck, unit-test, integration, coverage, build, runtime-smoke | Done |

## 8. Risks

- Upstream package tarball may omit source directories present in git tag; evidence must record which source was used and fail if required categories cannot be inspected.
- Regex-only extraction can overcount or undercount dynamic registries; report must classify ambiguous rows instead of hiding them.
- Large inventory expansion can make existing matrix tests noisy; prefer deterministic stable ids and sorted output.

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
  - `tests/frozen_source_inventory_extractor.rs`
  - `src/compatibility/inventory.rs`
  - `scripts/release/source-inventory-evidence.sh`
  - `docs/compatibility/matrix.md`
  - `docs/s2v-adapter.md`
  - `docs/specs/phases/phase-17-deep-upstream-parity-proof.md`
  - `docs/specs/tasks/task-17.1-frozen-source-inventory-extractor.md`
- **commit 列表**：
  - `3b32ccf` `test(compatibility): add SCEN-17.1.1 frozen source inventory RED tests`
  - `6beb141` `feat(compatibility): implement frozen source inventory extractor`
- **§9 Verification 结果**：
  - install: PASS — helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - typecheck: PASS — helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS — helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-17.1.1 ~ TEST-17.1.4 通过。
  - integration: PASS — `bash scripts/release/integration.sh` 通过，包含 `real_upstream_smoke_gate` contract tests。
  - coverage: PASS — `bash scripts/release/coverage.sh` 通过；`s2v_coverage_threshold_guard` 通过。
  - build: PASS — helper 执行 `cargo build --workspace`、viewer/npm `pnpm build` 通过。
  - runtime-smoke: PASS — `bash scripts/release/runtime-smoke.sh` 通过；`source-inventory-evidence.json` schema=`promptfoo-rs.source-inventory-evidence.v2`，status=`ready-with-blockers`，source-extracted item count=2549，command/provider/assertion/redteam/plugin/strategy/viewer/example counts 均达到或超过 2026-05-31 audit baseline。
- **剩余风险 / 未做项**：source evidence 当前如实记录 2549 个 source-extracted item 缺少 item-level matrix row 的 release blocker；这满足本 task “row or blocker” 契约，但 Phase 17 stable parity 仍需 task 17.4 将 provider/assertion/redteam 长尾分类并清理缺 reason / 缺 matrix row 风险。
- **下游 task 影响**：task 17.2 可使用 frozen source command inventory 与 upstream help 对齐 CLI surface；task 17.3 可引用 v2 evidence 的 frozen baseline metadata；task 17.4 需要消费 `target/release-gates/source-extracted-items.json` 与 blocker 列表完成长尾分类。
