# Phase 9: script-bridges-node-api

**Status**: Ready
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

JS/TS、Python、Shell/Ruby custom provider/assertion bridge 与 npm Node API wrapper 可运行并有 drift 测试。

## 2. Business Value

该阶段把 PRD 中的 script-bridges-node-api 里程碑转成可审计规格、测试和验证入口，降低 promptfoo 0.121.13 兼容迁移的不确定性。

## 3. Scope / Modules

script-bridge + node-api-wrapper + bridge fixtures

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 9.1 | script-bridge-sandbox | ../tasks/task-9.1-script-bridge-sandbox.md | Ready | 实现 JS/TS、Python、Shell/Ruby bridge 的显式授权、env、stdio、timeout 和 redaction |
| 9.2 | node-api-wrapper | ../tasks/task-9.2-node-api-wrapper.md | Ready | 实现 npm wrapper 与 Node API contract，避免 wrapper 与 Rust core 漂移 |

## 5. Dependencies

依赖 phase 4, 5, 6 完成或达到可集成状态。

## 6. Phase Acceptance Criteria

- [ ] Phase 9 所有 task spec 顶部 Status 已从 Draft 经本次授权 readiness pass 或用户审核推进到 Ready，再由实施 agent 完成到 Done。
- [ ] Phase 9 涉及模块的 unit-test/typecheck 验证通过，且失败证据已进入对应 task §10。
- [ ] Phase 9 的端到端 smoke 以 task §9 或本节记录的命令/手工证据完成。

## 7. Phase Risks

- 与 upstream promptfoo 0.121.13 行为不一致时，必须回写 compatibility matrix 和对应 task spec。
- Draft 阶段允许人工占位；进入 Ready 前必须清零。

## 8. Definition of Done

- 所有 task spec 完成 §10 Completion Notes 回填。
- Adapter 索引中 Phase 9 任务状态与文件状态一致。
- 相关 BDD feature 和 ADR 引用已同步。