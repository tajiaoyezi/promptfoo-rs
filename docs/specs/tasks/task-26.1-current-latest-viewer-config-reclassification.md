# Task 26.1: current-latest-viewer-config-reclassification

**Status**: Ready
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 26 — current-latest-viewer-config-reclassification
**Dependencies**: task-25.1-current-latest-source-taxonomy-burndown, task-19.1-viewer-config-source-reclassification

## 1. Background

Phase 25 eliminated current-latest `unclassified` rows, but the generated artifacts still show P0 `config` blockers for upstream `src/app/**` files such as `src/app/postcss.config.js` and React viewer components whose filenames or paths include `config`. Task 19.1 already established the frozen-baseline rule: upstream `src/app/**` config/editor/test source belongs to Local Web viewer P1 accounting, while non-app config rows remain P0 core config blockers. This task applies that rule to the current-latest target without deleting rows or weakening non-app config parity. 依据 PRD §Compatibility Matrix / §Current Latest Rebaseline Addendum、ADR-009、ADR-011、Phase 25 §10 artifact evidence、task 19.1。

## 2. Goal

Remove duplicate current-latest P0 `config` blockers for `src/app/**` viewer config files while preserving viewer evidence rows and all non-app P0 config blockers.

## 3. Scope

### In Scope

- `src/compatibility/inventory.rs`
- `scripts/release/current-latest-source-inventory.sh`
- `tests/current_latest_viewer_config_reclassification.rs`
- `target/release-gates/current-latest-source-inventory.json`
- `target/release-gates/current-latest-matrix.json`
- `target/release-gates/current-latest-golden-corpus.json`
- `target/release-gates/current-latest-quality.json`
- `test/features/perfect-refactor-parity.feature`

### Out Of Scope

- 不实现 upstream React UI 像素级 parity。
- 不降级 `src/util/config/**`、`src/commands/config.ts`、`src/configTypes.ts`、`src/globalConfig/**`、`src/server/config/**` 或其他 non-app runtime config rows。
- 不解决 provider、eval-runner、cache-store、prompt-processing、script-bridge、external authority 或 publication blockers。
- 不承诺“无任何潜在 bug”；claim 仍受 task 24.4 / ADR-011 的 evidence boundary 约束。

## 4. Users / Actors

- Compatibility maintainer: needs false P0 config blockers separated from real current-latest core config work.
- Release reviewer: needs blocker count reductions to be backed by path-scoped evidence rather than broad demotion.
- Future implementer: needs remaining config blockers to represent actual config-loader and runtime parity gaps.

## 5. Behavior Contract

The current-latest inventory extractor must treat files under `src/app/**` as viewer source even when their file name or directory contains `config`. Such files must not produce `category=config` rows or P0 config blockers, but they must still produce viewer rows with P1 viewer evidence. Files outside `src/app/**` that match config naming/path conventions must continue to produce P0 config rows with fixture/blocker evidence unless a separate task adds executable fixture coverage. The shell inventory script and Rust extractor must use equivalent classification semantics.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/phases/phase-24-current-latest-perfect-refactor.md
- docs/specs/phases/phase-25-current-latest-source-taxonomy-burndown.md
- docs/specs/tasks/task-19.1-viewer-config-source-reclassification.md
- docs/specs/tasks/task-24.4-current-latest-exhaustive-quality-gate.md
- docs/specs/tasks/task-25.1-current-latest-source-taxonomy-burndown.md
- docs/compatibility/matrix.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `promptfoo_rs::compatibility::inventory::{extract_current_latest_inventory, reconcile_current_latest_matrix, write_current_latest_inventory_artifacts, CurrentLatestTargetLock}`, `serde_json::Value`, `std::collections::BTreeSet`, `std::path::Path`, `std::process::Command`.
- Tooling commands: `bash scripts/release/current-latest-source-inventory.sh`, `bash scripts/release/current-latest-golden-corpus.sh`, `bash scripts/release/current-latest-quality-gate.sh`, `s2v_verify_full "$(s2v_extract_verify_keys "$TASK_SPEC")"`, adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke.

### 5.3 函数签名

- `current_latest_file_categories(file: &str) -> Vec<&'static str>`
- `is_current_latest_viewer_config_file(file: &str) -> bool`
- `is_config_file(file: &str) -> bool`
- Shell contract: `bash scripts/release/current-latest-source-inventory.sh`

## 6. Acceptance Criteria

- [ ] **AC1** (PRD §Compatibility Matrix / Local Web viewer): current-latest `src/app/**` config-named files produce viewer evidence rows and no duplicate P0 `config` rows.
- [ ] **AC2** (PRD §Core Capabilities / ADR-009): non-app config rows remain P0 config fixture/blocker rows; no blanket config demotion is allowed.
- [ ] **AC3** (Phase 25 §9 artifact evidence): source inventory, matrix, golden corpus, and quality artifacts show app viewer config duplicate blockers removed while row accounting remains complete and evidence is non-empty.
- [ ] **AC4** (task 24.4 claim contract / ADR-011): quality gate still rejects perfect-refactor completion while real current-latest P0 fixture, external authority, current-target, or publication blockers remain.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-26.1.1 | TEST-26.1.1 | tests/current_latest_viewer_config_reclassification.rs | install, typecheck, unit-test, integration, build | Ready |
| AC2 | SCEN-26.1.1 | TEST-26.1.2 | tests/current_latest_viewer_config_reclassification.rs | install, lint, typecheck, unit-test, coverage, build | Ready |
| AC3 | SCEN-26.1.1 | TEST-26.1.3 | tests/current_latest_viewer_config_reclassification.rs | install, typecheck, unit-test, e2e, runtime-smoke, build | Ready |
| AC4 | SCEN-26.1.1 | TEST-26.1.4 | tests/current_latest_viewer_config_reclassification.rs | install, lint, typecheck, unit-test, runtime-smoke, build | Ready |

## 8. Risks

- If path matching checks only for the word `app`, server/runtime config files could be misclassified. The rule must be exact-prefix `src/app/`.
- Reclassification can reduce the blocker count without adding native behavior coverage. AC4 requires the current-latest claim to remain false.
- Shell/Rust drift would make local tests pass while runtime smoke artifacts regress. Tests must cover both the Rust extractor and script-generated artifacts.

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

- 本 task 完成时按 AGENTS.md §10 schema 回填完成日期、改动文件、commit 列表、§9 Verification 结果、剩余风险和下游 task 影响。
