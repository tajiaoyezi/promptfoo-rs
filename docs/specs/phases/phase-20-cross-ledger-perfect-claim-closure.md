# Phase 20: cross-ledger-perfect-claim-closure

**Status**: Done
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

消除 Phase 19 后剩余的跨 gate 口径不一致：source accounting 不再把 provider burndown 已证明的 22 个 fixture-covered provider rows 计为 P0 accounting blockers，并新增完美重构 claim contract，把 local stable release gate 与 perfect-refactor claim 明确分离。依据 PRD §Compatibility Matrix / §Success Metrics、ADR-007、ADR-008、ADR-009、Phase 19 §9 artifact evidence。

## 2. Business Value

Phase 19 已把 provider 专属 blocker 降到 15 个 external-authority rows，但 `source-inventory-evidence.json` 仍报告 44 个 P0 accounting blockers，其中 22 个是已由 `longtail-classification.json` 证明 fixture-covered 的 provider rows。Phase 20 的价值是让 release/audit artifacts 共享同一 blocker 口径，并防止 `stable_allowed=true` 被误读为“promptfoo 完美重构完成”。

## 3. Scope / Modules

`src/compatibility/inventory.rs`、`src/compatibility/provider_assertion.rs`、`src/release.rs`、`scripts/release/source-inventory-evidence.sh`、`scripts/release/longtail-classification.sh`、`scripts/release/runtime-smoke.sh`、`target/release-gates/`、`docs/compatibility/`、`docs/audits/`、`tests/`、`test/features/perfect-refactor-parity.feature`。

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 20.1 | source-provider-accounting-reconciliation | ../tasks/task-20.1-source-provider-accounting-reconciliation.md | Done | 让 source accounting ledger 消费 provider burndown evidence，已 fixture-covered provider rows 不再计入 P0 accounting blocker |
| 20.2 | perfect-refactor-claim-contract | ../tasks/task-20.2-perfect-refactor-claim-contract.md | Done | 新增完美重构 claim contract，组合 source/current/publication/external authority gate，防止 local stable gate 被误读为目标完成 |

## 5. Dependencies

依赖 Phase 19 的 `source-inventory-evidence.json`、`longtail-classification.json`、`external-authority-blockers.json`、`release-candidate.json`；依赖 ADR-007/ADR-008/ADR-009 对 P0 blocker、publication authority、compatibility matrix 的定义。

## 6. Phase Acceptance Criteria

- [x] source accounting 的 provider rows 与 provider burndown 一致：fixture-covered provider rows 不再出现在 `remaining_p0_blockers`。
- [x] `p0_accounting_blocker_count` 从 44 收敛到 22，且 22 = 7 config external blockers + 15 provider external-authority blockers。
- [x] release candidate 或相邻 artifact 明确给出 `perfect_refactor_claim_allowed=false`，并列出 source/current/publication/external blockers。
- [x] 文档和 audit 不再把 local stable gate 与 perfect-refactor completion 混为一谈。

## 7. Phase Risks

- 不能通过删除 provider rows 或弱化 P0 来降低 blocker count；必须引用 fixture/external-authority evidence。
- `stable_allowed=true` 是 local release gate 结果，不等价于 perfect-refactor claim；新增 artifact 必须避免破坏已有 release gate 语义。
- External-authority blockers 不能在没有凭据/授权时改为 ready。

## 8. Definition of Done

两个 task 全部 Done 后，执行 phase §6 smoke：`s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`；检查 `source-inventory-evidence.json`、`longtail-classification.json`、`external-authority-blockers.json`、`release-candidate.json`、新 claim artifact 与 docs/audits/compatibility matrix 结论一致。

## 9. Phase Completion Notes

- **完成日期**：2026-05-31
- **Phase smoke**：PASS — `s2v_preflight_phase docs/specs/phases/phase-20-cross-ledger-perfect-claim-closure.md` 通过；随后执行 `s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`，9 项全部通过。
- **Artifact evidence**：
  - `target/release-gates/source-inventory-evidence.json`：`status=ready-with-blockers`，`p0_accounting_blocker_count=22`，`remaining_p0_blockers=22`。
  - `provider_source_accounting_reconciliation`：`provider_source_total=37`，`resolved_provider_fixture_count=22`，`provider_external_authority_count=15`，`provider_source_generic_blocker_count=0`。
  - `target/release-gates/perfect-refactor-claim.json`：`perfect_refactor_claim_allowed=false`，`local_stable_allowed=true`，`published=false`，`publication_ready=credential-blocked`，`external_authority_blocker_count=21`，`blockers=4`。
  - `target/release-gates/release-candidate.json`：`stable_allowed=true`，`published=false`，`external_authority.status=blocked`，`publication_authority.publication_ready=credential-blocked`，`perfect_refactor_claim.blocker_count=4`。
  - `target/release-gates/external-authority-blockers.json`：`status=blocked`，`blockers=21`。
- **保留边界**：Phase 20 完成的是 cross-ledger 口径闭合和 perfect-refactor claim gate；它不解除真实密钥、账号、私有服务权限、法律/品牌/发布权限或 current-upstream rebaseline blocker。因此 local stable gate 可通过，但不能公开声称 promptfoo perfect-refactor completion。
