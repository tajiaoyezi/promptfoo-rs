# Task 39.1: current-latest-evaluator-runtime-classification

**Status**: Ready
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 39 - current-latest-evaluator-runtime-classification
**Dependencies**: task-25.1-current-latest-source-taxonomy-burndown, task-29.1-current-latest-eval-runner-burndown, task-38.1-current-latest-0.121.14-target-refresh

## 1. Background

Task 38.1 refreshed current-latest target evidence to `promptfoo@0.121.14`. The refreshed runtime-smoke artifacts show `current-latest-source-inventory.json` and `current-latest-matrix.json` are `ready-with-blockers` because a new upstream file, `src/evaluator/runtime.ts`, appears as `unclassified:src-evaluator-runtime`. Phase 25 requires every current-latest source row to have a deterministic taxonomy, and Phase 29 already treats evaluator/scheduler surfaces as eval-runner evidence. 依据 PRD §Current Latest Rebaseline Addendum、ADR-009、ADR-011、task 25.1、task 29.1、task 38.1 §10 runtime-smoke evidence。

## 2. Goal

Reclassify `src/evaluator/runtime.ts` from `unclassified` to explicit `eval-runner` P0 blocker evidence in both Rust and shell extractors, eliminating source/matrix unknown rows without claiming native eval-runtime parity.

## 3. Scope

### In Scope

- `src/compatibility/inventory.rs`
- `scripts/release/current-latest-source-inventory.sh`
- `tests/current_latest_evaluator_runtime_classification.rs`
- `docs/compatibility/matrix.md`
- `docs/prds/promptfoo-rs.prd.md`
- `docs/s2v-adapter.md`
- `test/features/perfect-refactor-parity.feature`
- `docs/specs/phases/phase-39-current-latest-evaluator-runtime-classification.md`
- `docs/specs/tasks/task-39.1-current-latest-evaluator-runtime-classification.md`

### Out Of Scope

- 不实现 `src/evaluator/runtime.ts` 的 native runtime parity fixture。
- 不降低 `src/evaluator/runtime.ts` 的 P0 level。
- 不解除 current-target、golden corpus、external-authority、publication-authority 或 zero-bug claim blockers。
- 不调用真实 provider、private services、账号、API key 或 publication credentials。

## 4. Users / Actors

- Compatibility maintainer: needs current-latest taxonomy to have zero unknown rows.
- Eval maintainer: needs evaluator runtime to stay visible as a P0 eval-runner gap until behavior fixtures exist.
- Release reviewer: needs quality gate blocker counts to distinguish taxonomy cleanup from parity completion.

## 5. Behavior Contract

The current-latest source inventory extractor must treat `src/evaluator/runtime.ts` as an eval-runner source path. Its stable id must be `eval-runner:src-evaluator-runtime`; its metadata must remain `level=P0`, `implementation_status=blocked`, `verification_owner=eval-runner`, `evidence_kind=blocker`, and `evidence_reference=blocker:eval-runner:src-evaluator-runtime` because no dedicated fixture evidence exists yet. Rust and shell extraction paths must emit equivalent rows, and matrix/source inventory artifacts must no longer contain `unclassified:src-evaluator-runtime`.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/tasks/task-25.1-current-latest-source-taxonomy-burndown.md
- docs/specs/tasks/task-29.1-current-latest-eval-runner-burndown.md
- docs/specs/tasks/task-38.1-current-latest-0.121.14-target-refresh.md
- docs/compatibility/matrix.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `promptfoo_rs::compatibility::inventory::{extract_current_latest_inventory, CurrentLatestTargetLock}`, `serde_json::Value`, `std::collections::BTreeMap`, `std::path::Path`, `std::process::Command`.
- Shell/tooling commands: `CURRENT_LATEST_TARGET_LOCK_FILE=<path> CURRENT_LATEST_SOURCE_ROOT=<path> CURRENT_LATEST_GATE_DIR=<path> bash scripts/release/current-latest-source-inventory.sh`, `bash scripts/release/current-latest-golden-corpus.sh`, `bash scripts/release/current-latest-quality-gate.sh`, adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke.

### 5.3 函数签名

- `is_eval_runtime_file(file: &str) -> bool`
- `current_latest_file_categories(file: &str) -> Vec<&'static str>`
- `current_latest_eval_runner_fixture_ids(stable_id: &str) -> &'static [&'static str]`
- `current_latest_eval_runner_blocker_reason(stable_id: &str, file: &str) -> String`
- Shell contract: `isEvalRuntime(file)`, `currentLatestEvalRunnerFixtureIds(id)`, `currentLatestEvalRunnerBlockerReason(id, file)`, `metadata(category, id, file)`.

## 6. Acceptance Criteria

- [ ] **AC1** (Phase 25 taxonomy): Rust extractor classifies `src/evaluator/runtime.ts` as `category=eval-runner` with stable id `eval-runner:src-evaluator-runtime`, not `unclassified:src-evaluator-runtime`.
- [ ] **AC2** (ADR-009 / task 29.1): the evaluator runtime row remains P0 blocked eval-runner evidence with owner `eval-runner`, `evidence_kind=blocker`, and a blocker reason requiring dedicated current-latest eval-runner runtime fixture evidence.
- [ ] **AC3** (ADR-011): shell source-inventory extraction emits the same evaluator runtime classification as Rust and writes source inventory / matrix artifacts with `unclassified_rows=[]` for this fixture.
- [ ] **AC4** (task 38.1): runtime-smoke artifacts for the Phase 38 target no longer report `unclassified:src-evaluator-runtime` in source inventory or matrix, while `perfect_refactor_claim_allowed=false` remains.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-39.1.1 | TEST-39.1.1 | tests/current_latest_evaluator_runtime_classification.rs | install, lint, typecheck, unit-test, build | Spec Ready |
| AC2 | SCEN-39.1.1 | TEST-39.1.2 | tests/current_latest_evaluator_runtime_classification.rs | install, typecheck, unit-test, coverage, build | Spec Ready |
| AC3 | SCEN-39.1.1 | TEST-39.1.3 | tests/current_latest_evaluator_runtime_classification.rs | install, lint, typecheck, unit-test, integration, build | Spec Ready |
| AC4 | SCEN-39.1.1 | TEST-39.1.4 | tests/current_latest_evaluator_runtime_classification.rs | install, lint, typecheck, unit-test, e2e, runtime-smoke, build | Spec Ready |

## 8. Risks

- If this task marks the row native, it can hide a new upstream evaluator runtime behavior gap. AC2 requires the row to stay blocked.
- Runtime smoke is slow; failures must be debugged rather than marked passed.
- This task can improve source/matrix readiness but cannot resolve external authority, publication credentials, or impossible zero-bug claims.

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
