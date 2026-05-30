# Phase 1: baseline-freeze

**Status**: Ready
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

冻结 promptfoo 0.121.13 + 4860e99，生成 baseline lock 和完整兼容矩阵骨架。

## 2. Business Value

该阶段把 PRD 中的 baseline-freeze 里程碑转成可审计规格、测试和验证入口，降低 promptfoo 0.121.13 兼容迁移的不确定性。

## 3. Scope / Modules

compat-harness + docs/compatibility/baseline.lock.md + docs/compatibility/matrix.md

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 1.1 | baseline-lock | ../tasks/task-1.1-baseline-lock.md | Done | 写入可追溯 baseline lock，固定 tag、commit、npm artifact、container artifact 四重证据 |
| 1.2 | compatibility-matrix | ../tasks/task-1.2-compatibility-matrix.md | Done | 建立 promptfoo 0.121.13 已文档化能力域的完整兼容矩阵骨架 |

## 5. Dependencies

无前置 phase。

## 6. Phase Acceptance Criteria

- [ ] Phase 1 所有 task spec 顶部 Status 已从 Draft 经本次授权 readiness pass 或用户审核推进到 Ready，再由实施 agent 完成到 Done。
- [ ] Phase 1 涉及模块的 unit-test/typecheck 验证通过，且失败证据已进入对应 task §10。
- [ ] Phase 1 的端到端 smoke 以 task §9 或本节记录的命令/手工证据完成。

## 7. Phase Risks

- 与 upstream promptfoo 0.121.13 行为不一致时，必须回写 compatibility matrix 和对应 task spec。
- Draft 阶段允许人工占位；进入 Ready 前必须清零。

## 8. Definition of Done

- 所有 task spec 完成 §10 Completion Notes 回填。
- Adapter 索引中 Phase 1 任务状态与文件状态一致。
- 相关 BDD feature 和 ADR 引用已同步。
