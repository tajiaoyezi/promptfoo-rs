# Phase 18: perfect-refactor-blocker-burndown

**Status**: Done
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

把 2026-05-31 完美重构复审中仍阻断 `promptfoo/promptfoo` 完整重构声明的证据缺口转成可执行、可验证、可逐步清零的 blocker burndown：source inventory missing rows、P0 长尾 provider blockers、current-upstream rebaseline、公开发布凭据边界。依据 PRD §Compatibility Matrix / §Success Metrics、ADR-007、ADR-008、ADR-009、docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md。

## 2. Business Value

Phase 17 已证明 frozen baseline 的真实 upstream corpus、长尾分类和安装 dry-run 证据可执行，但“完美重构”仍被 2116 个 source inventory missing rows、37 个 P0 provider module blockers、moving upstream 差异和未发布公开渠道阻断。本 phase 的价值是把这些 blocker 从审计结论推进到可燃尽的工程队列，避免后续实现继续停留在“可审计但未完成”。

## 3. Scope / Modules

`compatibility/inventory/`、`compatibility/matrix/`、`compatibility/fixtures/`、`src/compatibility/`、`src/providers/`、`src/config/`、`scripts/release/`、`target/release-gates/`、`docs/compatibility/`、`docs/release.md`、`docs/audits/`、`tests/`、`test/features/perfect-refactor-parity.feature`。

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 18.1 | source-inventory-ledger-closure | ../tasks/task-18.1-source-inventory-ledger-closure.md | Done | 将 source-extracted items 全量转成 ledger/accounting rows，清零 missing-matrix-row silent omissions，同时保留真实 P0 blocker |
| 18.2 | p0-provider-module-fixture-burndown | ../tasks/task-18.2-p0-provider-module-fixture-burndown.md | Done | 为 37 个 P0 provider module blockers 补 fixture/native evidence 或给出可审计阻断决策 |
| 18.3 | current-upstream-rebaseline-gate | ../tasks/task-18.3-current-upstream-rebaseline-gate.md | Done | 建立 current upstream target mode，防止 frozen baseline 完成被误称为 current promptfoo 完美重构 |
| 18.4 | publication-authority-release-gate | ../tasks/task-18.4-publication-authority-release-gate.md | Done | 把 dry-run installability 与真实公开发布凭据/授权拆分成可验证 gate |

## 5. Dependencies

依赖 Phase 17 的 source-extracted inventory、longtail classification、real P0 corpus、installability reports；依赖 ADR-007/ADR-008/ADR-009 的 release gate 与兼容矩阵政策；依赖当前审计文档列出的 blocker 数字。

## 6. Phase Acceptance Criteria

- [x] source inventory evidence 不再把 source-extracted item 缺少显式矩阵行表现为 silent missing rows；每个 source item 都有 ledger row、level/status/owner/verification/reason，且 P0 未实现项仍为 release blocker。
- [x] 37 个 P0 provider module blockers 被逐项消解为 native fixture、bridge fixture、explicit legal/credential blocker 或 documented waiver；不得用笼统 later 行吞掉 P0。
- [x] current upstream HEAD 与 frozen npm baseline 的目标模式在 machine-readable policy 中分离；任何“完美重构”声明必须指明 target mode，current mode 不可复用 frozen-only evidence。
- [x] release candidate 报告清楚区分 local installability ready、public publication credential-blocked、legal/brand blocked 和 actually published；没有真实凭据时不得把渠道标 published=true。

## 7. Phase Risks

- 批量生成 source ledger 容易变成“把 blocker 改名”；task 18.1 必须保留 P0 blocker 数，且只清除 silent omission 类型。
- P0 provider module 可能需要真实账号、私有服务或品牌授权；遇到这类项按 BLOCKED/waiver 规则留证，不伪造兼容。
- current upstream moving HEAD 会持续变化；task 18.3 必须使用可冻结的 observed ref，不让 release gate 依赖浮动 main。
- 公开发布涉及真实凭据；task 18.4 只能实现 gate 与证据收集，真实发布仍需要授权。

## 8. Definition of Done

四个 task 全部 Done 后，执行 phase §6 smoke：`s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`；检查 `target/release-gates/source-inventory-evidence.json`、`longtail-classification.json`、`release-candidate.json`、current-upstream policy artifact 与 docs/audits 结论一致；确认 adapter / PRD / compatibility matrix / BDD feature 与 task §10 completion notes 一致。

## 9. Phase Completion Notes

- **完成日期**：2026-05-31
- **Phase smoke**：PASS — `s2v_preflight_phase docs/specs/phases/phase-18-perfect-refactor-blocker-burndown.md` 通过；`s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"` 通过 9 项。首次 smoke 暴露 `tests/source_inventory_ledger_closure.rs` rustfmt drift，已用 `ccce5e0` 修正后重跑通过。
- **Artifact evidence**：
  - `target/release-gates/source-inventory-evidence.json` status=`ready-with-blockers`，missing_matrix_rows=0，release_blockers=74，p0_accounting_blocker_count=111。
  - `target/release-gates/longtail-classification.json` status=`ready-with-blockers`，p0_provider_module_burndown initial=37/resolved_by_fixture=13/remaining=24，p0_release_blocker_count=24。
  - `target/release-gates/current-upstream-policy.json` status=`ready`，target_mode=`frozen`，current_perfect_claim_allowed=false。
  - `target/release-gates/publication-authority.json` publication_ready=`credential-blocked`，credential_blocked=true，legal_brand_blocked=true，blockers=6，all channels published=false。
  - `target/release-gates/release-candidate.json` includes `target_policy` and `publication_authority`; release candidate published=false and publication_ready=`credential-blocked`。
- **保留边界**：Phase 18 完成了 blocker 可审计化和 gate 化，不等于完美重构完成：111 个 generated P0 source accounting blockers、24 个 P0 provider module blockers、current upstream HEAD rebaseline、真实发布凭据/授权仍是后续工作边界。
