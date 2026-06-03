# Task 40.1: current-latest-evaluator-runtime-fixture-burndown

**Status**: Ready
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 40 - current-latest-evaluator-runtime-fixture-burndown
**Dependencies**: task-29.1-current-latest-eval-runner-burndown, task-34.1-current-latest-eval-scheduler-rate-limit-burndown, task-39.1-current-latest-evaluator-runtime-classification

## 1. Background

Task 39.1 classified `src/evaluator/runtime.ts` as `eval-runner:src-evaluator-runtime` and kept it as a P0 blocker. The row is now no longer an unknown taxonomy gap, so the next local step is to provide deterministic eval-runner fixture evidence for this runtime surface. Existing eval-runner fixture contracts already cover eval execution, output JSON, retry/timeout, scheduler delay/concurrency/partial failure, and provider wrapper/rate-limit behavior. 依据 task 29.1、task 34.1、task 39.1、ADR-009、ADR-011。

## 2. Goal

Promote `eval-runner:src-evaluator-runtime` to native fixture evidence in Rust and shell current-latest extraction, and prove the current-latest golden corpus no longer reports that item as a release blocker.

## 3. Scope

### In Scope

- `src/compatibility/inventory.rs`
- `scripts/release/current-latest-source-inventory.sh`
- `tests/current_latest_evaluator_runtime_fixture.rs`
- `docs/compatibility/matrix.md`
- `docs/prds/promptfoo-rs.prd.md`
- `docs/s2v-adapter.md`
- `test/features/perfect-refactor-parity.feature`
- `docs/specs/phases/phase-40-current-latest-evaluator-runtime-fixture-burndown.md`
- `docs/specs/tasks/task-40.1-current-latest-evaluator-runtime-fixture-burndown.md`

### Out Of Scope

- 不解除 config/provider external-authority blockers。
- 不修改 publication authority、Cargo/npm/Docker/Homebrew/GitHub 发布状态。
- 不改变 current-target `current_latest_claim_allowed=false` 语义。
- 不承诺“无任何潜在 bug”或 bug-free。

## 4. Users / Actors

- Eval maintainer: needs evaluator runtime to reuse deterministic eval-runner fixture coverage instead of remaining a generic blocker.
- Compatibility reviewer: needs shell and Rust extraction to agree on fixture evidence.
- Release reviewer: needs golden blocker count to drop only for this local row while external blockers remain visible.

## 5. Behavior Contract

`current_latest_eval_runner_fixture_ids("eval-runner:src-evaluator-runtime")` and shell `currentLatestEvalRunnerFixtureIds("eval-runner:src-evaluator-runtime")` must return dedicated evaluator runtime fixture evidence. The resulting row must be `level=P0`, `implementation_status=native`, `verification_owner=eval-runner`, `evidence_kind=fixture`, and `evidence_reference=fixture:eval-runner:src-evaluator-runtime`. The current-latest golden corpus must not emit a release blocker for `eval-runner:src-evaluator-runtime`; all unrelated config/provider/current-target/publication blockers remain unchanged.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/tasks/task-29.1-current-latest-eval-runner-burndown.md
- docs/specs/tasks/task-34.1-current-latest-eval-scheduler-rate-limit-burndown.md
- docs/specs/tasks/task-39.1-current-latest-evaluator-runtime-classification.md
- docs/compatibility/matrix.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `promptfoo_rs::compatibility::inventory::{extract_current_latest_inventory, CurrentLatestTargetLock}`, `promptfoo_rs::compatibility::harness::{build_current_latest_golden_corpus, evaluate_current_latest_release_blockers}`, `serde_json::Value`, `std::path::Path`, `std::process::Command`.
- Shell/tooling commands: `CURRENT_LATEST_TARGET_LOCK_FILE=<path> CURRENT_LATEST_SOURCE_ROOT=<path> CURRENT_LATEST_GATE_DIR=<path> bash scripts/release/current-latest-source-inventory.sh`, `bash scripts/release/current-latest-golden-corpus.sh`, adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke.

### 5.3 函数签名

- `current_latest_eval_runner_fixture_ids(stable_id: &str) -> &'static [&'static str]`
- `is_current_latest_eval_runner_fixture(stable_id: &str, file: &str) -> bool`
- Shell contract: `currentLatestEvalRunnerFixtureIds(id)`, `isCurrentLatestEvalRunnerFixture(id, file)`.
- Test helper contract: `run_current_latest_source_inventory_script(root: &Path, gate_dir: &Path) -> ()`

## 6. Acceptance Criteria

- [ ] **AC1** (task 29.1 / ADR-009): Rust extractor emits `eval-runner:src-evaluator-runtime` as P0 native fixture evidence.
- [ ] **AC2** (task 39.1): shell extractor emits the same evaluator runtime native fixture evidence and no blocker evidence for this row.
- [ ] **AC3** (ADR-011): current-latest golden corpus has no release blocker for `eval-runner:src-evaluator-runtime` in an isolated evaluator runtime fixture.
- [ ] **AC4** (task 38.1 / task 39.1): runtime smoke for the Phase 38 target reduces current-latest golden blocker count by one for this local row while keeping `perfect_refactor_claim_allowed=false`.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-40.1.1 | TEST-40.1.1 | tests/current_latest_evaluator_runtime_fixture.rs | install, lint, typecheck, unit-test, build | Spec Ready |
| AC2 | SCEN-40.1.1 | TEST-40.1.2 | tests/current_latest_evaluator_runtime_fixture.rs | install, lint, typecheck, unit-test, integration, build | Spec Ready |
| AC3 | SCEN-40.1.1 | TEST-40.1.3 | tests/current_latest_evaluator_runtime_fixture.rs | install, typecheck, unit-test, coverage, build | Spec Ready |
| AC4 | SCEN-40.1.1 | TEST-40.1.4 | tests/current_latest_evaluator_runtime_fixture.rs | install, lint, typecheck, unit-test, e2e, runtime-smoke, build | Spec Ready |

## 8. Risks

- The task must not promote external config/provider rows; only `eval-runner:src-evaluator-runtime` is in scope.
- A synthetic or aggregate fixture evidence claim must remain tied to deterministic eval-runner tests; release claims stay blocked until all gates agree.
- Runtime smoke can reveal moving upstream drift; any new upstream target movement must enter a separate S2V task.

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
