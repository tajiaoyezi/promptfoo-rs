# Phase 46: current-latest-evaluator-inmemorystore-classification

**Status**: Done
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Classify the new current-latest upstream source row `src/evaluator/inMemoryStore.ts` under an explicit eval-runner evidence contract instead of leaving it as `unclassified:*`. 依据 PRD §Current Latest Rebaseline Addendum、ADR-009、ADR-011、Phase 25 taxonomy contract、Phase 29 eval-runner burndown，以及 Phase 42 runtime-smoke artifacts showing `unclassified:src-evaluator-inmemorystore`。

## 2. Business Value

After the target moved to GitHub HEAD `2ca16c59b64e0afca10533de0f817c0d24eba20a`, release gates again report source-inventory and matrix blockers because one new evaluator in-memory store source file is unclassified. This phase removes the unknown taxonomy blocker while preserving honest fail-closed behavior: the in-memory store row remains a P0 eval-runner blocker until dedicated native fixture evidence exists.

## 3. Scope / Modules

`src/compatibility/inventory.rs`, `scripts/release/current-latest-source-inventory.sh`, `tests/current_latest_evaluator_inmemorystore_classification.rs`, `docs/compatibility/authority-decisions.json`, `target/release-gates/current-latest-source-inventory.json`, `target/release-gates/current-latest-matrix.json`, `target/release-gates/current-latest-golden-corpus.json`, `target/release-gates/current-latest-quality.json`, `docs/compatibility/matrix.md`, `docs/prds/promptfoo-rs.prd.md`, `docs/s2v-adapter.md`, and `test/features/perfect-refactor-parity.feature`.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 46.1 | current-latest-evaluator-inmemorystore-classification | ../tasks/task-46.1-current-latest-evaluator-inmemorystore-classification.md | Done | 将 `src/evaluator/inMemoryStore.ts` 从 unknown taxonomy 转成明确 eval-runner P0 blocker |

## 5. Dependencies

Depends on Phase 25 current-latest source taxonomy, Phase 29 eval-runner evidence mapping, Phase 42 current-latest target refresh, ADR-009, and ADR-011. No provider credentials, private service account, legal/brand approval, or publication credentials are required.

## 6. Phase Acceptance Criteria

- [x] `current-latest-source-inventory.json` and `current-latest-matrix.json` no longer contain `unclassified:src-evaluator-inmemorystore`.
- [x] `src/evaluator/inMemoryStore.ts` is classified as `eval-runner` with stable id `eval-runner:src-evaluator-inmemorystore`.
- [x] The in-memory store row remains `level=P0`, `implementation_status=blocked`, `evidence_kind=blocker`, and `verification_owner=eval-runner` until dedicated fixture evidence exists.
- [x] Runtime smoke and quality gates keep `perfect_refactor_claim_allowed=false`; this phase removes an unknown taxonomy blocker, not an in-memory store parity gap.

## 7. Phase Risks

- Marking `src/evaluator/inMemoryStore.ts` as native without a dedicated fixture would hide a P0 eval behavior gap. This phase intentionally keeps it blocked.
- The current-latest target can move again; this phase is scoped to the Phase 42 `2ca16c59` HEAD target packet.
- Removing the source/matrix unknown blocker can reduce quality blocker count, but golden/current-target/external/publication blockers still control completion.

## 8. Definition of Done

Task 46.1 spec is Done, phase §6 smoke passes with the task §9 verification plan, current-latest source inventory and matrix have zero unclassified rows for the Phase 42 target, and the repository is clean and pushed via PR.

## 9. Phase Completion Notes

- **完成日期**：2026-06-06
- **Phase smoke**：PASS — `cargo test --test current_latest_evaluator_inmemorystore_classification`；`bash scripts/release/runtime-smoke.sh` 确认 source inventory / matrix `unclassified_rows=[]`，in-memory store 行 `eval-runner:src-evaluator-inmemorystore` 仍为 `evidence_kind=blocker`，`perfect_refactor_claim_allowed=false`
- **Artifact evidence**：runtime smoke regenerated local `target/release-gates/current-latest-source-inventory.json` with `status=ready`, `unclassified_rows=[]`, and in-memory store row `eval-runner:src-evaluator-inmemorystore`. Matrix also `status=ready`. Quality `blockers.length=4`, golden `blocker_count=25`.
- **Remaining boundaries**：in-memory store native fixture、external authority、publication credentials、Phase 44 closure。