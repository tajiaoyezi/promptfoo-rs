# Phase 23: dynamic-upstream-release-observation

**Status**: Done
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

让 upstream distribution target gate 动态观测 GitHub latest release metadata，而不是固定查询 `refs/tags/code-scan-action-0.1.7`。该 phase 只提升 current-upstream evidence freshness；不解除 source、external authority、publication 或 current rebaseline blockers。依据 PRD §Compatibility Matrix / §Success Metrics、ADR-007、ADR-009、task-21.1 §10、BLOCKED-task-22.1 resumed audit。

## 2. Business Value

Phase 21/22 已经把 npm core package、GitHub HEAD、GitHub release channel 和 perfect-refactor blocker 拆清，但恢复审计发现 GitHub HEAD 已继续漂移，而 release observation 仍使用硬编码 `code-scan-action-0.1.7` tag。动态 latest release 观测可以防止 blocker handoff 使用陈旧 release evidence，同时保持 fail-closed 的 perfect-refactor claim。

## 3. Scope / Modules

`scripts/release/upstream-distribution-target.sh`、`scripts/release/runtime-smoke.sh`、`target/release-gates/upstream-distribution-target.json`、`tests/upstream_distribution_target_gate.rs`、`docs/compatibility/target-policy.md`、`docs/compatibility/matrix.md`、`docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`、`test/features/perfect-refactor-parity.feature`。

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 23.1 | dynamic-github-latest-release-observation | ../tasks/task-23.1-dynamic-github-latest-release-observation.md | Done | 动态解析 GitHub latest release tag，写入 upstream distribution target source evidence，并保持 current perfect-refactor claim fail-closed |

## 5. Dependencies

依赖 task 21.1 的 upstream distribution target gate、task 22.1 的 unblock packet gate、ADR-007 的 upstream evidence policy 和 ADR-009 的 P0/P1/P2 compatibility matrix。该 phase 不需要真实密钥或账号；公开 GitHub latest release metadata 不足以解除 perfect-refactor blocker。

## 6. Phase Acceptance Criteria

- [x] `upstream-distribution-target.sh` 从 `UPSTREAM_GITHUB_RELEASE_FILE` fixture 或 GitHub latest release metadata 动态解析 latest release tag。
- [x] `upstream-distribution-target.json.github.source` 记录实际查询的 latest release ref，而不是固定 `refs/tags/code-scan-action-0.1.7`。
- [x] 当动态 latest release 不是 npm core package release，`github_latest_release_is_core_package=false` 且 `current_repository_perfect_claim_allowed=false` 保持不变。
- [x] runtime smoke、target-policy、matrix、audit 和 BLOCKED 留痕说明该 phase 只刷新 release observation，不解除 external/publication/current blockers。

## 7. Phase Risks

- GitHub latest release API 或 CLI 可能暂时不可用；脚本必须 fail-closed，而不是回退到过期 hard-coded tag。
- latest release tag 可能是 GitHub Action 或其他非 core package channel；分类必须保留 channel distinction。
- 动态 release observation 可能被误读为 current rebaseline；docs 必须继续说明 same-ref inventory/golden/release evidence 仍缺失。

## 8. Definition of Done

task 23.1 Done 后，执行 phase §6 smoke：`s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`；检查 `upstream-distribution-target.json`、`perfect-refactor-claim.json`、`perfect-refactor-unblock-packet.json` 与 docs/audits/compatibility matrix 结论一致，且 perfect-refactor claim 在外部证据缺失时继续保持 false。

## 9. Phase Completion Notes

- **完成日期**：2026-06-01
- **Phase smoke**：PASS — `s2v_preflight_phase docs/specs/phases/phase-23-dynamic-upstream-release-observation.md` passed, then `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"` passed all 9 keys.
- **Artifact evidence**：latest runtime smoke generated `target/release-gates/upstream-distribution-target.json` with `status=ready-with-drift`, npm core `0.121.13` / `4860e990c7e9a2f8f677173fb92cf9867b34d03f`, GitHub HEAD `0b93733d48727be67e34433cb0fb1ad21026863a`, dynamic latest release ref `refs/tags/code-scan-action-0.1.7`, `github_latest_release_channel=github-action`, `github_latest_release_is_core_package=false`, and `current_repository_perfect_claim_allowed=false`. `perfect-refactor-claim.json` remains `perfect_refactor_claim_allowed=false`; `perfect-refactor-unblock-packet.json` remains `status=blocked`, `auto_resolvable=false`, `required_user_decision_count=29`, and `current_upstream_rebaseline_required=true`.
- **保留边界**：Phase 23 only refreshes dynamic latest release observation. It does not change the frozen compatibility target, waive external authority blockers, publish artifacts, or satisfy current-upstream same-ref rebaseline evidence.
