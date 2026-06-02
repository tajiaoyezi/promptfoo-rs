# Phase 33: current-latest-eval-deletion-burndown

**Status**: Ready
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Burn down the remaining current-latest `cache-store` P0 blocker by implementing and proving local eval deletion lifecycle semantics. The `src/database/evalDeletion.ts` source row becomes P0 native fixture evidence only after SQLite result deletion removes matching eval rows, cascades assertion rows, preserves unrelated eval rows, and Rust/shell current-latest artifacts agree. 依据 PRD §Technical Approach / §Compatibility Matrix、ADR-003、ADR-009、ADR-011、task 5.1、task 13.2、task 31.1、Phase 32 §9。

## 2. Business Value

Phase 32 leaves 41 current-latest P0 golden blockers, including exactly one `cache-store` blocker for eval deletion. Implementing deterministic local eval deletion removes the last local cache-store blocker without touching external authority, provider, script bridge, eval-runner rate-limit, publication, or current-target decisions.

## 3. Scope / Modules

`src/results/sqlite.rs`, `src/results/mod.rs`, `src/compatibility/inventory.rs`, `scripts/release/current-latest-source-inventory.sh`, `tests/current_latest_eval_deletion_burndown.rs`, `target/release-gates/current-latest-source-inventory.json`, `target/release-gates/current-latest-matrix.json`, `target/release-gates/current-latest-golden-corpus.json`, `target/release-gates/current-latest-quality.json`, `docs/compatibility/matrix.md`, `docs/s2v-adapter.md`, `docs/prds/promptfoo-rs.prd.md`, and `test/features/perfect-refactor-parity.feature`.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 33.1 | current-latest-eval-deletion-burndown | ../tasks/task-33.1-current-latest-eval-deletion-burndown.md | Done | 实现 SQLite eval 删除语义并将剩余 cache-store blocker 转为 native fixture evidence |

## 5. Dependencies

Depends on Phase 24 current-latest artifacts, Phase 31 cache-store split, Phase 32 prompt processor phase smoke, task 5.1 result-store schema, task 13.2 eval output/cache parity, ADR-003, ADR-009, and ADR-011. This phase does not require real provider credentials, private services, legal/brand confirmation, or publication authority. Non-cache-store blockers must remain visible.

## 6. Phase Acceptance Criteria

- [ ] SQLite result-store eval deletion removes matching eval records and their assertion rows while preserving unrelated eval records.
- [ ] Missing eval deletion is deterministic and non-destructive.
- [ ] The current-latest eval deletion row has P0 native fixture evidence in both Rust and shell artifacts, and cache-store blockers drop to zero.
- [ ] `current-latest-golden-corpus.json` total blocker count drops from 41 to 40 under the tracked-lock phase smoke target, and `perfect_refactor_claim_allowed=false` remains.

## 7. Phase Risks

- A broad delete could remove unrelated eval history; tests must prove item-level scoping.
- SQLite foreign-key cascade behavior is platform/runtime sensitive if `PRAGMA foreign_keys` is not enabled; implementation must make cascade behavior explicit or test equivalent manual deletion.
- Reducing cache-store blockers does not prove external authority, provider behavior, script bridge runtime discovery, eval-runner adaptive/rate-limit behavior, publication, current-target readiness, or impossible zero-bug claims.

## 8. Definition of Done

Task 33.1 spec is Done, phase §6 smoke passes with `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`, `cache-store=0`, total blockers are 40, and perfect-refactor claim remains blocked by remaining evidence gaps.

## 9. Phase Completion Notes

- **完成日期**：待实施
- **Phase smoke**：待实施
- **Artifact evidence**：待实施
- **保留边界**：待实施
