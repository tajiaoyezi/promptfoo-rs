# Phase 39: current-latest-evaluator-runtime-classification

**Status**: Done
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Classify the new current-latest upstream source row `src/evaluator/runtime.ts` under an explicit eval-runner evidence contract instead of leaving it as `unclassified:*`. 依据 PRD §Current Latest Rebaseline Addendum、ADR-009、ADR-011、Phase 25 taxonomy contract、Phase 29 eval-runner burndown，以及 Phase 38 runtime-smoke artifacts showing `unclassified:src-evaluator-runtime`.

## 2. Business Value

After the target moved to `promptfoo@0.121.14`, release gates again report source-inventory and matrix blockers because one new evaluator runtime source file is unclassified. This phase removes the unknown taxonomy blocker while preserving honest fail-closed behavior: the evaluator runtime row remains a P0 eval-runner blocker until a dedicated native fixture proves behavior.

## 3. Scope / Modules

`src/compatibility/inventory.rs`, `scripts/release/current-latest-source-inventory.sh`, `tests/current_latest_evaluator_runtime_classification.rs`, `target/release-gates/current-latest-source-inventory.json`, `target/release-gates/current-latest-matrix.json`, `target/release-gates/current-latest-golden-corpus.json`, `target/release-gates/current-latest-quality.json`, `docs/compatibility/matrix.md`, `docs/prds/promptfoo-rs.prd.md`, `docs/s2v-adapter.md`, and `test/features/perfect-refactor-parity.feature`.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 39.1 | current-latest-evaluator-runtime-classification | ../tasks/task-39.1-current-latest-evaluator-runtime-classification.md | Done | 将 `src/evaluator/runtime.ts` 从 unknown taxonomy 转成明确 eval-runner P0 blocker |

## 5. Dependencies

Depends on Phase 25 current-latest source taxonomy, Phase 29 eval-runner evidence mapping, Phase 38 current-latest target refresh, ADR-009, and ADR-011. No provider credentials, private service account, legal/brand approval, or publication credentials are required.

## 6. Phase Acceptance Criteria

- [x] `current-latest-source-inventory.json` and `current-latest-matrix.json` no longer contain `unclassified:src-evaluator-runtime`.
- [x] `src/evaluator/runtime.ts` is classified as `eval-runner` with stable id `eval-runner:src-evaluator-runtime`.
- [x] The evaluator runtime row remains `level=P0`, `implementation_status=blocked`, `evidence_kind=blocker`, and `verification_owner=eval-runner` until dedicated runtime fixture evidence exists.
- [x] Runtime smoke and quality gates keep `perfect_refactor_claim_allowed=false`; this phase removes an unknown taxonomy blocker, not an eval-runtime parity gap.

## 7. Phase Risks

- Marking `src/evaluator/runtime.ts` as native without a dedicated fixture would hide a P0 eval behavior gap. This phase intentionally keeps it blocked.
- The current-latest target can move again; this phase is scoped to the Phase 38 `0.121.14` target packet.
- Removing the source/matrix unknown blocker can reduce quality blocker count, but golden/current-target/external/publication blockers still control completion.

## 8. Definition of Done

Task 39.1 spec is Done, phase §6 smoke passes with the task §9 verification plan, current-latest source inventory and matrix have zero unclassified rows for the Phase 38 target, and the repository is clean and pushed.

## 9. Phase Completion Notes

- **完成日期**：2026-06-03
- **Phase smoke**：PASS - task 39.1 full §9 verification passed with install, lint, typecheck, unit-test, integration, e2e, coverage, build, runtime-smoke, plus `s2v_coverage_threshold_guard`. `s2v_preflight_phase "docs/specs/phases/phase-39-current-latest-evaluator-runtime-classification.md"` passed after this completion update.
- **Artifact evidence**：runtime smoke regenerated `target/release-gates/current-latest-source-inventory.json` with `status=ready`, `unclassified_rows=[]`, and evaluator runtime row `eval-runner:src-evaluator-runtime`. `target/release-gates/current-latest-matrix.json` also reports `status=ready` and `unclassified_rows=[]`. `target/release-gates/current-latest-quality.json` reports `blocker_count=4`, `local_current_latest_ready=false`, and `perfect_refactor_claim_allowed=false`. `target/release-gates/release-candidate.json` keeps `publication_ready=credential-blocked`.
- **Remaining boundaries**：Phase 39 converts an unknown taxonomy row into an explicit P0 eval-runner blocker. Dedicated evaluator runtime fixtures, external authority evidence, publication credentials, and provable zero-bug guarantees remain outside this phase.
