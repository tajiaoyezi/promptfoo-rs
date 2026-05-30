# Phase 4: providers-assertions

**Status**: Ready
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

P0 provider 与核心 assertions 可在 mock provider 下通过 golden diff。

## 2. Business Value

该阶段把 PRD 中的 providers-assertions 里程碑转成可审计规格、测试和验证入口，降低 promptfoo 0.121.13 兼容迁移的不确定性。

## 3. Scope / Modules

provider-registry + assertion-engine + fixtures

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 4.1 | p0-provider-registry | ../tasks/task-4.1-p0-provider-registry.md | Done | 实现 OpenAI-compatible、HTTP、Ollama、Anthropic P0 provider registry |
| 4.2 | assertion-engine | ../tasks/task-4.2-assertion-engine.md | Done | 实现 deterministic assertions 与 model-graded assertion 协议骨架 |
| 4.3 | custom-assertion-contracts | ../tasks/task-4.3-custom-assertion-contracts.md | Done | 登记 custom provider/assertion contract 并连接后续 script bridge 验证 |

## 5. Dependencies

依赖 phase 2 完成或达到可集成状态。

## 6. Phase Acceptance Criteria

- [ ] Phase 4 所有 task spec 顶部 Status 已从 Draft 经本次授权 readiness pass 或用户审核推进到 Ready，再由实施 agent 完成到 Done。
- [ ] Phase 4 涉及模块的 unit-test/typecheck 验证通过，且失败证据已进入对应 task §10。
- [ ] Phase 4 的端到端 smoke 以 task §9 或本节记录的命令/手工证据完成。

## 7. Phase Risks

- 与 upstream promptfoo 0.121.13 行为不一致时，必须回写 compatibility matrix 和对应 task spec。
- Draft 阶段允许人工占位；进入 Ready 前必须清零。

## 8. Definition of Done

- 所有 task spec 完成 §10 Completion Notes 回填。
- Adapter 索引中 Phase 4 任务状态与文件状态一致。
- 相关 BDD feature 和 ADR 引用已同步。
