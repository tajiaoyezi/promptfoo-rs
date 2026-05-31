# Phase 16: parity-proof-hardening

**Status**: Ready
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

移除复审中仍可观察到的三类“完美重构”证据缺口：CLI P0 命令仍返回 explicit later、release performance/security/report 仍有合成值、upstream golden smoke 仍主要由本地 test binary 证明。依据 PRD §Core Capabilities / §Success Metrics、ADR-004、ADR-007、ADR-009、task-13.1 §10、task-15.2 §10。

## 2. Business Value

用户迁移 promptfoo 项目时首先触达 CLI 和 release gate。若这些路径仍依赖 later 分类或合成证据，即使 S2V 机械门禁为绿，也不能证明项目已经达到可发布的高可信重构状态。

## 3. Scope / Modules

`src/cli.rs`、`src/viewer_server.rs`、`src/cache/`、`scripts/release/`、`src/release.rs`、`src/compatibility/`、`compatibility/inventory/`、`compatibility/artifacts/`、`tests/cli_command_behavior_closure.rs`、`tests/measured_release_gate_reports.rs`、`tests/real_upstream_smoke_gate.rs`、`test/features/perfect-refactor-parity.feature`。

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 16.1 | cli-command-behavior-closure | ../tasks/task-16.1-cli-command-behavior-closure.md | Ready | 将 view/cache/import/export 从 explicit later 推进到本地可执行兼容行为 |
| 16.2 | measured-release-gate-reports | ../tasks/task-16.2-measured-release-gate-reports.md | Ready | 用实际测量/派生报告替换 runtime-smoke 中的合成 performance/security/release JSON |
| 16.3 | source-extracted-inventory-real-upstream-smoke | ../tasks/task-16.3-source-extracted-inventory-real-upstream-smoke.md | Ready | 用 source extraction 与真实 `promptfoo@0.121.13` smoke 证明矩阵和 golden runner 不是仅靠本地替身 |

## 5. Dependencies

依赖 Phase 15 已完成的 release gate 脚本、task-13.1/13.2 的 CLI/eval 基础、task-12.2/12.3 的 executable harness 与 release gate API。

## 6. Phase Acceptance Criteria

- [ ] `view/cache/import/export` 不再输出 “not yet implemented” 或 `later` 状态；CLI surface 与 item-level matrix 中对应 P0 rows 均有 implemented/native evidence。
- [ ] runtime smoke 生成的 performance/security/release-candidate 报告来自本次执行的测量或 gate summary，不再写入固定 observed 值或无来源 stable 决策。
- [ ] 至少一个 P0 fixture 由真实 `promptfoo@0.121.13` 与当前 `promptfoo-rs` 二进制执行并持久化 artifacts；若真实 upstream 执行缺失，stable gate fail closed。

## 7. Phase Risks

- 真实 upstream npm smoke 依赖公网 npm registry；若 npm 不可用但不需要密钥，应写 blocked evidence，而不是把本地 test binary 冒充 upstream。
- CLI import/export 与 upstream 细节可能存在长尾差异；本阶段先实现 local result artifact 行为，并通过 matrix 标明边界。
- 性能阈值在共享机器上可能抖动；报告必须记录命令、host 和阻断证据。

## 8. Definition of Done

三个 task 全部 Done 后，执行 phase §6 smoke：`s2v_verify_full "install lint typecheck unit-test integration e2e coverage build runtime-smoke"`，并人工检查 `target/release-gates/release-candidate.json`、`target/release-gates/performance.json`、`target/release-gates/security.json`、`compatibility/artifacts/` 中真实 upstream smoke evidence。
