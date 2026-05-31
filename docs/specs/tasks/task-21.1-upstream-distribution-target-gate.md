# Task 21.1: upstream-distribution-target-gate

**Status**: Ready
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 21 — upstream-distribution-target-disambiguation
**Dependencies**: task-18.3-current-upstream-rebaseline-gate, task-20.2-perfect-refactor-claim-contract

## 1. Background

task 18.3 记录 GitHub repository HEAD 与 frozen baseline 不同，并观察到 `code-scan-action: 0.1.7` release；task 20.2 因 current/source/publication/external blockers 禁止 perfect-refactor claim。继续推进时需要把 upstream target 事实源拆清：npm `promptfoo` core package 的 latest metadata、GitHub repo HEAD、GitHub latest release tag 可能不同。依据 PRD §Upstream Baseline Freeze Strategy / §Compatibility Harness Design、ADR-007、ADR-009、task-18.3 §10、task-20.2 §10。

## 2. Goal

新增 upstream distribution target gate：生成 `target/release-gates/upstream-distribution-target.json`，记录 npm core package、GitHub repository HEAD、observed latest release/tag 和 frozen baseline 的关系；若 npm core package 仍等于 frozen baseline，则明确 frozen evidence 对 published core package 仍有效；若 GitHub repo HEAD 或 non-core release 漂移，则保持 current repository perfect-refactor claim blocked。

## 3. Scope

### In Scope

- `src/compatibility/inventory.rs`
- `scripts/release/upstream-distribution-target.sh`
- `scripts/release/runtime-smoke.sh`
- `target/release-gates/upstream-distribution-target.json`
- `tests/upstream_distribution_target_gate.rs`
- `docs/compatibility/target-policy.md`
- `docs/compatibility/matrix.md`
- `docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`
- `test/features/perfect-refactor-parity.feature`

### Out Of Scope

- 不把 GitHub HEAD 自动设为 compatibility target。
- 不公开发布任何 artifact。
- 不解除 external-authority、publication-authority 或 source accounting blockers。
- 不把 non-core release tag 当作 npm core package 最新版本。

## 4. Users / Actors

- Release maintainer：需要判断 frozen evidence 是否仍覆盖最新 npm core package。
- Compatibility reviewer：需要看到 GitHub repo/release drift 是 core package drift 还是 release-channel drift。
- Future rebaseline implementer：需要知道 current-mode rebaseline 应以哪个 upstream ref/package metadata 为输入。

## 5. Behavior Contract

Distribution target gate 必须从 npm package metadata 和 GitHub ref/release metadata 构建 fail-closed artifact。artifact 必须记录 full SHA、npm integrity、source command、observed_at 和分类字段。`npm_core_matches_frozen_baseline=true` 只能说明 frozen-baseline evidence 覆盖 latest npm core package；只要 repository HEAD 不等于 npm core gitHead，或 GitHub latest release 不是 core package release，`current_repository_perfect_claim_allowed` 必须为 false。该 artifact 不得把 current-upstream policy、publication authority 或 external authority blocker 变为 ready。

### 5.1 Required Reading

- docs/specs/tasks/task-18.3-current-upstream-rebaseline-gate.md
- docs/specs/tasks/task-20.2-perfect-refactor-claim-contract.md
- docs/compatibility/target-policy.md
- docs/compatibility/baseline.lock.md
- docs/compatibility/matrix.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`promptfoo_rs::compatibility::inventory`、`serde_json`、`std::fs`、`std::path::Path`、`std::process::Command`。
- Tooling commands：`npm view promptfoo version gitHead dist.tarball dist.integrity time.modified --json`、`git ls-remote https://github.com/promptfoo/promptfoo.git HEAD refs/tags/0.121.13 refs/tags/code-scan-action-0.1.7`、adapter §Commands Install / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke。

### 5.3 函数签名

- `parse_npm_package_observation(json: &str) -> Result<NpmPackageObservation, DistributionTargetError>`
- `build_upstream_distribution_target(npm: NpmPackageObservation, github: CurrentUpstreamObservation, frozen: FrozenSourceReference) -> UpstreamDistributionTarget`
- `write_upstream_distribution_target(target: &UpstreamDistributionTarget, path: &Path) -> Result<(), DistributionTargetError>`

## 6. Acceptance Criteria

- [ ] **AC1** (PRD §Upstream Baseline Freeze Strategy): artifact records npm package version, gitHead, tarball, integrity, modified time and frozen baseline with full SHA/integrity.
- [ ] **AC2** (task-18.3): artifact records GitHub repository HEAD and observed latest release ref/commit separately from npm package metadata.
- [ ] **AC3** (ADR-007): non-core GitHub release tags, including `code-scan-action:*`, cannot set `github_latest_release_is_core_package=true` or allow current repository perfect-refactor claim.
- [ ] **AC4** (task-20.2): release candidate and docs/audit consume the artifact without weakening `perfect_refactor_claim_allowed=false` while source/current/publication/external blockers remain.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-21.1.1 | TEST-21.1.1 | tests/upstream_distribution_target_gate.rs | install, typecheck, unit-test, integration, build | Not Started |
| AC2 | SCEN-21.1.1 | TEST-21.1.2 | tests/upstream_distribution_target_gate.rs | install, typecheck, unit-test, integration, build | Not Started |
| AC3 | SCEN-21.1.1 | TEST-21.1.3 | tests/upstream_distribution_target_gate.rs | install, typecheck, unit-test, coverage, build | Not Started |
| AC4 | SCEN-21.1.1 | TEST-21.1.4 | tests/upstream_distribution_target_gate.rs | install, typecheck, unit-test, e2e, runtime-smoke, build | Not Started |

## 8. Risks

- GitHub latest release may refer to a package/channel other than npm `promptfoo`; the classifier must preserve that distinction。
- npm metadata can change; runtime smoke must record observed metadata, and tests must rely on fixtures。
- A green distribution target artifact is not a perfect-refactor claim; external/current/publication blockers remain authoritative。

## 9. Verification Plan

- **Install**: adapter §Commands Install
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
