# Task 38.1: current-latest-0.121.14-target-refresh

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 38 - current-latest-0.121.14-target-refresh
**Dependencies**: task-24.1-current-latest-upstream-authority-lock, task-37.1-current-latest-unblock-packet-refresh

## 1. Background

On 2026-06-03, live upstream observation showed npm latest `promptfoo@0.121.14` with gitHead `7a48c5fce614bee617efbb3b7fc93d404c75b628`, GitHub default branch HEAD `4d22e57f5f9b4c7cdde494f00558d9afde8b4975`, and GitHub latest release `refs/tags/0.121.14` pointing to `7a48c5fce614bee617efbb3b7fc93d404c75b628`. The current tracked lock still records `0.121.13` / `1d09dfeb5f0766905409117f923dd5c4b0838d9f`. The shell lock also fails when npm tag ref and latest release ref are the same because one `ls-remote` row is consumed by an `else if` branch. 依据 PRD §Current Latest Rebaseline Addendum、ADR-007、ADR-009、ADR-011、task 24.1、task 37.1，以及 live observations captured by `npm view`, GitHub latest release API, and `git ls-remote`.

## 2. Goal

Refresh the current-latest target lock to the observed `promptfoo@0.121.14` packet, fix same-ref npm/latest-release accounting in both Rust and shell paths, and keep downstream perfect-refactor gates blocked until all refreshed-target evidence and external decisions exist.

## 3. Scope

### In Scope

- `scripts/release/current-latest-target-lock.sh`
- `scripts/release/runtime-smoke.sh`
- `src/compatibility/inventory.rs`
- `compatibility/inventory/current-latest-target.json`
- `docs/compatibility/current-latest.lock.md`
- `tests/current_latest_target_drift_refresh.rs`
- `docs/compatibility/matrix.md`
- `docs/prds/promptfoo-rs.prd.md`
- `docs/s2v-adapter.md`
- `test/features/perfect-refactor-parity.feature`

### Out Of Scope

- 不重新实现所有 `0.121.14` 新增功能 parity。
- 不解除 config/provider external-authority blockers。
- 不发布 Cargo/npm/Docker/Homebrew/GitHub artifacts。
- 不提供真实 provider credentials、账号权限、private service、法律/品牌或产品授权。
- 不把 current-latest target refresh 解释为 perfect-refactor 完成。

## 4. Users / Actors

- Maintainer: needs current-latest evidence to follow the live upstream package instead of stale `0.121.13`.
- Compatibility reviewer: needs same-ref npm/latest-release parsing to be explicit and tested.
- Future implementation agent: needs downstream artifacts to know whether remaining blockers are target drift or external authority.

## 5. Behavior Contract

`CurrentLatestTargetLock::from_observations` and `current-latest-target-lock.sh` must treat each `ls-remote` row as potentially satisfying multiple refs. If `npm_tag_ref` and `latest_release_ref` are identical, the same SHA must populate both `npm_tag_commit` and `latest_release_commit`. The refreshed tracked lock must record npm `0.121.14`, gitHead `7a48c5fce614bee617efbb3b7fc93d404c75b628`, tarball and integrity from npm, GitHub HEAD `4d22e57f5f9b4c7cdde494f00558d9afde8b4975`, and latest release `refs/tags/0.121.14` / `7a48c5fce614bee617efbb3b7fc93d404c75b628`. `current_latest_claim_allowed` remains `false`; any downstream packet must fail closed if refreshed source inventory, matrix, golden corpus, quality, external authority, and publication evidence are not all present for the same target.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/compatibility/current-latest.lock.md
- docs/compatibility/matrix.md
- docs/specs/tasks/task-24.1-current-latest-upstream-authority-lock.md
- docs/specs/tasks/task-37.1-current-latest-unblock-packet-refresh.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust: `promptfoo_rs::compatibility::inventory::CurrentLatestTargetLock`, `serde_json::Value`, `std::fs`, `std::path::Path`, `std::process::Command`.
- Shell/Node: `npm view promptfoo version gitHead dist.tarball dist.integrity time.modified --json`, GitHub latest release API, `git ls-remote https://github.com/promptfoo/promptfoo.git HEAD refs/tags/0.121.14`, `CURRENT_LATEST_WRITE_TRACKED=1 bash scripts/release/current-latest-target-lock.sh`, adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke.

### 5.3 函数签名

- `CurrentLatestTargetLock::from_observations(npm_json: &str, latest_release_json: &str, ls_remote: &str) -> Result<CurrentLatestTargetLock, CurrentLatestTargetError>`
- Shell contract: `CURRENT_LATEST_NPM_VIEW_FILE=<path> CURRENT_LATEST_GITHUB_RELEASE_FILE=<path> CURRENT_LATEST_LS_REMOTE_FILE=<path> CURRENT_LATEST_WRITE_TRACKED=1 bash scripts/release/current-latest-target-lock.sh`
- Test helper contract: `assert_current_latest_target_packet(path, expected_version, expected_git_head, expected_github_head) -> ()`

## 6. Acceptance Criteria

- [x] **AC1** (ADR-011): Rust target-lock parsing accepts same-ref npm/latest-release evidence and records both `npm_tag_commit` and `latest_release_commit` for `refs/tags/0.121.14`.
- [x] **AC2** (PRD §Current Latest Rebaseline Addendum): tracked current-latest lock artifacts record `promptfoo@0.121.14`, npm gitHead `7a48c5fce614bee617efbb3b7fc93d404c75b628`, GitHub HEAD `4d22e57f5f9b4c7cdde494f00558d9afde8b4975`, latest release `refs/tags/0.121.14`, tarball, integrity, source commands, and observation timestamp.
- [x] **AC3** (ADR-007 / ADR-009): runtime smoke and downstream release-gate artifacts remain fail-closed with `current_latest_claim_allowed=false` and `perfect_refactor_claim_allowed=false` after target refresh.
- [x] **AC4** (task 37.1): perfect-refactor unblock packet continues to expose current-target, external-authority, and publication decisions instead of using stale `0.121.13` target evidence as the current-latest authority.
- [x] **AC5** (PRD §Current Latest Rebaseline Addendum): runtime smoke prefers the tracked current-latest lock over a stale ignored `target/release-gates/current-latest-target.json` copy when preparing deterministic distribution fixtures.
- [x] **AC6** (ADR-007): runtime smoke distribution fixtures include the frozen baseline `refs/tags/0.121.13` ref even when current-latest npm/release refs have moved to `0.121.14`.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-38.1.1 | TEST-38.1.1 | tests/current_latest_target_drift_refresh.rs | install, typecheck, unit-test, build | Done |
| AC2 | SCEN-38.1.1 | TEST-38.1.2 | tests/current_latest_target_drift_refresh.rs | install, lint, typecheck, unit-test, integration, build | Done |
| AC3 | SCEN-38.1.1 | TEST-38.1.3 | tests/current_latest_target_drift_refresh.rs | install, typecheck, unit-test, e2e, runtime-smoke, build | Done |
| AC4 | SCEN-38.1.1 | TEST-38.1.4 | tests/current_latest_target_drift_refresh.rs | install, lint, typecheck, unit-test, coverage, runtime-smoke, build | Done |
| AC5 | SCEN-38.1.1 | TEST-38.1.5 | tests/current_latest_target_drift_refresh.rs | install, lint, typecheck, unit-test, runtime-smoke, build | Done |
| AC6 | SCEN-38.1.1 | TEST-38.1.6 | tests/current_latest_target_drift_refresh.rs | install, lint, typecheck, unit-test, runtime-smoke, build | Done |

## 8. Risks

- Upstream may move again while this task is running; this task locks the 2026-06-03 observed packet and keeps future drift visible.
- Runtime smoke can be slow because it rebuilds release-gate artifacts; failures must be debugged instead of marked as passing.
- Updating only the target lock is not enough for a perfect-refactor claim; downstream source inventory, golden corpus, quality, external authority, and publication gates still control completion.

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
  - `docs/prds/promptfoo-rs.prd.md`
  - `docs/s2v-adapter.md`
  - `docs/specs/phases/phase-38-current-latest-0.121.14-target-refresh.md`
  - `docs/specs/tasks/task-38.1-current-latest-0.121.14-target-refresh.md`
  - `test/features/perfect-refactor-parity.feature`
  - `tests/current_latest_target_drift_refresh.rs`
  - `src/compatibility/inventory.rs`
  - `scripts/release/current-latest-target-lock.sh`
  - `scripts/release/runtime-smoke.sh`
  - `compatibility/inventory/current-latest-target.json`
  - `docs/compatibility/current-latest.lock.md`
  - `docs/compatibility/matrix.md`
- **commit 列表**：
  - `4664f79 docs(spec): add current latest 0.121.14 target refresh task`
  - `40acab2 docs(spec): task 38.1 enters implementation`
  - `7a5be83 test(compatibility): add current latest 0.121.14 drift RED tests`
  - `1e32da4 feat(compatibility): refresh current latest target lock for 0.121.14`
  - `e8881cb test(compatibility): add current latest runtime smoke stale lock RED test`
  - `09db4f7 fix(release): prefer tracked current latest lock in runtime smoke`
  - `6da192b test(release): add frozen baseline ref RED coverage for runtime smoke`
  - `396864c fix(release): include frozen baseline ref in runtime smoke fixtures`
  - `<this docs commit> docs(spec): complete task 38.1 current latest target refresh`
- **§9 Verification 结果**：
  - install: PASS - adapter §Commands Install passed inside `s2v_verify_full`.
  - lint: PASS - adapter §Commands Lint passed inside `s2v_verify_full`.
  - typecheck: PASS - adapter §Commands Typecheck passed inside `s2v_verify_full`.
  - unit-test: PASS - adapter §Commands Unit Test passed; `TEST-38.1.1` through `TEST-38.1.6` pass in `tests/current_latest_target_drift_refresh.rs`.
  - integration: PASS - adapter §Commands Integration tests passed inside `s2v_verify_full`.
  - e2e: PASS - adapter §Commands E2E tests passed inside `s2v_verify_full`.
  - coverage: PASS - adapter §Commands Coverage passed inside `s2v_verify_full`; `s2v_coverage_threshold_guard` returned PASS.
  - build: PASS - adapter §Commands Build passed inside `s2v_verify_full`.
  - runtime-smoke: PASS - adapter §Commands Runtime smoke regenerated release gates with `current-latest-target.status=locked-with-drift`, npm `0.121.14`, npm gitHead `7a48c5fce614bee617efbb3b7fc93d404c75b628`, GitHub HEAD `4d22e57f5f9b4c7cdde494f00558d9afde8b4975`, `current_latest_claim_allowed=false`, current-latest golden `blocker_count=25`, quality `blocker_count=6`, release-candidate `publication_ready=credential-blocked`, and unblock packet `status=blocked`.
- **剩余风险 / 未做项**：
  - current-latest target 已刷新到 2026-06-03 观测的 `0.121.14` packet，但 `perfect_refactor_claim_allowed=false` 仍正确：source inventory / matrix 仍有 `unclassified_rows=1`，current-latest golden corpus 仍有 `blocker_count=25`，quality gate 仍有 `blocker_count=6`。
  - 真实 provider credentials、私有服务账号、法律/品牌授权、发布凭据，以及“无任何潜在 bug / bug-free”一类不可证明声明仍需外部证据或正式 waiver，不能由本地实现消除。
  - 一次 live GitHub API 调用返回 403；tracked lock 使用已记录的 npm / GitHub release / `git ls-remote` 观测 fixture 写入，runtime smoke 已改为优先使用 tracked lock，避免 stale ignored gate copy。
- **下游 task 影响**：
  - 后续 current-latest 证据必须以 `0.121.14` tracked lock 为目标，不得继续把 `0.121.13` 当作当前最新。
  - runtime smoke fixture 生成现在同时保留 frozen baseline `refs/tags/0.121.13` 与 current-latest `refs/tags/0.121.14`，避免 frozen gate 与 current-latest gate 互相破坏。
  - 若继续推进，下一步只能针对 current-latest source inventory / matrix unclassified row、golden blockers、quality blockers 或外部 authority/waiver 创建新 task。
