# Task 23.1: dynamic-github-latest-release-observation

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 23 — dynamic-upstream-release-observation
**Dependencies**: task-21.1-upstream-distribution-target-gate, task-22.1-authority-unblock-packet-gate

## 1. Background

Task 21.1 created `upstream-distribution-target.json`, but the shell gate still queries a fixed observed release ref (`refs/tags/code-scan-action-0.1.7`). On 2026-06-01 resumed audit, `npm view promptfoo` still reports `0.121.13` / `4860e990c7e9a2f8f677173fb92cf9867b34d03f`, while `git ls-remote ... HEAD` reports `0b93733d48727be67e34433cb0fb1ad21026863a`. The gate must keep current perfect-refactor blocked, but its latest-release evidence should be dynamically observed. 依据 PRD §Compatibility Matrix / §Success Metrics、ADR-007、ADR-009、task-21.1 §10、BLOCKED-task-22.1 Resume audit — 2026-06-01。

## 2. Goal

修改 upstream distribution target gate，使它从 GitHub latest release metadata 动态解析 release tag，并在 `github.source` / release candidate 中记录实际查询 ref；测试覆盖 fixture-driven latest release，确保不再依赖 hard-coded `code-scan-action-0.1.7`。

## 3. Scope

### In Scope

- `scripts/release/upstream-distribution-target.sh`
- `scripts/release/runtime-smoke.sh`
- `target/release-gates/upstream-distribution-target.json`
- `tests/upstream_distribution_target_gate.rs`
- `docs/compatibility/target-policy.md`
- `docs/compatibility/matrix.md`
- `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`
- `test/features/perfect-refactor-parity.feature`

### Out Of Scope

- 不把 GitHub latest release 自动设为 compatibility target。
- 不解除 `perfect_refactor_claim_allowed=false`。
- 不发布任何 artifact。
- 不提供 credentials、账号、法律/品牌或 external authority evidence。

## 4. Users / Actors

- Release maintainer：需要看到 latest release evidence 来自当前 GitHub metadata。
- Compatibility reviewer：需要确认 non-core release channel 仍不能解除 current-upstream blocker。
- Future rebaseline implementer：需要知道 dynamic latest release observation 与 same-ref rebaseline evidence 是两件事。

## 5. Behavior Contract

`upstream-distribution-target.sh` 必须先解析 latest release tag：测试中从 `UPSTREAM_GITHUB_RELEASE_FILE` 读取 fixture，真实运行中从公开 GitHub latest release metadata 读取。随后 `git ls-remote` 必须查询 HEAD、frozen tag 和解析出的 latest release tag。若 latest release metadata 缺失或 tag 无法解析，脚本必须失败，而不是回退到 hard-coded tag。`github_latest_release_is_core_package` 仍只能在 latest release channel 是 core package 且 release commit 等于 npm core gitHead 时为 true。

### 5.1 Required Reading

- docs/specs/tasks/task-21.1-upstream-distribution-target-gate.md
- docs/specs/tasks/task-22.1-authority-unblock-packet-gate.md
- docs/compatibility/target-policy.md
- docs/compatibility/matrix.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- BLOCKED-task-22.1-perfect-refactor-external-authority.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`serde_json::Value`、`std::fs`、`std::path::Path`、`std::process::Command`、`promptfoo_rs::compatibility::inventory::{CurrentUpstreamObservation, parse_npm_package_observation, build_upstream_distribution_target}`。
- Tooling commands：`npm view promptfoo version gitHead dist.tarball dist.integrity time.modified --json`、GitHub latest release metadata fetch、`git ls-remote https://github.com/promptfoo/promptfoo.git HEAD refs/tags/0.121.13 <dynamic-latest-ref>`、adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke。

### 5.3 函数签名

- Shell helper：`resolve_latest_release_tag <json-file>` 输出 GitHub latest release tag。
- Script contract：`UPSTREAM_GITHUB_RELEASE_FILE=<path> UPSTREAM_NPM_VIEW_FILE=<path> UPSTREAM_LS_REMOTE_FILE=<path> bash scripts/release/upstream-distribution-target.sh`
- Existing Rust contract：`CurrentUpstreamObservation::from_ls_remote(output: &str) -> Result<CurrentUpstreamObservation, TargetPolicyError>`

## 6. Acceptance Criteria

- [x] **AC1** (task-21.1): script output records the latest release ref from `UPSTREAM_GITHUB_RELEASE_FILE` fixture, not the hard-coded `refs/tags/code-scan-action-0.1.7`.
- [x] **AC2** (ADR-007): generated `github.source` includes the dynamic `refs/tags/<latest>` query and omits the old hard-coded release ref when a different latest release fixture is provided.
- [x] **AC3** (ADR-009): a dynamic non-core latest release keeps `github_latest_release_is_core_package=false` and `current_repository_perfect_claim_allowed=false`.
- [x] **AC4** (task-22.1): runtime smoke, docs, audit, and BLOCKED notes keep perfect-refactor external/current blockers visible after dynamic release observation.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-23.1.1 | TEST-23.1.1 | tests/upstream_distribution_target_gate.rs | install, typecheck, unit-test, integration, build | Done |
| AC2 | SCEN-23.1.1 | TEST-23.1.2 | tests/upstream_distribution_target_gate.rs | install, typecheck, unit-test, integration, build | Done |
| AC3 | SCEN-23.1.1 | TEST-23.1.3 | tests/upstream_distribution_target_gate.rs | install, typecheck, unit-test, coverage, build | Done |
| AC4 | SCEN-23.1.1 | TEST-23.1.4 | tests/upstream_distribution_target_gate.rs | install, lint, typecheck, unit-test, e2e, runtime-smoke, build | Done |

## 8. Risks

- Public GitHub API shape may differ between `gh release view` and REST JSON; parser must accept `tagName` and `tag_name`.
- Tests must use fixtures and must not depend on live GitHub availability.
- Dynamic release freshness does not imply current-upstream rebaseline completeness.

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
- **改动文件**：`docs/prds/promptfoo-rs.prd.md`、`docs/s2v-adapter.md`、`docs/specs/phases/phase-23-dynamic-upstream-release-observation.md`、`docs/specs/tasks/task-23.1-dynamic-github-latest-release-observation.md`、`docs/superpowers/plans/2026-06-01-dynamic-upstream-release-observation.md`、`test/features/perfect-refactor-parity.feature`、`tests/upstream_distribution_target_gate.rs`、`scripts/release/upstream-distribution-target.sh`、`docs/compatibility/target-policy.md`、`docs/compatibility/matrix.md`、`docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`、`BLOCKED-task-22.1-perfect-refactor-external-authority.md`
- **commit 列表**：
  - `c360e99 docs(spec): add phase 23 dynamic upstream release observation`
  - `a1f6809 test(compatibility): add SCEN-23.1.1 dynamic release observation RED test`
  - `97344b1 feat(compatibility): observe dynamic upstream latest release`
  - 本次 docs 回填提交：`docs(spec): complete task 23.1 dynamic release observation`
- **§9 Verification 结果**：
  - install: PASS — `s2v_verify_full "$(s2v_extract_verify_keys "$TASK_SPEC")"`
  - lint: PASS — `s2v_verify_full "$(s2v_extract_verify_keys "$TASK_SPEC")"`
  - typecheck: PASS — `s2v_verify_full "$(s2v_extract_verify_keys "$TASK_SPEC")"`
  - unit-test: PASS — `cargo test --workspace` included `tests/upstream_distribution_target_gate.rs` with 5/5 tests passing.
  - integration: PASS — adapter integration gate passed.
  - e2e: PASS — adapter e2e gate passed.
  - coverage: PASS — adapter coverage gate passed; `s2v_coverage_threshold_guard` passed.
  - build: PASS — adapter build gate passed.
  - runtime-smoke: PASS — runtime smoke regenerated release-gate artifacts.
- **剩余风险 / 未做项**：dynamic latest release observation only refreshes public GitHub release evidence. It does not resolve source accounting blockers, provider/product external authority blockers, publication credentials/legal-brand blockers, or same-ref current-upstream rebaseline evidence. `perfect_refactor_claim_allowed` remains false by design.
- **下游 task 影响**：future current-upstream rebaseline work can rely on `upstream-distribution-target.json.github.source` reflecting the actual latest release ref queried at smoke time, but it must still produce same-ref inventory, matrix, fixture, golden corpus, and release evidence before any current perfect-refactor claim.
