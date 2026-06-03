# Task 38.1: current-latest-0.121.14-target-refresh

**Status**: In Progress
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

- [ ] **AC1** (ADR-011): Rust target-lock parsing accepts same-ref npm/latest-release evidence and records both `npm_tag_commit` and `latest_release_commit` for `refs/tags/0.121.14`.
- [ ] **AC2** (PRD §Current Latest Rebaseline Addendum): tracked current-latest lock artifacts record `promptfoo@0.121.14`, npm gitHead `7a48c5fce614bee617efbb3b7fc93d404c75b628`, GitHub HEAD `4d22e57f5f9b4c7cdde494f00558d9afde8b4975`, latest release `refs/tags/0.121.14`, tarball, integrity, source commands, and observation timestamp.
- [ ] **AC3** (ADR-007 / ADR-009): runtime smoke and downstream release-gate artifacts remain fail-closed with `current_latest_claim_allowed=false` and `perfect_refactor_claim_allowed=false` after target refresh.
- [ ] **AC4** (task 37.1): perfect-refactor unblock packet continues to expose current-target, external-authority, and publication decisions instead of using stale `0.121.13` target evidence as the current-latest authority.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-38.1.1 | TEST-38.1.1 | tests/current_latest_target_drift_refresh.rs | install, typecheck, unit-test, build | Spec Ready |
| AC2 | SCEN-38.1.1 | TEST-38.1.2 | tests/current_latest_target_drift_refresh.rs | install, lint, typecheck, unit-test, integration, build | Spec Ready |
| AC3 | SCEN-38.1.1 | TEST-38.1.3 | tests/current_latest_target_drift_refresh.rs | install, typecheck, unit-test, e2e, runtime-smoke, build | Spec Ready |
| AC4 | SCEN-38.1.1 | TEST-38.1.4 | tests/current_latest_target_drift_refresh.rs | install, lint, typecheck, unit-test, coverage, runtime-smoke, build | Spec Ready |

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

- **完成日期**：<TBD-after-impl>
- **改动文件**：<TBD-after-impl>
- **commit 列表**：<TBD-after-impl>
- **§9 Verification 结果**：<TBD-after-impl>
- **剩余风险 / 未做项**：<TBD-after-impl>
- **下游 task 影响**：<TBD-after-impl>
