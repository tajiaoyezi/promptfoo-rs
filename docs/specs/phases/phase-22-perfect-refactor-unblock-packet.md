# Phase 22: perfect-refactor-unblock-packet

**Status**: Ready
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

将 Phase 21 后仍阻止 `perfect_refactor_claim_allowed=true` 的 source accounting、external authority、publication authority 和 current-upstream blockers 聚合成一个 machine-readable unblock packet。该 phase 不解除真实凭据、账号、私有服务、法律/品牌或公开发布边界；它把剩余完美重构条件转成可交接、可验证、不可伪造的最小决策清单。依据 PRD §Compatibility Matrix / §Release / §Success Metrics、ADR-007、ADR-008、ADR-009、task-19.4、task-20.2、task-21.1。

## 2. Business Value

当前 artifacts 已证明 local stable 可以在 frozen baseline 下成立，但 perfect-refactor claim 仍因 22 个 source P0 blockers、21 个 external/publication authority blockers、current-upstream drift 和 publication credential/legal blockers 而保持 false。Phase 22 的价值是把这些 blockers 从多个 artifact 中聚合为一个 unblock packet，便于用户或维护者逐项提供授权、凭据、服务证据或正式 waiver；同时防止 agent 在无外部证据时把阻塞项误标为完成。

## 3. Scope / Modules

`src/release.rs`、`scripts/release/perfect-refactor-unblock-packet.sh`、`scripts/release/runtime-smoke.sh`、`target/release-gates/perfect-refactor-unblock-packet.json`、`docs/release.md`、`docs/compatibility/matrix.md`、`docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md`、`tests/`、`test/features/perfect-refactor-parity.feature`。

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 22.1 | authority-unblock-packet-gate | ../tasks/task-22.1-authority-unblock-packet-gate.md | Ready | 生成 perfect-refactor unblock packet，将剩余阻塞转换为最小外部决策/证据清单并纳入 release gate |

## 5. Dependencies

依赖 Phase 19 的 external authority gate、Phase 20 的 perfect-refactor claim contract、Phase 21 的 upstream distribution target gate，以及 runtime smoke 生成的 source inventory、publication authority、external authority、release candidate artifacts。该 phase 不需要真实密钥或账号即可生成阻塞清单；解除清单中的 blocker 仍需要真实外部证据。

## 6. Phase Acceptance Criteria

- [ ] `perfect-refactor-unblock-packet.json` 聚合 `perfect-refactor-claim.json`、`source-inventory-evidence.json`、`external-authority-blockers.json`、`publication-authority.json`、`upstream-distribution-target.json` 的剩余 blocker。
- [ ] packet 明确区分 source-only config blockers、provider/account/product authority blockers、publication channel blockers 和 current-upstream rebaseline requirement，且不得重复计算同一 provider blocker。
- [ ] packet 中每个 unblock item 都包含 `required_actor`、`required_evidence`、`source_artifact`、`release_impact` 和 `auto_resolvable=false`，防止 agent 自动伪造授权。
- [ ] runtime smoke 与 release candidate 引用该 packet；docs/audit/compatibility matrix 说明该 packet 是 handoff/blocker artifact，不是 perfect-refactor completion claim。

## 7. Phase Risks

- packet 可能被误读为 waiver；必须保留 `perfect_refactor_claim_allowed=false` 和 `auto_resolvable=false`。
- provider blockers 同时出现在 source accounting 和 external authority artifacts；实现必须去重而不是膨胀 blocker count。
- publication blockers 需要真实凭据、release authority、法律/品牌确认和外部 URL/digest；本 phase 只能记录，不执行发布。

## 8. Definition of Done

task 22.1 Done 后，执行 phase §6 smoke：`s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`；检查 `perfect-refactor-unblock-packet.json`、`perfect-refactor-claim.json`、`release-candidate.json` 与 docs/audits/compatibility matrix 结论一致，且 perfect-refactor claim 仍在未获外部证据时保持 false。

## 9. Phase Completion Notes

- **完成日期**：<TBD-after-impl>
- **Phase smoke**：<TBD-after-impl>
- **Artifact evidence**：<TBD-after-impl>
- **保留边界**：<TBD-after-impl>
