# Phase 6: compatibility-harness

**Status**: Ready
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

upstream 与 promptfoo-rs 的 P0 golden diff、P1 snapshot 和 release gate 自动化可运行。

## 2. Business Value

该阶段把 PRD 中的 compatibility-harness 里程碑转成可审计规格、测试和验证入口，降低 promptfoo 0.121.13 兼容迁移的不确定性。

## 3. Scope / Modules

compat-harness + fixtures + CI scripts

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 6.1 | upstream-harness-runner | ../tasks/task-6.1-upstream-harness-runner.md | Ready | 执行 upstream promptfoo@0.121.13 与 promptfoo-rs 的同输入 fixture runner |
| 6.2 | golden-diff-release-gate | ../tasks/task-6.2-golden-diff-release-gate.md | Ready | 实现 golden diff 分类、coverage report 和 stable release gate |

## 5. Dependencies

依赖 phase 1, 3, 4, 5 完成或达到可集成状态。

## 6. Phase Acceptance Criteria

- [ ] Phase 6 所有 task spec 顶部 Status 已从 Draft 经本次授权 readiness pass 或用户审核推进到 Ready，再由实施 agent 完成到 Done。
- [ ] Phase 6 涉及模块的 unit-test/typecheck 验证通过，且失败证据已进入对应 task §10。
- [ ] Phase 6 的端到端 smoke 以 task §9 或本节记录的命令/手工证据完成。

## 7. Phase Risks

- 与 upstream promptfoo 0.121.13 行为不一致时，必须回写 compatibility matrix 和对应 task spec。
- Draft 阶段允许人工占位；进入 Ready 前必须清零。

## 8. Definition of Done

- 所有 task spec 完成 §10 Completion Notes 回填。
- Adapter 索引中 Phase 6 任务状态与文件状态一致。
- 相关 BDD feature 和 ADR 引用已同步。