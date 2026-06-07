# Task 48.1: current-latest-0.121.15-target-refresh

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 48 - current-latest-0.121.15-target-refresh
**Dependencies**: task-42.1-current-latest-2ca16c-head-refresh, task-47.1-current-latest-evaluator-inmemorystore-fixture-burndown

## 1. Background

On 2026-06-07, live upstream observation showed npm latest `promptfoo@0.121.15` with gitHead `4805856060d026521794d4e69decb938155580ad`, GitHub latest release `refs/tags/0.121.15` pointing to the same commit, and `git ls-remote https://github.com/promptfoo/promptfoo.git HEAD refs/tags/0.121.14 refs/tags/0.121.15` showing default branch HEAD `c54a30668ad8319d76c20ae96e6680ad6c51a2c6`. The tracked lock still records `0.121.14` / `2ca16c59b64e0afca10533de0f817c0d24eba20a`. 依据 PRD §Current Latest Rebaseline Addendum、ADR-007、ADR-009、ADR-011、task 42.1、task 47.1。

## 2. Goal

Refresh the current-latest target lock and downstream release-gate evidence to the observed `promptfoo@0.121.15` packet and GitHub default branch HEAD `c54a30668ad8319d76c20ae96e6680ad6c51a2c6`, preserving fail-closed perfect-refactor gates.

## 3. Scope

### In Scope

- `tests/current_latest_0_121_15_target_refresh.rs`
- `tests/current_latest_target_drift_refresh.rs`
- `tests/current_latest_github_head_drift_refresh.rs`
- `compatibility/inventory/current-latest-target.json`
- `docs/compatibility/current-latest.lock.md`
- `docs/compatibility/matrix.md`
- `docs/prds/promptfoo-rs.prd.md`
- `docs/s2v-adapter.md`
- `test/features/perfect-refactor-parity.feature`
- `docs/specs/phases/phase-48-current-latest-0.121.15-target-refresh.md`
- `docs/specs/tasks/task-48.1-current-latest-0.121.15-target-refresh.md`

### Out Of Scope

- 不解除 provider/config external-authority blockers。
- 不发布 Cargo/npm/Docker/Homebrew/GitHub artifacts。
- 不提供真实 provider credentials、账号权限、private service、法律/品牌或产品授权。
- 不把 target refresh 解释为 perfect-refactor 完成。

## 4. Users / Actors

- Maintainer: needs current-latest evidence to follow live npm latest and repository HEAD drift.
- Compatibility reviewer: needs source inventory and golden gates to target the same refreshed packet.
- Future implementation agent: needs the target lock to distinguish local code blockers from external decisions.

## 5. Behavior Contract

The current-latest target lock must record npm latest `0.121.15` / gitHead `4805856060d026521794d4e69decb938155580ad`, GitHub latest release `refs/tags/0.121.15` / `4805856060d026521794d4e69decb938155580ad`, and GitHub default branch HEAD `c54a30668ad8319d76c20ae96e6680ad6c51a2c6`. `current_latest_claim_allowed` remains `false` unless all downstream source/matrix/golden/quality/external/publication evidence agrees. Runtime smoke must regenerate current-latest artifacts from the tracked lock and keep downstream claims fail-closed.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/compatibility/current-latest.lock.md
- docs/compatibility/matrix.md
- docs/specs/tasks/task-42.1-current-latest-2ca16c-head-refresh.md
- docs/specs/tasks/task-47.1-current-latest-evaluator-inmemorystore-fixture-burndown.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `promptfoo_rs::compatibility::inventory::CurrentLatestTargetLock`, `serde_json::Value`, `std::fs`, `std::path::Path`, `std::process::Command`.
- Shell/tooling commands: `npm view promptfoo version gitHead dist.tarball dist.integrity time.modified --json`, GitHub latest release API, `git ls-remote https://github.com/promptfoo/promptfoo.git HEAD refs/tags/0.121.14 refs/tags/0.121.15`, `CURRENT_LATEST_WRITE_TRACKED=1 bash scripts/release/current-latest-target-lock.sh`, adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke.

### 5.3 函数签名

- `CurrentLatestTargetLock::from_observations(npm_json: &str, latest_release_json: &str, ls_remote: &str) -> Result<CurrentLatestTargetLock, CurrentLatestTargetError>`
- Shell contract: `CURRENT_LATEST_NPM_VIEW_FILE=<path> CURRENT_LATEST_GITHUB_RELEASE_FILE=<path> CURRENT_LATEST_LS_REMOTE_FILE=<path> CURRENT_LATEST_WRITE_TRACKED=1 bash scripts/release/current-latest-target-lock.sh`
- Test helper contract: `assert_tracked_current_latest_target(path: &Path, expected_version: &str, expected_github_head: &str) -> ()`

## 6. Acceptance Criteria

- [x] **AC1** (ADR-011): Rust target-lock parsing records npm `0.121.15` and GitHub default branch HEAD `c54a30668ad8319d76c20ae96e6680ad6c51a2c6`.
- [x] **AC2** (PRD §Current Latest Rebaseline Addendum): tracked current-latest lock artifacts record the refreshed npm tarball, integrity, GitHub HEAD, and observation timestamp.
- [x] **AC3** (ADR-007 / ADR-009): runtime smoke regenerates downstream current-latest artifacts for the refreshed target and leaves `current_latest_claim_allowed=false` / `perfect_refactor_claim_allowed=false` unless all gates agree.
- [x] **AC4** (task 47.1): evaluator in-memory store native fixture evidence and prior local current-latest fixture evidence survive re-extraction; no new unclassified rows were introduced.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-48.1.1 | TEST-48.1.1 | tests/current_latest_0_121_15_target_refresh.rs | install, lint, typecheck, unit-test, build | Done |
| AC2 | SCEN-48.1.1 | TEST-48.1.2 | tests/current_latest_0_121_15_target_refresh.rs | install, lint, typecheck, unit-test, integration, build | Done |
| AC3 | SCEN-48.1.1 | TEST-48.1.3 | tests/current_latest_0_121_15_target_refresh.rs | install, typecheck, unit-test, e2e, runtime-smoke, build | Done |
| AC4 | SCEN-48.1.1 | TEST-48.1.4 | tests/current_latest_0_121_15_target_refresh.rs | install, lint, typecheck, unit-test, coverage, runtime-smoke, build | Done |

## 8. Risks

- Upstream may move again while this task is running; this task records the observed immutable packet and future drift must enter a new S2V task.
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

- **完成日期**：2026-06-07
- **改动文件**：
  - `compatibility/inventory/current-latest-target.json`
  - `docs/compatibility/current-latest.lock.md`
  - `docs/compatibility/matrix.md`
  - `docs/compatibility/authority-decisions.json`
  - `docs/compatibility/v1-release-authority-policy.md`
  - `docs/prds/promptfoo-rs.prd.md`
  - `docs/s2v-adapter.md`
  - `test/features/perfect-refactor-parity.feature`
  - `tests/current_latest_0_121_15_target_refresh.rs`
  - `tests/current_latest_github_head_drift_refresh.rs`
  - `tests/current_latest_target_drift_refresh.rs`
  - `target/release-gates/current-latest-*.json` and regenerated gate artifacts
- **commit 列表**：
  - pending test(compatibility): add 0.121.15 target refresh RED tests
  - pending feat(compatibility): refresh current latest target lock to 0.121.15
  - pending docs(spec): complete task 48.1 + phase 48 Done
- **§9 Verification 结果**：
  - install: PASS - cargo fetch / build prerequisites satisfied
  - lint: PASS - cargo clippy on project sources
  - typecheck: PASS - cargo check --all-targets
  - unit-test: PASS - TEST-48.1.1 through TEST-48.1.4 pass in `tests/current_latest_0_121_15_target_refresh.rs`; historical Phase 38/42 tests still pass
  - integration: PASS - tracked lock + shell script integration via TEST-48.1.2 and TEST-48.1.3
  - e2e: PASS - runtime-smoke subprocess chain completed
  - coverage: PASS - cargo llvm-cov with project thresholds
  - build: PASS - cargo build --release
  - runtime-smoke: PASS - regenerated release gates with `status=locked-with-drift`, `github.default_branch_head=c54a30668ad8319d76c20ae96e6680ad6c51a2c6`, source inventory `status=ready`, matrix `status=ready`, `unclassified_rows=[]`, evaluator runtime and in-memory store fixture evidence native, current-latest golden `blocker_count=24`, quality `blocker_count=4`, release candidate `publication_ready=credential-blocked`, `perfect_refactor_claim_allowed=false`, unblock packet `required_user_decision_count=31`
- **剩余风险 / 未做项**：
  - GitHub default branch and npm latest can drift again after this observed lock; future drift must enter a new S2V target-refresh task.
  - Target refresh does not waive external authority, publication, or bug-free claim boundaries.
- **下游 task 影响**：
  - Downstream current-latest gates now consume `git:https://github.com/promptfoo/promptfoo.git#c54a30668ad8319d76c20ae96e6680ad6c51a2c6` with npm/release evidence `promptfoo@0.121.15`.
  - No new local taxonomy blockers were introduced by the refreshed target.