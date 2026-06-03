# Phase 40: current-latest-evaluator-runtime-fixture-burndown

**Status**: Ready
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Promote the classified current-latest row `eval-runner:src-evaluator-runtime` from explicit P0 blocker to native fixture evidence, reducing local current-latest golden blockers without touching external authority or publication gates. 依据 PRD §Current Latest Rebaseline Addendum、ADR-009、ADR-011、task 29.1 eval-runner fixture contract、task 39.1 completion evidence。

## 2. Business Value

Phase 39 removed the unknown taxonomy blocker but intentionally left evaluator runtime as a P0 blocker. This phase is the next local burn-down step: it ties the new upstream evaluator runtime source to deterministic eval-runner fixture evidence while keeping all provider/config external authority, current-target, publication, and zero-bug claim boundaries intact.

## 3. Scope / Modules

`src/compatibility/inventory.rs`, `scripts/release/current-latest-source-inventory.sh`, `tests/current_latest_evaluator_runtime_fixture.rs`, `target/release-gates/current-latest-source-inventory.json`, `target/release-gates/current-latest-matrix.json`, `target/release-gates/current-latest-golden-corpus.json`, `target/release-gates/current-latest-quality.json`, `docs/compatibility/matrix.md`, `docs/prds/promptfoo-rs.prd.md`, `docs/s2v-adapter.md`, and `test/features/perfect-refactor-parity.feature`.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 40.1 | current-latest-evaluator-runtime-fixture-burndown | ../tasks/task-40.1-current-latest-evaluator-runtime-fixture-burndown.md | Ready | 将 `eval-runner:src-evaluator-runtime` 从 P0 blocker 提升为 native fixture evidence |

## 5. Dependencies

Depends on task 29.1 eval-runner fixture mapping, task 34.1 scheduler/rate-limit fixture evidence, task 39.1 evaluator runtime classification, ADR-009, and ADR-011. No provider credentials, private service account, legal/brand approval, or publication credentials are required.

## 6. Phase Acceptance Criteria

- [ ] `eval-runner:src-evaluator-runtime` carries `level=P0`, `implementation_status=native`, `evidence_kind=fixture`, and `verification_owner=eval-runner`.
- [ ] Rust and shell extractors emit equivalent evaluator runtime fixture evidence.
- [ ] Current-latest golden corpus no longer includes `eval-runner:src-evaluator-runtime` in release blockers.
- [ ] Runtime smoke keeps `perfect_refactor_claim_allowed=false`; this phase only reduces one local eval-runner blocker.

## 7. Phase Risks

- Overclaim risk: this phase may only promote the row if tests prove both extractors produce fixture evidence and golden blockers drop for this item.
- Full promptfoo parity still depends on external config/provider authority, current-target consistency, and publication authority.
- Runtime smoke is slow and must not be skipped.

## 8. Definition of Done

Task 40.1 spec is Done, phase §6 smoke passes with task §9 verification, current-latest golden blockers no longer include `eval-runner:src-evaluator-runtime`, and the repository is clean and pushed.
