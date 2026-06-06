# Task 46.1: current-latest-evaluator-inmemorystore-classification

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 46 - current-latest-evaluator-inmemorystore-classification
**Dependencies**: task-25.1-current-latest-source-taxonomy-burndown, task-29.1-current-latest-eval-runner-burndown, task-42.1-current-latest-2ca16c-head-refresh

## 1. Background

Task 42.1 refreshed current-latest target evidence to GitHub HEAD `2ca16c59b64e0afca10533de0f817c0d24eba20a`. The refreshed runtime-smoke artifacts show `current-latest-source-inventory.json` and `current-latest-matrix.json` are `ready-with-blockers` because a new upstream file, `src/evaluator/inMemoryStore.ts`, appears as `unclassified:src-evaluator-inmemorystore`. Phase 25 requires every current-latest source row to have a deterministic taxonomy, and Phase 29 already treats evaluator surfaces as eval-runner evidence. 依据 PRD §Current Latest Rebaseline Addendum、ADR-009、ADR-011、task 25.1、task 29.1、task 42.1 §10 runtime-smoke evidence。

## 2. Goal

Reclassify `src/evaluator/inMemoryStore.ts` from `unclassified` to explicit `eval-runner` P0 blocker evidence in both Rust and shell extractors, eliminating source/matrix unknown rows without claiming native in-memory store parity.

## 3. Scope

### In Scope

- `src/compatibility/inventory.rs`
- `scripts/release/current-latest-source-inventory.sh`
- `tests/current_latest_evaluator_inmemorystore_classification.rs`
- `docs/compatibility/authority-decisions.json`
- `docs/compatibility/matrix.md`
- `docs/prds/promptfoo-rs.prd.md`
- `docs/s2v-adapter.md`
- `test/features/perfect-refactor-parity.feature`
- `docs/specs/phases/phase-46-current-latest-evaluator-inmemorystore-classification.md`
- `docs/specs/tasks/task-46.1-current-latest-evaluator-inmemorystore-classification.md`

### Out Of Scope

- 不实现 `src/evaluator/inMemoryStore.ts` 的 native in-memory store parity fixture。
- 不降低 `src/evaluator/inMemoryStore.ts` 的 P0 level。
- 不解除 current-target、golden corpus、external-authority、publication-authority 或 zero-bug claim blockers。
- 不调用真实 provider、private services、账号、API key 或 publication credentials。

## 4. Users / Actors

- Compatibility maintainer: needs current-latest taxonomy to have zero unknown rows.
- Eval maintainer: needs evaluator in-memory store to stay visible as a P0 eval-runner gap until behavior fixtures exist.
- Release reviewer: needs quality gate blocker counts to distinguish taxonomy cleanup from parity completion.

## 5. Behavior Contract

The current-latest source inventory extractor must treat `src/evaluator/inMemoryStore.ts` as an eval-runner source path. Its stable id must be `eval-runner:src-evaluator-inmemorystore`; its metadata must remain `level=P0`, `implementation_status=blocked`, `verification_owner=eval-runner`, `evidence_kind=blocker`, and `evidence_reference=blocker:eval-runner:src-evaluator-inmemorystore` because no dedicated fixture evidence exists yet. Rust and shell extraction paths must emit equivalent rows, and matrix/source inventory artifacts must no longer contain `unclassified:src-evaluator-inmemorystore`.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/tasks/task-25.1-current-latest-source-taxonomy-burndown.md
- docs/specs/tasks/task-29.1-current-latest-eval-runner-burndown.md
- docs/specs/tasks/task-42.1-current-latest-2ca16c-head-refresh.md
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
- `current_latest_eval_runner_blocker_reason(stable_id: &str, file: &str) -> String`
- Shell contract: `isEvalRuntime(file)`, `currentLatestEvalRunnerBlockerReason(id, file)`, `metadata(category, id, file)`.

## 6. Acceptance Criteria

- [x] **AC1** (Phase 25 taxonomy): Rust extractor classifies `src/evaluator/inMemoryStore.ts` as `category=eval-runner` with stable id `eval-runner:src-evaluator-inmemorystore`, not `unclassified:src-evaluator-inmemorystore`.
- [x] **AC2** (ADR-009 / task 29.1): the in-memory store row remains P0 blocked eval-runner evidence with owner `eval-runner`, `evidence_kind=blocker`, and a blocker reason requiring dedicated current-latest eval-runner in-memory store fixture evidence.
- [x] **AC3** (ADR-011): shell source-inventory extraction emits the same in-memory store classification as Rust and writes source inventory / matrix artifacts with `unclassified_rows=[]` for this fixture.
- [x] **AC4** (task 42.1): runtime-smoke artifacts for the Phase 42 target no longer report `unclassified:src-evaluator-inmemorystore` in source inventory or matrix, while `perfect_refactor_claim_allowed=false` remains.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-46.1.1 | TEST-46.1.1 | tests/current_latest_evaluator_inmemorystore_classification.rs | install, lint, typecheck, unit-test, build | Done |
| AC2 | SCEN-46.1.1 | TEST-46.1.2 | tests/current_latest_evaluator_inmemorystore_classification.rs | install, typecheck, unit-test, coverage, build | Done |
| AC3 | SCEN-46.1.1 | TEST-46.1.3 | tests/current_latest_evaluator_inmemorystore_classification.rs | install, lint, typecheck, unit-test, integration, build | Done |
| AC4 | SCEN-46.1.1 | TEST-46.1.4 | tests/current_latest_evaluator_inmemorystore_classification.rs | install, lint, typecheck, unit-test, e2e, runtime-smoke, build | Done |

## 8. Risks

- If this task marks the row native, it can hide a new upstream evaluator in-memory store behavior gap. AC2 requires the row to stay blocked.
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

- **完成日期**：2026-06-06
- **改动文件**：
  - `src/compatibility/inventory.rs`
  - `scripts/release/current-latest-source-inventory.sh`
  - `tests/current_latest_evaluator_inmemorystore_classification.rs`
  - `docs/compatibility/authority-decisions.json`
  - `docs/compatibility/matrix.md`
  - `docs/prds/promptfoo-rs.prd.md`
  - `docs/s2v-adapter.md`
  - `test/features/perfect-refactor-parity.feature`
  - `docs/specs/phases/phase-46-current-latest-evaluator-inmemorystore-classification.md`
  - `docs/specs/tasks/task-46.1-current-latest-evaluator-inmemorystore-classification.md`
- **commit 列表**：
  - `fd11019` feat(inventory): classify evaluator inMemoryStore as eval-runner P0 blocker
  - `9028f51` docs(spec): complete task-46.1 and Phase 46
- **§9 Verification 结果**：
  - install: ✅
  - lint: ✅
  - typecheck: ✅
  - unit-test: all passed / 0 failed
  - integration: ✅
  - e2e: ✅
  - coverage: ✅
  - build: ✅
  - runtime-smoke: PASS — source inventory `status=ready` `unclassified_rows=[]`, matrix `status=ready` `unclassified_rows=[]`, in-memory store row `eval-runner:src-evaluator-inmemorystore` `evidence_kind=blocker`, golden `blocker_count=25`, quality `blockers.length=4`, `perfect_refactor_claim_allowed=false`
- **剩余风险 / 未做项**：`eval-runner:src-evaluator-inmemorystore` 仍为 P0 blocker，需后续 fixture burndown；Phase 44 外部 authority / publication evidence 仍等待 maintainer。
- **下游 task 影响**：无新 task spec；后续可选 in-memory store fixture burndown task（类比 task 40.1）。