<!--
  Pull Request 模板 · promptfoo-rs
  本项目遵循 S2V：行为变更走 RED -> GREEN -> verification -> completion notes。
  详见 AGENTS.md、docs/s2v-adapter.md、CONTRIBUTING.md。
  必填字段请勿删除；不适用处填 N/A。
-->

## Summary · 概述

<!-- 1-3 句：这个 PR 做了什么，解决哪个 task / issue -->

## 关联 Task / Issue

- Task spec: `docs/specs/tasks/task-XX.Y-<slug>.md`（若实施某 task；纯基础设施 / 文档可填 N/A）
- Closes: #

## 变更类型（多选）

- [ ] 代码实施（task 的 GREEN 实现）
- [ ] Spec / phase / task 状态翻转
- [ ] 兼容性证据（matrix / golden corpus / fixture）
- [ ] Release / gate / 发布证据
- [ ] 文档 / 示例
- [ ] 基础设施 / CI

## S2V 检查（行为变更必填；纯文档 / 基础设施可整体 N/A）

- [ ] 对应 task spec 状态为 `Ready` 后再实施
- [ ] **先加 RED 测试**，再做最小 GREEN 实现
- [ ] 已运行该 task 声明的 verification keys（如 `install lint typecheck unit-test integration e2e coverage build runtime-smoke`）
- [ ] 已回填 task **completion notes**（commit、验证结果、风险、下游影响）
- [ ] 仅在验证通过后才把 task 状态置为 `Done`

## 兼容性影响

- [ ] 不涉及 promptfoo 兼容行为
- [ ] 涉及：已按分级补证据 → P0 = fixture / golden-diff，P1 = snapshot / 协议，P2 = 登记 unsupported / later / bridge 理由
- [ ] 未分类的新差异已作为 **blocker** 保留在矩阵中，未删除

## 发布 / 文档主张

- [ ] 本 PR 未引入被禁止的 claim（"bug-free" / "no potential bugs" / "完整替代最新 promptfoo" / "public stable release 可用"）
- [ ] 如涉及就绪 / 兼容主张，措辞限于 release gate 实际允许的范围（如 "no known release-blocking defects under the declared gates"）

## Test Plan · 验证计划

<!-- 逐项对照 task Acceptance；基础设施 PR 描述自检项 -->

- [ ]
- [ ]

## Breaking Changes

- [ ] N/A
- [ ] 有（描述 + 迁移说明）

## 安全自检

- [ ] 未提交任何 secret（provider API key、token、账号凭据、registry token）
- [ ] 新增网络 / 脚本 / 上传行为均显式且默认 fail-closed（符合 local-first 安全默认）

## Related · 相关

- ADR: `docs/decisions/adr-XXX.md`
- 前置 PR: #
