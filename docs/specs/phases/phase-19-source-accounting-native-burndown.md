# Phase 19: source-accounting-native-burndown

**Status**: Done
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

把 Phase 18 暴露但未清零的 P0 source accounting/provider blockers 从“显式阻断”推进到真实燃尽：先纠正 `src/app/**` viewer 配置项被误算为 P0 core config 的分类，再为剩余 core config 和非外部 provider 模块补 fixture/native evidence，最后把确实需要真实账号/产品授权的项集中到 external authority gate。依据 PRD §Compatibility Matrix / §Success Metrics、ADR-007、ADR-009、docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md、Phase 18 §9。

## 2. Business Value

Phase 18 已让 silent omissions 变成可审计 blocker，但 `source-inventory-ledger.json` 仍有 111 个 generated P0 accounting blockers，`longtail-classification.json` 仍有 24 个 P0 provider module blockers。Phase 19 的价值是把这些 blocker 继续拆成可实际减少的工程队列，避免“完美重构”目标停留在可审计但未燃尽。

## 3. Scope / Modules

`src/compatibility/inventory.rs`、`src/compatibility/provider_assertion.rs`、`scripts/release/source-inventory-evidence.sh`、`scripts/release/longtail-classification.sh`、`compatibility/inventory/`、`compatibility/matrix/`、`compatibility/fixtures/`、`docs/compatibility/`、`docs/audits/`、`tests/`、`test/features/perfect-refactor-parity.feature`。

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 19.1 | viewer-config-source-reclassification | ../tasks/task-19.1-viewer-config-source-reclassification.md | Done | 将 `src/app/**` viewer config/editor 源文件从 generated P0 core config blocker 纠正为 P1 Local Web viewer accounting evidence |
| 19.2 | core-config-source-fixture-burndown | ../tasks/task-19.2-core-config-source-fixture-burndown.md | Done | 为剩余 non-app core config rows 补 native/bridge fixture 或显式 blocker |
| 19.3 | provider-request-response-fixture-burndown | ../tasks/task-19.3-provider-request-response-fixture-burndown.md | Done | 为不需要真实账号授权的 provider module blockers 补 dedicated request/response fixtures |
| 19.4 | external-authority-blocker-waiver-gate | ../tasks/task-19.4-external-authority-blocker-waiver-gate.md | Done | 把 Codex/Agents/Assistant/Billing/ChatKit/Realtime/Claude Code/真实发布等外部授权项集中为不可伪造的 waiver/blocker gate |

## 5. Dependencies

依赖 Phase 18 的 `source-inventory-ledger.json`、`source-inventory-evidence.json`、`longtail-classification.json`、`publication-authority.json`；依赖 PRD 中 Local Web viewer=P1、promptfooconfig/env/files=P0、核心 provider=P0 的分级；依赖 ADR-009 的 P0/P1/P2 矩阵政策。

## 6. Phase Acceptance Criteria

- [x] `src/app/**` viewer config/editor rows 不再计入 P0 core config accounting blocker；它们必须保留为 P1 Local Web viewer evidence，并带有 reason、owner、verification。
- [x] non-app config source rows 要么有 native/bridge fixture evidence，要么保留 explicit blocker；不得把 core config P0 降级成 viewer P1。
- [x] 不需要真实账号/私有服务的 provider module blockers 要有 dedicated request/response fixture evidence；仍需账号/产品授权的项进入 external authority gate。
- [x] Phase 19 artifacts 继续阻止“完美重构”过度声明：current-upstream、真实发布、外部产品授权和剩余 P0 blocker 必须可机读。

## 7. Phase Risks

- 误把 core config 降级为 viewer P1 会弱化 PRD P0 compatibility；task 19.1 必须用路径白名单限定 `src/app/**`。
- provider fixtures 可能触及真实服务协议；没有真实账号和授权时只能使用 mock/recorded contract，不得伪造 live parity。
- external authority blocker 只能让边界更清楚，不能替代用户提供凭据、账号或法律/品牌确认。

## 8. Definition of Done

四个 task 全部 Done 后，执行 phase §6 smoke：`s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`；检查 `target/release-gates/source-inventory-evidence.json`、`source-inventory-ledger.json`、`longtail-classification.json`、`release-candidate.json`、`publication-authority.json` 与 docs/audits/compatibility matrix 结论一致。

## 9. Phase Completion Notes

- **完成日期**：2026-05-31
- **Phase smoke**：PASS — `s2v_preflight_phase docs/specs/phases/phase-19-source-accounting-native-burndown.md` 通过；随后 `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"` 通过 9 项。
- **Artifact evidence**：
  - `target/release-gates/source-inventory-evidence.json`: status=`ready-with-blockers`, `viewer_config_reclassified_count=56`, `p0_accounting_blocker_count=44`。
  - `target/release-gates/longtail-classification.json`: status=`ready-with-blockers`, `p0_provider_module_burndown.remaining_blocker_count=15`, `external_authority_blocker_count=15`, `generic_blocker_count=0`, `p0_release_blocker_count=15`。
  - `target/release-gates/external-authority-blockers.json`: status=`blocked`, `blocker_count=21`, `provider_external_blocker_count=15`, `publication_blocker_count=6`, `ready_count=0`。
  - `target/release-gates/publication-authority.json`: publication_ready=`credential-blocked`, `credential_blocked=true`, `legal_brand_blocked=true`, blockers=6。
  - `target/release-gates/release-candidate.json`: `stable_allowed=true` for local release gates while `external_authority.status=blocked`, `publication_authority.publication_ready=credential-blocked`, and `published=false` preserve non-perfect/publication boundaries.
- **保留边界**：Phase 19 burns down generated/provider ambiguity but does not claim current-upstream parity, public publication, or external product/account authority. Remaining blockers are explicit and machine-readable: 44 P0 source accounting blockers, 15 provider external-authority blockers, and 6 publication authority blockers.
