# Phase 7: redteam-core

**Status**: Ready
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

redteam init/generate/eval/run/report 最小兼容闭环、核心插件/strategy registry、风险评分和 report 输出可运行。

## 2. Business Value

该阶段把 PRD 中的 redteam-core 里程碑转成可审计规格、测试和验证入口，降低 promptfoo 0.121.13 兼容迁移的不确定性。

## 3. Scope / Modules

redteam-engine + config-loader + output-writers + redteam fixtures

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 7.1 | redteam-command-flow | ../tasks/task-7.1-redteam-command-flow.md | Done | 实现 redteam init/generate/eval/run/report 最小兼容闭环 |
| 7.2 | redteam-registry-report | ../tasks/task-7.2-redteam-registry-report.md | Done | 实现核心插件/strategy registry、风险评分和 report 输出 |

## 5. Dependencies

依赖 phase 4, 5, 6 完成或达到可集成状态。

## 6. Phase Acceptance Criteria

- [ ] Phase 7 所有 task spec 顶部 Status 已从 Draft 经本次授权 readiness pass 或用户审核推进到 Ready，再由实施 agent 完成到 Done。
- [ ] Phase 7 涉及模块的 unit-test/typecheck 验证通过，且失败证据已进入对应 task §10。
- [ ] Phase 7 的端到端 smoke 以 task §9 或本节记录的命令/手工证据完成。

## 7. Phase Risks

- 与 upstream promptfoo 0.121.13 行为不一致时，必须回写 compatibility matrix 和对应 task spec。
- Draft 阶段允许人工占位；进入 Ready 前必须清零。

## 8. Definition of Done

- 所有 task spec 完成 §10 Completion Notes 回填。
- Adapter 索引中 Phase 7 任务状态与文件状态一致。
- 相关 BDD feature 和 ADR 引用已同步。
