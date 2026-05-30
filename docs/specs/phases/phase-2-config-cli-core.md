# Phase 2: config-cli-core

**Status**: Ready
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

promptfoo-rs eval -c promptfooconfig.yaml 能解析基础配置、env、prompts、tests 并进入 runner。

## 2. Business Value

该阶段把 PRD 中的 config-cli-core 里程碑转成可审计规格、测试和验证入口，降低 promptfoo 0.121.13 兼容迁移的不确定性。

## 3. Scope / Modules

cli + config-loader + eval-runner

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 2.1 | workspace-cli-skeleton | ../tasks/task-2.1-workspace-cli-skeleton.md | Done | 建立 Rust workspace 与 promptfoo-compatible CLI command skeleton |
| 2.2 | config-loader | ../tasks/task-2.2-config-loader.md | Done | 加载 promptfooconfig.yaml/json、redteam.yaml、.env、file prompts 和 CSV/JSON/YAML tests |
| 2.3 | eval-command-smoke | ../tasks/task-2.3-eval-command-smoke.md | Done | 让 eval -c 进入 runner 并在 mock provider 下产生最小结果 |

## 5. Dependencies

依赖 phase 1 完成或达到可集成状态。

## 6. Phase Acceptance Criteria

- [ ] Phase 2 所有 task spec 顶部 Status 已从 Draft 经本次授权 readiness pass 或用户审核推进到 Ready，再由实施 agent 完成到 Done。
- [ ] Phase 2 涉及模块的 unit-test/typecheck 验证通过，且失败证据已进入对应 task §10。
- [ ] Phase 2 的端到端 smoke 以 task §9 或本节记录的命令/手工证据完成。

## 7. Phase Risks

- 与 upstream promptfoo 0.121.13 行为不一致时，必须回写 compatibility matrix 和对应 task spec。
- Draft 阶段允许人工占位；进入 Ready 前必须清零。

## 8. Definition of Done

- 所有 task spec 完成 §10 Completion Notes 回填。
- Adapter 索引中 Phase 2 任务状态与文件状态一致。
- 相关 BDD feature 和 ADR 引用已同步。
