# Phase 3: eval-runner-cache

**Status**: Draft
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

runner 支持并发、retry、delay、cache、resume、partial result 和 cancellation。

## 2. Business Value

该阶段把 PRD 中的 eval-runner-cache 里程碑转成可审计规格、测试和验证入口，降低 promptfoo 0.121.13 兼容迁移的不确定性。

## 3. Scope / Modules

eval-runner + cache-resume-store + integration tests

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 3.1 | scheduler-runtime | ../tasks/task-3.1-scheduler-runtime.md | Draft | 实现 eval graph 调度、max-concurrency、delay、cancellation 和 partial failure |
| 3.2 | cache-resume-retry | ../tasks/task-3.2-cache-resume-retry.md | Draft | 实现 cache key、resume cursor、retry-errors 和 backoff 行为 |

## 5. Dependencies

依赖 phase 2 完成或达到可集成状态。

## 6. Phase Acceptance Criteria

- [ ] Phase 3 所有 task spec 顶部 Status 已从 Draft 经用户审核后推进到 Ready，再由实施 agent 完成到 Done。
- [ ] Phase 3 涉及模块的 unit-test/typecheck 验证通过，且失败证据已进入对应 task §10。
- [ ] Phase 3 的端到端 smoke 以 task §9 或本节记录的命令/手工证据完成。

## 7. Phase Risks

- 与 upstream promptfoo 0.121.13 行为不一致时，必须回写 compatibility matrix 和对应 task spec。
- Draft 阶段允许 <TBD-by-user>；进入 Ready 前必须清零。

## 8. Definition of Done

- 所有 task spec 完成 §10 Completion Notes 回填。
- Adapter 索引中 Phase 3 任务状态与文件状态一致。
- 相关 BDD feature 和 ADR 引用已同步。