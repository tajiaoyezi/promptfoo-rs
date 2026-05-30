# Phase 5: output-ci

**Status**: Done
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

JSON/JSONL/CSV/YAML/JUnit/SARIF/HTML 输出和 CLI exit code 协议稳定。

## 2. Business Value

该阶段把 PRD 中的 output-ci 里程碑转成可审计规格、测试和验证入口，降低 promptfoo 0.121.13 兼容迁移的不确定性。

## 3. Scope / Modules

output-writers + cli + schema snapshots

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 5.1 | result-store-schema | ../tasks/task-5.1-result-store-schema.md | Done | 定义 JSONL/SQLite result schema 与大结果流式写入策略 |
| 5.2 | output-ci-contracts | ../tasks/task-5.2-output-ci-contracts.md | Done | 实现 JSON/JSONL/CSV/YAML/JUnit/SARIF/HTML 输出与 exit code/stdout/stderr 合约 |

## 5. Dependencies

依赖 phase 2 完成或达到可集成状态。

## 6. Phase Acceptance Criteria

- [x] Phase 5 所有 task spec 顶部 Status 已从 Draft 经本次授权 readiness pass 或用户审核推进到 Ready，再由实施 agent 完成到 Done。
- [x] Phase 5 涉及模块的 unit-test/typecheck 验证通过，且失败证据已进入对应 task §10。
- [x] Phase 5 的端到端 smoke 以 task §9 或本节记录的命令/手工证据完成：2026-05-30 `s2v_preflight_phase docs/specs/phases/phase-5-output-ci.md && cargo test --workspace` 通过，36 integration tests passed / 0 failed。

## 7. Phase Risks

- 与 upstream promptfoo 0.121.13 行为不一致时，必须回写 compatibility matrix 和对应 task spec。
- Draft 阶段允许人工占位；进入 Ready 前必须清零。

## 8. Definition of Done

- 所有 task spec 完成 §10 Completion Notes 回填。
- Adapter 索引中 Phase 5 任务状态与文件状态一致。
- 相关 BDD feature 和 ADR 引用已同步。
