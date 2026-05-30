# Task 11.1: current-upstream-target-policy

**Status**: Ready
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 11 — upstream-inventory-baseline
**Dependencies**: Phase 11 order 1; audits finalized at docs/audits/

## 1. Background

审计发现本地 frozen baseline 是 `promptfoo 0.121.13 + 4860e99`，而当前 upstream `origin/main` 可在同一 package version 下继续漂移。必须先明确兼容声明边界，避免把 moving upstream 与 frozen baseline 混为一个 release gate。依据 PRD §Upstream Baseline Freeze Strategy、ADR-007、docs/audits/promptfoo-final-audit-index-2026-05-30.md。

## 2. Goal

建立 compatibility target policy：stable release 只允许绑定一个明确目标，同时可记录 moving upstream 作为未来再基线化输入。

## 3. Scope

### In Scope

- docs/compatibility/baseline.lock.md
- docs/compatibility/target-policy.md
- src/compatibility/baseline_lock.rs
- tests/current_upstream_target_policy.rs
- docs/audits/ 中的最终审计证据引用

### Out Of Scope

- 不在本 task 实现 upstream item inventory。
- 不修改 frozen baseline 的 artifact 事实，除非验证发现 lock 证据错误。

## 4. Users / Actors

- AI infra / 平台工程团队：需要知道 release gate 绑定哪个 promptfoo 目标。
- 开源 maintainer：需要区分 frozen baseline parity 与 moving upstream tracking。
- 企业安全 / 合规团队：需要可审计证据证明 stable release 不引用浮动目标。

## 5. Behavior Contract

Stable release target 必须是 frozen target 或 explicitly rebaselined target；moving upstream 只能生成 tracking report，不得隐式替换 stable baseline。

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/compatibility/baseline.lock.md
- docs/audits/promptfoo-final-audit-index-2026-05-30.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`serde`、`serde_json`、内部模块 `compatibility::baseline_lock`。
- Docs：`docs/compatibility/target-policy.md`。

### 5.3 函数签名

- `CompatibilityTargetPolicy::load(path: &Path) -> Result<CompatibilityTargetPolicy, TargetPolicyError>`
- `validate_single_stable_target(policy: &CompatibilityTargetPolicy) -> TargetPolicyReport`
- `record_moving_upstream_observation(head: &str, package_version: &str) -> UpstreamObservation`

## 6. Acceptance Criteria

- [ ] **AC1** (PRD §Upstream Baseline Freeze Strategy / ADR-007): stable release policy 明确 frozen target 与 moving upstream tracking 的差异。
- [ ] **AC2** (PRD §Compatibility Matrix): policy validator 拒绝 `latest`、`main`、`HEAD` 或同时存在多个 stable targets 的配置。
- [ ] **AC3** (docs/audits/final): moving upstream observation 记录 head、package version、采集时间和来源，不修改 frozen baseline。

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-11.1.1 | TEST-11.1.1 | tests/current_upstream_target_policy.rs | install, typecheck, unit-test, manual | Not Started |
| AC2 | SCEN-11.1.1 | TEST-11.1.2 | tests/current_upstream_target_policy.rs | install, typecheck, unit-test, manual | Not Started |
| AC3 | SCEN-11.1.1 | TEST-11.1.3 | tests/current_upstream_target_policy.rs | install, typecheck, unit-test, manual | Not Started |

## 8. Risks

- upstream package version 与 commit 漂移可能再次发生；policy 必须把 observation append-only 化。
- 错把 moving upstream 当 stable baseline 会让 release gate 不可复现。

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Manual**: 审核 target-policy 与 baseline.lock 不冲突，且 audit references 可追溯。

## 10. Completion Notes

- **完成日期**：<TBD-after-impl>
- **改动文件**：<TBD-after-impl>
- **commit 列表**：<TBD-after-impl>
- **§9 Verification 结果**：<TBD-after-impl>
- **剩余风险 / 未做项**：<TBD-after-impl>
- **下游 task 影响**：<TBD-after-impl>
