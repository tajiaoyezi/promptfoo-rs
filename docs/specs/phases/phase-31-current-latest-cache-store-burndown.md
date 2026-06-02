# Phase 31: current-latest-cache-store-burndown

**Status**: In Progress
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Burn down current-latest `cache-store` P0 golden blockers by applying already-proven cache key, resume, JSONL/SQLite result-store, and eval output cache evidence to the locked current-latest inventory. Rows covered by deterministic local fixtures become P0 native fixture evidence; test/signal helper rows move to P1 snapshot evidence; eval deletion remains an explicit P0 blocker until deletion semantics are implemented and golden-diffed. 依据 PRD §Technical Approach / §Compatibility Matrix、ADR-003、ADR-009、ADR-011、task 3.2、task 5.1、task 13.2、Phase 30 §9。

## 2. Business Value

After Phase 30, phase-smoke artifacts report 52 current-latest P0 golden blockers, including 9 `cache-store` blockers. Existing cache/resume/result-store fixtures already prove local cache key, resume, JSONL append, SQLite query, and eval output cache behavior. Reusing that evidence prevents locally proven storage behavior from staying as generic missing evidence while preserving deletion and helper semantics that still need dedicated tests.

## 3. Scope / Modules

`src/compatibility/inventory.rs`, `scripts/release/current-latest-source-inventory.sh`, `tests/current_latest_cache_store_burndown.rs`, `target/release-gates/current-latest-source-inventory.json`, `target/release-gates/current-latest-matrix.json`, `target/release-gates/current-latest-golden-corpus.json`, `target/release-gates/current-latest-quality.json`, `docs/compatibility/matrix.md`, and `test/features/perfect-refactor-parity.feature`.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 31.1 | current-latest-cache-store-burndown | ../tasks/task-31.1-current-latest-cache-store-burndown.md | Done | 将 current-latest 9 个 cache-store blockers 拆为 6 个 fixture-covered rows、2 个 P1 snapshot rows 和 1 个保留 P0 blocker |

## 5. Dependencies

依赖 Phase 24 current-latest target artifacts、Phase 25 taxonomy burndown、Phase 30 prompt-processing burndown、task 3.2 cache/resume/retry、task 5.1 result-store schema、task 13.2 eval output/cache parity、ADR-003、ADR-009 和 ADR-011。该 phase 不需要真实 provider credentials、外部账号、私有服务、法律/品牌确认或 publication credentials；未证明的 eval deletion semantics 必须继续阻塞。

## 6. Phase Acceptance Criteria

- [ ] current-latest cache, database index/tables, and local filesystem storage rows already covered by local deterministic cache/result-store fixtures no longer appear as generic P0 cache-store blockers and carry `fixture:` evidence references.
- [ ] current-latest database testing and signal helper rows are recorded as P1 snapshot evidence rather than P0 native parity.
- [ ] current-latest eval deletion remains an explicit P0 cache-store blocker.
- [ ] `current-latest-golden-corpus.json` cache-store blocker count drops from 9 to 1, total blocker count drops from 52 to 44 under the tracked-lock phase smoke target, and `perfect_refactor_claim_allowed=false` remains.

## 7. Phase Risks

- Broadly marking all `src/database/**` native would hide deletion, lifecycle, and helper behavior gaps. Classification must use explicit fixture, snapshot, and blocker allowlists.
- Local filesystem storage evidence is not publication or cloud storage authority; it is only local result/cache persistence evidence.
- Reducing cache-store blockers does not prove provider external authority, script bridge runtime discovery, prompt processor parity, eval runner adaptive/rate-limit behavior, publication, or impossible zero-bug claims.

## 8. Definition of Done

Task 31.1 spec is Done, phase §6 smoke passes with `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`, fixture-covered cache-store rows stop generating P0 golden findings, eval deletion remains visible, and perfect-refactor claim remains blocked by remaining evidence gaps.

## 9. Phase Completion Notes

- **完成日期**：待实施
- **Phase smoke**：待实施
- **Artifact evidence**：待实施
- **保留边界**：待实施
