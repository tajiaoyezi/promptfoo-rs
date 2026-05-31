# Phase 21: upstream-distribution-target-disambiguation

**Status**: Done
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

把 current-upstream blocker 进一步拆清：npm `promptfoo` core package 最新发布、GitHub repository `HEAD`、GitHub latest release tag 和 frozen baseline 不是同一事实源。Phase 21 生成 machine-readable distribution target gate，防止把 `code-scan-action` release 或 unreleased repository HEAD 漂移误读成 promptfoo core package parity 已完成或已失败。依据 PRD §Upstream Baseline Freeze Strategy、PRD §Compatibility Harness Design、ADR-007、ADR-009、task-18.3、task-20.2。

## 2. Business Value

Phase 20 已禁止 perfect-refactor claim，但 current-upstream blocker 仍把 npm core package、GitHub latest release 和 repository HEAD 放在同一段原因里。Phase 21 的价值是给后续 rebaseline 一个更小、更可审计的输入：如果 npm core package 仍等于 frozen baseline，则 frozen-baseline evidence 对 published core package 仍有效；如果 GitHub HEAD 或 non-core release 漂移，则只能作为 repository-current / release-channel blocker，而不能覆盖 core package 结论。

## 3. Scope / Modules

`src/compatibility/inventory.rs`、`scripts/release/upstream-distribution-target.sh`、`scripts/release/runtime-smoke.sh`、`target/release-gates/upstream-distribution-target.json`、`docs/compatibility/target-policy.md`、`docs/compatibility/matrix.md`、`docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`、`tests/`、`test/features/perfect-refactor-parity.feature`。

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 21.1 | upstream-distribution-target-gate | ../tasks/task-21.1-upstream-distribution-target-gate.md | Done | 生成 upstream distribution target gate，区分 npm core package、GitHub repo HEAD 和 non-core latest release 对 perfect-refactor claim 的影响 |

## 5. Dependencies

依赖 Phase 20 的 `perfect-refactor-claim.json`、task 18.3 的 `current-upstream-policy.json` 和 PRD 冻结基线定义。无需真实密钥、账号或发布权限；该 phase 只记录公开 upstream/package metadata 和 release target 语义。

## 6. Phase Acceptance Criteria

- [x] `upstream-distribution-target.json` 记录 npm core package version/gitHead/integrity、GitHub repository HEAD、GitHub latest observed release ref，并给出 full SHA / integrity evidence。
- [x] artifact 区分 `npm_core_matches_frozen_baseline`、`repository_head_matches_npm_core`、`github_latest_release_is_core_package`，且不能把 non-core GitHub release 当成 promptfoo core package rebaseline。
- [x] `release-candidate.json` 引用 distribution target artifact，并把 distribution target status 纳入 gate status。
- [x] docs/audit/compatibility matrix 明确说明：published npm core package alignment 可以保留 frozen-baseline claim；repository HEAD 或 non-core release drift 仍阻止 current repository perfect-refactor claim。

## 7. Phase Risks

- npm registry、GitHub tags 和 GitHub Releases 的“latest”语义不同；artifact 必须记录 source 和分类，不能只存一个 latest 字符串。
- current-upstream metadata 会漂移；测试必须使用 fixture，runtime smoke 记录 observed_at，不把时间相关字段做稳定断言。
- 该 phase 不能解除 external authority、publication authority 或 current-mode evidence 缺口；只能缩小 current-upstream 判断的歧义。

## 8. Definition of Done

task 21.1 Done 后，执行 phase §6 smoke：`s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`；检查 `upstream-distribution-target.json`、`current-upstream-policy.json`、`perfect-refactor-claim.json`、`release-candidate.json` 与 docs/audits/compatibility matrix 结论一致。

## 9. Phase Completion Notes

- **完成日期**：2026-05-31
- **Phase smoke**：PASS。执行 `s2v_preflight_phase docs/specs/phases/phase-21-upstream-distribution-target-disambiguation.md` 通过；随后执行 `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`，9 项全部通过。
- **Artifact evidence**：`target/release-gates/upstream-distribution-target.json` 记录 `status=ready-with-drift`、npm core package `0.121.13`、npm `gitHead=4860e990c7e9a2f8f677173fb92cf9867b34d03f`、npm integrity `sha512-DBPSixUophzcD7S7lML6SqVwnVtrhK5A3HsZ03IG9Xrw0t24r5imG7nLj+YMb0vlAjbdFtE7yFG+rsqDpfYp6g==`、`npm_core_matches_frozen_baseline=true`、`repository_head_matches_npm_core=false`、`github_latest_release_is_core_package=false`、`github_latest_release_channel=github-action`、`current_repository_perfect_claim_allowed=false`；`target/release-gates/release-candidate.json.distribution_target` 引用同一 artifact；`target/release-gates/perfect-refactor-claim.json` 仍为 `perfect_refactor_claim_allowed=false`、`local_stable_allowed=true`、`published=false`、`publication_ready=credential-blocked`。
- **保留边界**：Phase 21 只澄清 upstream distribution target，不解除 current repository rebaseline、source accounting、external authority 或 publication authority blocker；不得据此声明当前 repository 已完成 perfect refactor。
