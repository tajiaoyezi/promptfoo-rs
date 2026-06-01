# Phase 25: current-latest-source-taxonomy-burndown

**Status**: In Progress
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Consume the current-latest quality blocker for 318 unclassified source and matrix rows by turning path-based unknown rows into deterministic capability taxonomy rows with P0/P1/P2 level, implementation status, verification owner, and evidence kind. 依据 Phase 24 §9 `current-latest-quality.json` blocker evidence、PRD §Current Latest Rebaseline Addendum / §Compatibility Matrix、ADR-009、ADR-011。

## 2. Business Value

Phase 24 proved the project cannot claim current-latest perfect refactor readiness while source rows are unknown. This phase removes the "unknown capability" bucket without deleting rows or weakening gates, so the next phase can focus on true native/bridge fixture blockers, external authority blockers, and publication blockers.

## 3. Scope / Modules

`src/compatibility/inventory.rs`、`scripts/release/current-latest-source-inventory.sh`、`scripts/release/current-latest-golden-corpus.sh`、`scripts/release/runtime-smoke.sh`、`tests/current_latest_source_taxonomy_burndown.rs`、`target/release-gates/current-latest-source-inventory.json`、`target/release-gates/current-latest-matrix.json`、`target/release-gates/current-latest-golden-corpus.json`、`target/release-gates/current-latest-quality.json`、`test/features/perfect-refactor-parity.feature`。

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 25.1 | current-latest-source-taxonomy-burndown | ../tasks/task-25.1-current-latest-source-taxonomy-burndown.md | In Progress | 将 current-latest source/matrix 的 unclassified rows 降为 0，并保留真实 P0/external/publication blockers |

## 5. Dependencies

依赖 Phase 24 current-latest target lock、source inventory、matrix、golden corpus 和 quality gate。该 phase 不需要真实 provider credentials、发布凭据或法律/品牌确认；这些仍由现有 external/publication gates 阻塞。

## 6. Phase Acceptance Criteria

- [ ] `target/release-gates/current-latest-source-inventory.json` 与 `current-latest-matrix.json` 的 `unclassified_rows` 均为空，且 row_count 不因删除 unknown rows 下降。
- [ ] 新 taxonomy 对 `src/util`、`src/redteam`、`src/types`、`src/server`、`src/scheduler`、`src/prompts`、`src/matchers`、`src/database/storage/cache`、script bridge、cloud/share/integration 等 current-latest source families 给出 owner、level 和 evidence kind。
- [ ] golden corpus 不再因 `category=unclassified` 产生 `Unclassified` blocker；剩余 P0 blocker 必须继续以 `Bug` 或 explicit blocker reason 暴露。
- [ ] `current-latest-quality.json` 仍禁止 perfect-refactor claim，除非 source/matrix/golden/current-target/external/publication blockers 全部清零或正式 waiver。

## 7. Phase Risks

- 过宽分类会把真实 P0 runtime 行降级成 P1/P2。分类规则必须按 PRD 模块边界保守处理 eval/config/cache/provider/assertion/redteam/script bridge。
- 该 phase 只消除 unknown taxonomy blocker，不代表 native behavior parity 完成。
- Upstream target lock 仍是 Phase 24 的 immutable packet；不得改回 floating latest/main/HEAD。

## 8. Definition of Done

Task 25.1 spec is Done, phase §6 smoke passes with `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`, current-latest source/matrix unclassified rows are zero, and remaining quality blockers are explicit native/bridge/external/publication work rather than unknown rows.

## 9. Phase Completion Notes

待实施。
