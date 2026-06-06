# Phase 47: current-latest-evaluator-inmemorystore-fixture-burndown

**Status**: Done
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Promote the classified current-latest row `eval-runner:src-evaluator-inmemorystore` from explicit P0 blocker to native fixture evidence, reducing local current-latest golden blockers without touching external authority or publication gates. 依据 PRD §Current Latest Rebaseline Addendum、ADR-009、ADR-011、task 46.1 classification evidence、docs/compatibility/v1-release-authority-policy.md §Evaluator in-memory store。

## 2. Business Value

Phase 46 removed the unknown taxonomy blocker but intentionally left evaluator in-memory store as a P0 blocker. This phase is the planned v1 follow-up: it ties `src/evaluator/inMemoryStore.ts` to deterministic eval-runner fixture evidence while keeping all provider/config external authority, current-target, publication, and zero-bug claim boundaries intact.

## 3. Scope / Modules

`src/compatibility/inventory.rs`, `scripts/release/current-latest-source-inventory.sh`, `scripts/release/generate-v1-authority-manifest.mjs`, `tests/current_latest_evaluator_inmemorystore_fixture.rs`, `tests/current_latest_evaluator_inmemorystore_classification.rs`, `target/release-gates/current-latest-source-inventory.json`, `target/release-gates/current-latest-matrix.json`, `target/release-gates/current-latest-golden-corpus.json`, `target/release-gates/current-latest-quality.json`, `docs/compatibility/authority-decisions.json`, `docs/compatibility/matrix.md`, `docs/compatibility/v1-release-authority-policy.md`, `docs/prds/promptfoo-rs.prd.md`, `docs/s2v-adapter.md`, and `test/features/perfect-refactor-parity.feature`.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 47.1 | current-latest-evaluator-inmemorystore-fixture-burndown | ../tasks/task-47.1-current-latest-evaluator-inmemorystore-fixture-burndown.md | Done | 将 `eval-runner:src-evaluator-inmemorystore` 从 P0 blocker 提升为 native fixture evidence |

## 5. Dependencies

Depends on task 46.1 evaluator in-memory store classification, task 40.1 evaluator runtime fixture burndown pattern, ADR-009, and ADR-011. No provider credentials, private service account, legal/brand approval, or publication credentials are required.

## 6. Phase Acceptance Criteria

- [x] `eval-runner:src-evaluator-inmemorystore` carries `level=P0`, `implementation_status=native`, `evidence_kind=fixture`, and `verification_owner=eval-runner`.
- [x] Rust and shell extractors emit equivalent evaluator in-memory store fixture evidence.
- [x] Current-latest golden corpus no longer includes `eval-runner:src-evaluator-inmemorystore` in release blockers.
- [x] Runtime smoke keeps `perfect_refactor_claim_allowed=false`; this phase only reduces one local eval-runner blocker.

## 7. Phase Risks

- Overclaim risk: this phase may only promote the row if tests prove both extractors produce fixture evidence and golden blockers drop for this item.
- Full promptfoo parity still depends on external config/provider authority, current-target consistency, and publication authority.
- Runtime smoke is slow and must not be skipped.

## 8. Definition of Done

Task 47.1 spec is Done, phase §6 smoke passes with task §9 verification, current-latest golden blockers no longer include `eval-runner:src-evaluator-inmemorystore`, and the repository is clean and pushed via PR.

## 9. Phase Completion Notes

- **完成日期**：2026-06-06
- **Phase smoke**：PASS — task 47.1 full §9 verification；`eval-runner:src-evaluator-inmemorystore` 为 `native`/`fixture`；golden `blocker_count=24`；`perfect_refactor_claim_allowed=false`
- **Artifact evidence**：runtime smoke regenerated local gates with in-memory store row `evidence_reference=fixture:eval-runner:src-evaluator-inmemorystore` and empty diff findings for that row.
- **Remaining boundaries**：config/provider external authority、publication 聚合 ready、current-target claim 与 perfect-refactor 完成声明仍未解除。