# Task 24.1: current-latest-upstream-authority-lock

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 24 — current-latest-perfect-refactor
**Dependencies**: task-18.3-current-upstream-rebaseline-gate, task-21.1-upstream-distribution-target-gate, task-23.1-dynamic-github-latest-release-observation

## 1. Background

The user clarified on 2026-06-01 that the intended perfect refactor target is the original promptfoo project's current latest complete functionality. Fresh observations show npm latest stable package `promptfoo@0.121.13` / `4860e990c7e9a2f8f677173fb92cf9867b34d03f`, GitHub default branch HEAD `1d09dfeb5f0766905409117f923dd5c4b0838d9f`, and GitHub latest release `code-scan-action-0.1.7` / `1c743afe0e4807882e858c4f322fc064fa5f0770`. Task 24.1 must turn that target into immutable evidence before implementation resumes. 依据用户 2026-06-01 澄清、PRD §Upstream Baseline Freeze Strategy / §Compatibility Harness Design、ADR-007、ADR-009、ADR-011。

## 2. Goal

Add a current-latest target lock artifact that records npm latest, GitHub HEAD, and GitHub latest release observations, rejects floating references as proof, and turns the previous current-upstream blocker into a concrete rebaseline input for downstream tasks.

## 3. Scope

### In Scope

- `docs/compatibility/current-latest.lock.md`
- `compatibility/inventory/current-latest-target.json`
- `scripts/release/current-latest-target-lock.sh`
- `scripts/release/runtime-smoke.sh`
- `src/compatibility/inventory.rs`
- `tests/current_latest_target_lock.rs`
- `docs/compatibility/target-policy.md`
- `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`
- `target/release-gates/current-latest-target.json`
- `test/features/perfect-refactor-parity.feature`

### Out Of Scope

- 不实现全部 current-latest 功能 parity。
- 不把 current-latest target lock 单独解释为 perfect-refactor 完成。
- 不伪造 provider credentials、publication evidence 或 legal/brand approval。

## 4. Users / Actors

- Release maintainer: needs one immutable target packet for current-latest rebaseline.
- Compatibility reviewer: needs npm, GitHub HEAD, and GitHub release channel evidence separated.
- Future implementation agent: needs downstream tasks to consume a stable target instead of floating latest.

## 5. Behavior Contract

The new lock command must fetch or read fixture inputs for npm latest metadata, GitHub default branch HEAD, and GitHub latest release metadata. It must write a JSON lock and Markdown lock with full SHAs, URLs, timestamps, commands, and channel classification. It must fail closed when any required SHA/ref/integrity is missing, and it must reject raw floating values as release claim evidence. Runtime smoke must expose this target packet without changing `perfect_refactor_claim_allowed` until downstream tasks produce full evidence.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/compatibility/target-policy.md
- docs/specs/tasks/task-18.3-current-upstream-rebaseline-gate.md
- docs/specs/tasks/task-21.1-upstream-distribution-target-gate.md
- docs/specs/tasks/task-23.1-dynamic-github-latest-release-observation.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- BLOCKED-task-22.1-perfect-refactor-external-authority.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `serde::{Serialize, Deserialize}`, `serde_json::Value`, `std::fs`, `std::path::Path`, `std::process::Command`, `promptfoo_rs::compatibility::inventory::{CurrentLatestTargetLock, CurrentLatestTargetError}`.
- Tooling commands: `npm view promptfoo version gitHead dist.tarball dist.integrity time.modified --json`, GitHub latest release metadata fetch, `git ls-remote https://github.com/promptfoo/promptfoo.git HEAD refs/tags/<npm-version> refs/tags/<latest-release>`, adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke.

### 5.3 函数签名

- `CurrentLatestTargetLock::from_observations(npm_json: &str, latest_release_json: &str, ls_remote: &str) -> Result<CurrentLatestTargetLock, CurrentLatestTargetError>`
- `write_current_latest_target_lock(lock: &CurrentLatestTargetLock, json_path: &Path, md_path: &Path) -> Result<(), CurrentLatestTargetError>`
- Shell contract: `CURRENT_LATEST_NPM_VIEW_FILE=<path> CURRENT_LATEST_GITHUB_RELEASE_FILE=<path> CURRENT_LATEST_LS_REMOTE_FILE=<path> bash scripts/release/current-latest-target-lock.sh`

## 6. Acceptance Criteria

- [x] **AC1** (ADR-011): JSON and Markdown lock artifacts record npm latest package, GitHub default branch HEAD, and GitHub latest release channel with full SHAs and acquisition commands.
- [x] **AC2** (ADR-007): raw floating refs such as `latest`, `main`, `master`, and `HEAD` are never accepted as proof of current-latest completion without the observed full SHA packet.
- [x] **AC3** (ADR-009): non-core GitHub release channel remains classified separately from npm latest stable package and cannot set `perfect_refactor_claim_allowed=true`.
- [x] **AC4** (task-22.1): unblock packet decision `current-upstream:rebaseline` is replaced or narrowed to downstream current-latest inventory/golden/publication/external evidence requirements, not left as an ambiguous target-selection blocker.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-24.1.1 | TEST-24.1.1 | tests/current_latest_target_lock.rs | install, typecheck, unit-test, integration, build | Done |
| AC2 | SCEN-24.1.1 | TEST-24.1.2 | tests/current_latest_target_lock.rs | install, lint, typecheck, unit-test, coverage, build | Done |
| AC3 | SCEN-24.1.1 | TEST-24.1.3 | tests/current_latest_target_lock.rs | install, typecheck, unit-test, e2e, runtime-smoke, build | Done |
| AC4 | SCEN-24.1.1 | TEST-24.1.4 | tests/current_latest_target_lock.rs | install, lint, typecheck, unit-test, runtime-smoke, build | Done |

## 8. Risks

- Upstream can move between npm, GitHub release, and git ls-remote calls; the lock must record exactly what was observed and fail on missing refs.
- This task can reduce target-selection ambiguity, but it does not solve external credentials or complete parity.
- GitHub API/network failures must be explicit errors, not silent stale evidence.

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

- **完成日期**：2026-06-01
- **改动文件**：
  - `docs/prds/promptfoo-rs.prd.md`
  - `docs/s2v-adapter.md`
  - `docs/decisions/adr-011-current-latest-full-refactor-target.md`
  - `docs/specs/phases/phase-24-current-latest-perfect-refactor.md`
  - `docs/specs/tasks/task-24.1-current-latest-upstream-authority-lock.md`
  - `docs/specs/tasks/task-24.2-current-latest-source-inventory-reextract.md`
  - `docs/specs/tasks/task-24.3-current-latest-full-function-golden-corpus.md`
  - `docs/specs/tasks/task-24.4-current-latest-exhaustive-quality-gate.md`
  - `docs/superpowers/plans/2026-06-01-current-latest-perfect-refactor.md`
  - `test/features/perfect-refactor-parity.feature`
  - `tests/current_latest_target_lock.rs`
  - `src/compatibility/inventory.rs`
  - `scripts/release/current-latest-target-lock.sh`
  - `scripts/release/runtime-smoke.sh`
  - `compatibility/inventory/current-latest-target.json`
  - `docs/compatibility/current-latest.lock.md`
- **commit 列表**：
  - `8358620` `docs(spec): add current latest perfect refactor phase`
  - `d7252bc` `test(compatibility): add current latest target lock RED tests`
  - `db079cb` `feat(compatibility): lock current latest upstream target`
  - 本次 docs 回填提交：`docs(spec): complete task 24.1 current latest target lock`
- **§9 Verification 结果**：
  - install: PASS — `s2v_verify_full "$(s2v_extract_verify_keys "$TASK_SPEC")"` 执行 adapter Install 通过。
  - lint: PASS — adapter Lint 通过。
  - typecheck: PASS — `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS — `cargo test --workspace` included `tests/current_latest_target_lock.rs` with TEST-24.1.1 ~ TEST-24.1.4 passing; viewer/npm tests passed.
  - integration: PASS — adapter Integration tests 通过。
  - e2e: PASS — adapter E2E tests 通过。
  - build: PASS — adapter Build 通过。
  - coverage: PASS — adapter Coverage gate 通过；`s2v_coverage_threshold_guard` passed。
  - runtime-smoke: PASS — runtime smoke now runs `current-latest-target-lock.sh`, validates `target/release-gates/current-latest-target.json`, and includes `current_latest_target` in `release-candidate.json`.
- **剩余风险 / 未做项**：Task 24.1 only resolves target-selection ambiguity by locking npm latest, GitHub HEAD, and GitHub latest release evidence. It intentionally keeps `current_latest_claim_allowed=false` until task 24.2 source inventory, task 24.3 golden corpus, task 24.4 quality gates, and external/publication authority evidence are complete or formally waived. First full verification attempt exceeded the 5-minute tool timeout during real-upstream corpus; lingering child processes were cleaned and the same §9 verification was rerun with a longer timeout and passed.
- **下游 task 影响**：task 24.2 can now consume `compatibility/inventory/current-latest-target.json` and `target/release-gates/current-latest-target.json` as the immutable current-latest target packet. task 24.3 and 24.4 must preserve the distinction between target lock readiness and perfect-refactor completion.
