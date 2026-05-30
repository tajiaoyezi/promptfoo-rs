# Phase 10: web-viewer-release

**Status**: Ready
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

本地 viewer 可读取结果并完成跨平台发布、安装、文档和 release gate 汇总。

## 2. Business Value

该阶段把 PRD 中的 web-viewer-release 里程碑转成可审计规格、测试和验证入口，降低 promptfoo 0.121.13 兼容迁移的不确定性。

## 3. Scope / Modules

web-viewer + release scripts + README + docs + GitHub Actions

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 10.1 | web-viewer | ../tasks/task-10.1-web-viewer.md | Ready | 实现本地 viewer 读取 JSONL/SQLite 结果、筛选失败样本和导出 |
| 10.2 | release-docs-packaging | ../tasks/task-10.2-release-docs-packaging.md | Ready | 完成 GitHub Releases、Homebrew、Cargo、Docker、npm wrapper、GitHub Action 示例和贡献文档 |

## 5. Dependencies

依赖 phase 6, 7, 8, 9 完成或达到可集成状态。

## 6. Phase Acceptance Criteria

- [ ] Phase 10 所有 task spec 顶部 Status 已从 Draft 经本次授权 readiness pass 或用户审核推进到 Ready，再由实施 agent 完成到 Done。
- [ ] Phase 10 涉及模块的 unit-test/typecheck 验证通过，且失败证据已进入对应 task §10。
- [ ] Phase 10 的端到端 smoke 以 task §9 或本节记录的命令/手工证据完成。

## 7. Phase Risks

- 与 upstream promptfoo 0.121.13 行为不一致时，必须回写 compatibility matrix 和对应 task spec。
- Draft 阶段允许人工占位；进入 Ready 前必须清零。

## 8. Definition of Done

- 所有 task spec 完成 §10 Completion Notes 回填。
- Adapter 索引中 Phase 10 任务状态与文件状态一致。
- 相关 BDD feature 和 ADR 引用已同步。