# Task 33.1: current-latest-eval-deletion-burndown

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 33 - current-latest-eval-deletion-burndown
**Dependencies**: task-5.1-result-store-schema, task-13.2-eval-output-cache-parity, task-24.4-current-latest-exhaustive-quality-gate, task-31.1-current-latest-cache-store-burndown, task-32.1-current-latest-local-prompt-processor-burndown

## 1. Background

Phase 32 leaves tracked-lock phase-smoke artifacts at 41 P0 golden blockers, including exactly one `cache-store` blocker: `cache-store:src-database-evaldeletion`. Task 5.1 already proves SQLite result schema and assertion rows; task 13.2 proves eval output/cache behavior; task 31.1 intentionally left eval deletion blocked until dedicated deletion lifecycle evidence exists. This task adds that evidence without changing external-authority or non-cache-store blockers. 依据 PRD §Technical Approach / §Compatibility Matrix、ADR-003、ADR-009、ADR-011、task 5.1、task 13.2、task 31.1、Phase 32 §9。

## 2. Goal

Implement deterministic local SQLite eval deletion semantics and promote only the current-latest eval deletion row to P0 native fixture evidence, reducing cache-store blockers from 1 to 0 and total blockers from 41 to 40.

## 3. Scope

### In Scope

- `src/results/sqlite.rs`
- `src/results/mod.rs`
- `src/compatibility/inventory.rs`
- `scripts/release/current-latest-source-inventory.sh`
- `tests/current_latest_eval_deletion_burndown.rs`
- `target/release-gates/current-latest-source-inventory.json`
- `target/release-gates/current-latest-matrix.json`
- `target/release-gates/current-latest-golden-corpus.json`
- `target/release-gates/current-latest-quality.json`
- `docs/compatibility/matrix.md`
- `docs/s2v-adapter.md`
- `docs/prds/promptfoo-rs.prd.md`
- `test/features/perfect-refactor-parity.feature`

### Out Of Scope

- 不实现 remote/cloud eval deletion 或 upload/mutation 行为。
- 不修改 unsupported top-level `delete` command policy。
- 不解决 config/provider external authority、eval-runner adaptive/rate-limit、script bridge runtime discovery、JS/Python/executable prompt processor parity、current-target drift、publication authority 或“无任何潜在 bug”承诺。

## 4. Users / Actors

- Local operator: needs local eval deletion to remove only the selected eval's stored records.
- Release reviewer: needs cache-store blocker reduction to be backed by deletion lifecycle tests, not broad classification.
- Data reviewer: needs assertion rows and unrelated eval rows preserved or removed according to explicit SQLite semantics.

## 5. Behavior Contract

`SqliteResultStore::delete_eval(eval_id)` must delete all `result_records` for the supplied eval id and remove associated `assertion_results` without deleting unrelated eval rows. Deleting a missing eval id must be a successful no-op returning zero deleted records. Current-latest `cache-store:src-database-evaldeletion` must classify as `level=P0`, `implementation_status=native`, `verification_owner=cache-resume-store`, `evidence_kind=fixture`, and `evidence_reference=fixture:cache-store:src-database-evaldeletion`. Rust and shell artifact generation must use equivalent rules.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/phases/phase-24-current-latest-perfect-refactor.md
- docs/specs/phases/phase-31-current-latest-cache-store-burndown.md
- docs/specs/phases/phase-32-current-latest-local-prompt-processor-burndown.md
- docs/specs/tasks/task-5.1-result-store-schema.md
- docs/specs/tasks/task-13.2-eval-output-cache-parity.md
- docs/specs/tasks/task-24.4-current-latest-exhaustive-quality-gate.md
- docs/specs/tasks/task-31.1-current-latest-cache-store-burndown.md
- docs/specs/tasks/task-32.1-current-latest-local-prompt-processor-burndown.md
- docs/compatibility/current-latest.lock.md
- docs/compatibility/matrix.md
- docs/decisions/adr-003-local-first-storage.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `promptfoo_rs::results::{AssertionResultRecord, ResultQuery, ResultRecord, ResultStatus, SqliteResultStore}`, `promptfoo_rs::compatibility::inventory::{extract_current_latest_inventory, CurrentLatestTargetLock}`, `serde_json::Value`, `std::path::Path`, `std::process::Command`.
- Tooling commands: `bash scripts/release/current-latest-source-inventory.sh`, `bash scripts/release/current-latest-golden-corpus.sh`, `bash scripts/release/current-latest-quality-gate.sh`, `s2v_verify_full "$(s2v_extract_verify_keys "$TASK_SPEC")"`, adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke.

### 5.3 函数签名

- `impl SqliteResultStore { pub fn delete_eval(&self, eval_id: &str) -> Result<u64, StoreError> }`
- `is_current_latest_cache_store_fixture(stable_id: &str, file: &str) -> bool`
- Shell contract: `isCurrentLatestCacheStoreFixture(id, file)`, `metadata(category, id, file)`.

## 6. Acceptance Criteria

- [x] **AC1** (task 5.1 / ADR-003): `SqliteResultStore::delete_eval` removes all rows for the selected eval id and removes their assertion rows.
- [x] **AC2** (task 13.2 / ADR-003): deleting a missing eval id returns zero and preserves unrelated eval output/cache rows.
- [x] **AC3** (ADR-009 / ADR-011): current-latest eval deletion row has P0 native fixture evidence and no cache-store blocker remains.
- [x] **AC4** (ADR-011 / task 24.4): Rust extractor and shell extractor emit equivalent eval deletion classification, total blockers drop to 40, and perfect-refactor completion remains false.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-33.1.1 | TEST-33.1.1 | tests/current_latest_eval_deletion_burndown.rs | install, lint, typecheck, unit-test, integration, build | Done |
| AC2 | SCEN-33.1.1 | TEST-33.1.2 | tests/current_latest_eval_deletion_burndown.rs | install, typecheck, unit-test, e2e, build | Done |
| AC3 | SCEN-33.1.1 | TEST-33.1.3 | tests/current_latest_eval_deletion_burndown.rs | install, lint, typecheck, unit-test, coverage, build | Done |
| AC4 | SCEN-33.1.1 | TEST-33.1.4 | tests/current_latest_eval_deletion_burndown.rs | install, lint, typecheck, unit-test, integration, e2e, runtime-smoke, build | Done |

## 8. Risks

- Deletion must be eval-scoped; case/provider scoped deletes or broad file deletes would violate local history safety.
- Assertion cascade must be proven directly; relying on SQLite defaults without enabling cascade can leave orphan rows.
- This task removes only the cache-store eval deletion blocker and does not affect external authority, provider, script bridge, eval-runner, publication, current-target, or impossible zero-bug claim blockers.

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

- **完成日期**：2026-06-02
- **改动文件**：
  - `docs/specs/phases/phase-33-current-latest-eval-deletion-burndown.md`
  - `docs/specs/tasks/task-33.1-current-latest-eval-deletion-burndown.md`
  - `docs/s2v-adapter.md`
  - `docs/prds/promptfoo-rs.prd.md`
  - `docs/compatibility/matrix.md`
  - `test/features/perfect-refactor-parity.feature`
  - `tests/current_latest_eval_deletion_burndown.rs`
  - `tests/current_latest_cache_store_burndown.rs`
  - `src/results/sqlite.rs`
  - `src/compatibility/inventory.rs`
  - `scripts/release/current-latest-source-inventory.sh`
- **commit 列表**：
  - `c68e774` `docs(spec): add phase 33 current latest eval deletion burndown`
  - `d8362e3` `docs(spec): task-33.1 enters implementation`
  - `7fc656f` `test(cache-store): add current latest eval deletion RED tests`
  - `c1a0abd` `feat(cache-store): implement current latest eval deletion evidence`
  - 本次 docs 回填提交：`docs(spec): complete task 33.1 current latest eval deletion burndown`
- **§9 Verification 结果**：
  - install: PASS - helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - lint: PASS - helper 执行 `bash scripts/release/lint.sh` 通过。
  - typecheck: PASS - helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS - helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-33.1.1 ~ TEST-33.1.4 与累计 TEST-31.1.1 ~ TEST-31.1.5 均通过。
  - integration: PASS - helper 执行 adapter Integration tests 通过。
  - e2e: PASS - helper 执行 adapter E2E tests 通过。
  - coverage: PASS - helper 执行 adapter Coverage，通过覆盖率阈值守卫。
  - build: PASS - helper 执行 adapter Build 通过。
  - runtime-smoke: PASS - helper 执行 adapter Runtime smoke 通过；`current-latest-golden-corpus.json` 为 `ready-with-blockers`、`p0_total=92`、`fixture_case_count=92`、`blocker_count=40`、分组 `config=7, eval-runner=7, prompt-processing=3, provider=16, script-bridge=7`，`cache-store=0`，`perfect_refactor_claim_allowed=false`。
- **剩余风险 / 未做项**：仍保留 config=7、eval-runner=7、prompt-processing=3、provider external-authority=16、script-bridge=7、current-target drift、external-authority、publication-authority 等 blockers；不承诺“无任何潜在 bug”。
- **下游 task 影响**：后续 current-latest burndown 可从 40 个总 blockers 继续推进；本 task 只证明本地 SQLite eval deletion lifecycle，不改变 remote/cloud `delete` command unsupported policy，也不证明 provider、script bridge、eval-runner rate-limit 或 publication parity。
