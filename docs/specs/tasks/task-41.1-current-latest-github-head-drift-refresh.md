# Task 41.1: current-latest-github-head-drift-refresh

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 41 - current-latest-github-head-drift-refresh
**Dependencies**: task-38.1-current-latest-0.121.14-target-refresh, task-40.1-current-latest-evaluator-runtime-fixture-burndown

## 1. Background

On 2026-06-03, live upstream observation showed npm latest remains `promptfoo@0.121.14` with gitHead `7a48c5fce614bee617efbb3b7fc93d404c75b628`, and GitHub latest release remains `refs/tags/0.121.14` pointing to the same commit. However, `git ls-remote https://github.com/promptfoo/promptfoo.git HEAD refs/tags/0.121.14 refs/tags/0.121.13` showed default branch HEAD moved to `9d7d810c2118c63cb537bf05ea2d34c12bd22066`; the tracked lock still records `4d22e57f5f9b4c7cdde494f00558d9afde8b4975`. 依据 PRD §Current Latest Rebaseline Addendum、ADR-007、ADR-009、ADR-011、task 38.1、task 40.1。

## 2. Goal

Refresh the current-latest target lock and downstream release-gate evidence to the observed GitHub default branch HEAD `9d7d810c2118c63cb537bf05ea2d34c12bd22066` while keeping npm latest/release evidence on `0.121.14` and preserving fail-closed perfect-refactor gates.

## 3. Scope

### In Scope

- `tests/current_latest_github_head_drift_refresh.rs`
- `tests/current_latest_target_drift_refresh.rs`
- `compatibility/inventory/current-latest-target.json`
- `docs/compatibility/current-latest.lock.md`
- `docs/compatibility/matrix.md`
- `docs/prds/promptfoo-rs.prd.md`
- `docs/s2v-adapter.md`
- `test/features/perfect-refactor-parity.feature`
- `docs/specs/phases/phase-41-current-latest-github-head-drift-refresh.md`
- `docs/specs/tasks/task-41.1-current-latest-github-head-drift-refresh.md`

### Out Of Scope

- 不升级 npm latest version；当前 npm latest 仍是 `0.121.14`。
- 不解除 provider/config external-authority blockers。
- 不发布 Cargo/npm/Docker/Homebrew/GitHub artifacts。
- 不提供真实 provider credentials、账号权限、private service、法律/品牌或产品授权。
- 不把 GitHub HEAD refresh 解释为 perfect-refactor 完成。

## 4. Users / Actors

- Maintainer: needs current-latest evidence to follow live upstream repository HEAD drift even without a new npm package publish.
- Compatibility reviewer: needs source inventory and golden gates to target the same refreshed default branch commit.
- Future implementation agent: needs the target lock to distinguish local code blockers from external decisions.

## 5. Behavior Contract

The current-latest target lock must record npm latest `0.121.14` / gitHead `7a48c5fce614bee617efbb3b7fc93d404c75b628`, GitHub latest release `refs/tags/0.121.14` / `7a48c5fce614bee617efbb3b7fc93d404c75b628`, and GitHub default branch HEAD `9d7d810c2118c63cb537bf05ea2d34c12bd22066`. `current_latest_claim_allowed` remains `false` because npm latest gitHead and repository HEAD differ and downstream source/matrix/golden/quality/external/publication evidence must agree before any perfect-refactor claim. Runtime smoke must regenerate current-latest artifacts from the tracked lock and keep downstream claims fail-closed.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/compatibility/current-latest.lock.md
- docs/compatibility/matrix.md
- docs/specs/tasks/task-38.1-current-latest-0.121.14-target-refresh.md
- docs/specs/tasks/task-40.1-current-latest-evaluator-runtime-fixture-burndown.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `promptfoo_rs::compatibility::inventory::CurrentLatestTargetLock`, `serde_json::Value`, `std::fs`, `std::path::Path`, `std::process::Command`.
- Shell/tooling commands: `npm view promptfoo version gitHead dist.tarball dist.integrity time.modified --json`, GitHub latest release API, `git ls-remote https://github.com/promptfoo/promptfoo.git HEAD refs/tags/0.121.14 refs/tags/0.121.13`, `CURRENT_LATEST_WRITE_TRACKED=1 bash scripts/release/current-latest-target-lock.sh`, adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke.

### 5.3 函数签名

- `CurrentLatestTargetLock::from_observations(npm_json: &str, latest_release_json: &str, ls_remote: &str) -> Result<CurrentLatestTargetLock, CurrentLatestTargetError>`
- Shell contract: `CURRENT_LATEST_NPM_VIEW_FILE=<path> CURRENT_LATEST_GITHUB_RELEASE_FILE=<path> CURRENT_LATEST_LS_REMOTE_FILE=<path> CURRENT_LATEST_WRITE_TRACKED=1 bash scripts/release/current-latest-target-lock.sh`
- Test helper contract: `assert_tracked_current_latest_head(path, expected_github_head) -> ()`

## 6. Acceptance Criteria

- [x] **AC1** (ADR-011): Rust target-lock parsing records GitHub default branch HEAD `9d7d810c2118c63cb537bf05ea2d34c12bd22066` while npm latest and GitHub latest release remain `0.121.14`.
- [x] **AC2** (PRD §Current Latest Rebaseline Addendum): tracked current-latest lock artifacts record the refreshed GitHub HEAD and retain npm tarball, integrity, source commands, and observation timestamp.
- [x] **AC3** (ADR-007 / ADR-009): runtime smoke regenerates downstream current-latest artifacts for the refreshed HEAD and leaves `current_latest_claim_allowed=false` / `perfect_refactor_claim_allowed=false`.
- [x] **AC4** (task 40.1): evaluator runtime native fixture evidence remains present after the refreshed HEAD re-extraction, or any new local blocker is surfaced explicitly by release-gate artifacts.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-41.1.1 | TEST-41.1.1 | tests/current_latest_github_head_drift_refresh.rs | install, lint, typecheck, unit-test, build | Done |
| AC2 | SCEN-41.1.1 | TEST-41.1.2 | tests/current_latest_github_head_drift_refresh.rs | install, lint, typecheck, unit-test, integration, build | Done |
| AC3 | SCEN-41.1.1 | TEST-41.1.3 | tests/current_latest_github_head_drift_refresh.rs | install, typecheck, unit-test, e2e, runtime-smoke, build | Done |
| AC4 | SCEN-41.1.1 | TEST-41.1.4 | tests/current_latest_github_head_drift_refresh.rs | install, lint, typecheck, unit-test, coverage, runtime-smoke, build | Done |

## 8. Risks

- GitHub HEAD may move again while this task is running; this task records the observed immutable `9d7d810...` packet and future drift must enter a new S2V task.
- Runtime smoke can reveal new current-latest source rows; those must be handled as real evidence, not deleted from matrices.
- Updating the target lock is not enough for a perfect-refactor claim; external authority, publication, and current-target policy still control completion.

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

- **完成日期**：2026-06-03
- **改动文件**：
  - `compatibility/inventory/current-latest-target.json`
  - `docs/compatibility/current-latest.lock.md`
  - `docs/compatibility/matrix.md`
  - `docs/prds/promptfoo-rs.prd.md`
  - `docs/s2v-adapter.md`
  - `docs/specs/phases/phase-41-current-latest-github-head-drift-refresh.md`
  - `docs/specs/tasks/task-41.1-current-latest-github-head-drift-refresh.md`
  - `test/features/perfect-refactor-parity.feature`
  - `tests/current_latest_github_head_drift_refresh.rs`
  - `tests/current_latest_target_drift_refresh.rs`
- **commit 列表**：
  - `7a2f032 docs(spec): add github head drift refresh task`
  - `df79624 test(compatibility): add github head drift refresh RED tests`
  - `7543f7d feat(compatibility): refresh current latest github head target lock`
  - `3543a24 refactor(compatibility): decouple prior drift test from moving head`
  - `<this docs commit> docs(spec): complete task 41.1 github head drift refresh`
- **§9 Verification 结果**：
  - install: PASS - adapter §Commands Install passed inside `s2v_verify_full`.
  - lint: PASS - adapter §Commands Lint passed inside `s2v_verify_full`.
  - typecheck: PASS - adapter §Commands Typecheck passed inside `s2v_verify_full`.
  - unit-test: PASS - adapter §Commands Unit Test passed; `TEST-41.1.1` through `TEST-41.1.4` pass in `tests/current_latest_github_head_drift_refresh.rs`, and historical `TEST-38.1.2` now validates the moving tracked GitHub HEAD through the same lock artifact.
  - integration: PASS - adapter §Commands Integration tests passed inside `s2v_verify_full`.
  - e2e: PASS - adapter §Commands E2E tests passed inside `s2v_verify_full`.
  - coverage: PASS - adapter §Commands Coverage passed inside `s2v_verify_full`; `s2v_coverage_threshold_guard` returned PASS.
  - build: PASS - adapter §Commands Build passed inside `s2v_verify_full`.
  - runtime-smoke: PASS - adapter §Commands Runtime smoke regenerated release gates with current-latest target `status=locked-with-drift`, `github.default_branch_head=9d7d810c2118c63cb537bf05ea2d34c12bd22066`, source inventory `status=ready`, matrix `status=ready`, both `unclassified_rows=[]`, current-latest golden `blocker_count=24`, current-latest quality `blocker_count=4`, `local_current_latest_ready=false`, release candidate `publication_ready=credential-blocked`, `perfect_refactor_claim_allowed=false`, and unblock packet `required_user_decision_count=31`.
- **剩余风险 / 未做项**：
  - GitHub default branch can drift again after this observed lock; future drift must enter a new S2V target-refresh task rather than silently changing this completion proof.
  - Remaining blockers are external config/provider authority, current-target drift policy, publication authority, and formal zero-bug / bug-free claim boundaries; this task does not and cannot waive them.
- **下游 task 影响**：
  - Downstream current-latest gates now consume `git:https://github.com/promptfoo/promptfoo.git#9d7d810c2118c63cb537bf05ea2d34c12bd22066`.
  - No new local unclassified rows were introduced; evaluator runtime fixture evidence remains native, so future work should focus on external authority/publication decisions or new upstream drift.
