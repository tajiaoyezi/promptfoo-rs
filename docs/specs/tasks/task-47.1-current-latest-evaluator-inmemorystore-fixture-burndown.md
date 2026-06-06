# Task 47.1: current-latest-evaluator-inmemorystore-fixture-burndown

**Status**: Ready
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 47 - current-latest-evaluator-inmemorystore-fixture-burndown
**Dependencies**: task-46.1-current-latest-evaluator-inmemorystore-classification, task-40.1-current-latest-evaluator-runtime-fixture-burndown

## 1. Background

Task 46.1 classified `src/evaluator/inMemoryStore.ts` as `eval-runner:src-evaluator-inmemorystore` and kept it as a P0 blocker. The v1 release authority policy documents this as a planned follow-up once dedicated fixture evidence exists. The next local step is to provide deterministic eval-runner fixture evidence for this in-memory store surface, following the Phase 40 evaluator runtime fixture burndown pattern. 依据 task 46.1、task 40.1、ADR-009、ADR-011、docs/compatibility/v1-release-authority-policy.md。

## 2. Goal

Promote `eval-runner:src-evaluator-inmemorystore` to native fixture evidence in Rust and shell current-latest extraction, and prove the current-latest golden corpus no longer reports that item as a release blocker.

## 3. Scope

### In Scope

- `src/compatibility/inventory.rs`
- `scripts/release/current-latest-source-inventory.sh`
- `scripts/release/generate-v1-authority-manifest.mjs`
- `tests/current_latest_evaluator_inmemorystore_fixture.rs`
- `tests/current_latest_evaluator_inmemorystore_classification.rs`
- `docs/compatibility/authority-decisions.json`
- `docs/compatibility/matrix.md`
- `docs/compatibility/v1-release-authority-policy.md`
- `docs/prds/promptfoo-rs.prd.md`
- `docs/s2v-adapter.md`
- `test/features/perfect-refactor-parity.feature`
- `docs/specs/phases/phase-47-current-latest-evaluator-inmemorystore-fixture-burndown.md`
- `docs/specs/tasks/task-47.1-current-latest-evaluator-inmemorystore-fixture-burndown.md`

### Out Of Scope

- 不解除 config/provider external-authority blockers。
- 不修改 publication authority、Cargo/npm/Docker/Homebrew/GitHub 发布状态。
- 不改变 current-target `current_latest_claim_allowed=false` 语义。
- 不承诺“无任何潜在 bug”或 bug-free。

## 4. Users / Actors

- Eval maintainer: needs evaluator in-memory store to reuse deterministic eval-runner fixture coverage instead of remaining a generic blocker.
- Compatibility reviewer: needs shell and Rust extraction to agree on fixture evidence.
- Release reviewer: needs golden blocker count to drop only for this local row while external blockers remain visible.

## 5. Behavior Contract

`current_latest_eval_runner_fixture_ids("eval-runner:src-evaluator-inmemorystore")` and shell `currentLatestEvalRunnerFixtureIds("eval-runner:src-evaluator-inmemorystore")` must return dedicated evaluator in-memory store fixture evidence. The resulting row must be `level=P0`, `implementation_status=native`, `verification_owner=eval-runner`, `evidence_kind=fixture`, and `evidence_reference=fixture:eval-runner:src-evaluator-inmemorystore`. The current-latest golden corpus must not emit a release blocker for `eval-runner:src-evaluator-inmemorystore`; all unrelated config/provider/current-target/publication blockers remain unchanged.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/tasks/task-46.1-current-latest-evaluator-inmemorystore-classification.md
- docs/specs/tasks/task-40.1-current-latest-evaluator-runtime-fixture-burndown.md
- docs/compatibility/matrix.md
- docs/compatibility/v1-release-authority-policy.md
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

- [ ] **AC1** (task 46.1 / ADR-009): Rust extractor emits `eval-runner:src-evaluator-inmemorystore` as P0 native fixture evidence.
- [ ] **AC2** (task 46.1): shell extractor emits the same evaluator in-memory store native fixture evidence and no blocker evidence for this row.
- [ ] **AC3** (ADR-011): current-latest golden corpus has no release blocker for `eval-runner:src-evaluator-inmemorystore` in an isolated evaluator in-memory store fixture.
- [ ] **AC4** (task 42.1 / task 46.1): runtime smoke for the Phase 42 target reduces current-latest golden blocker count by one for this local row while keeping `perfect_refactor_claim_allowed=false`.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-47.1.1 | TEST-47.1.1 | tests/current_latest_evaluator_inmemorystore_fixture.rs | install, lint, typecheck, unit-test, build | Not Started |
| AC2 | SCEN-47.1.1 | TEST-47.1.2 | tests/current_latest_evaluator_inmemorystore_fixture.rs | install, lint, typecheck, unit-test, integration, build | Not Started |
| AC3 | SCEN-47.1.1 | TEST-47.1.3 | tests/current_latest_evaluator_inmemorystore_fixture.rs | install, typecheck, unit-test, coverage, build | Not Started |
| AC4 | SCEN-47.1.1 | TEST-47.1.4 | tests/current_latest_evaluator_inmemorystore_fixture.rs | install, lint, typecheck, unit-test, e2e, runtime-smoke, build | Not Started |

## 8. Risks

- The task must not promote external config/provider rows; only `eval-runner:src-evaluator-inmemorystore` is in scope.
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
- **改动文件**：
  - <source-file-1>
- **commit 列表**：
  - <hash1> <message>
- **§9 Verification 结果**：
  - install: <TBD-after-impl>
  - lint: <TBD-after-impl>
  - typecheck: <TBD-after-impl>
  - unit-test: <TBD-after-impl>
  - integration: <TBD-after-impl>
  - e2e: <TBD-after-impl>
  - coverage: <TBD-after-impl>
  - build: <TBD-after-impl>
  - runtime-smoke: <TBD-after-impl>
- **剩余风险 / 未做项**：<RISK_OR_NONE>
- **下游 task 影响**：<DOWNSTREAM_OR_NONE>