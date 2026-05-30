# Task 2.1: workspace-cli-skeleton

> ⚠️ **Status: Draft** — 此 spec 含 <TBD-by-user> 字段，禁止进入 /s2v-implement。实施前请填完 §3/§4/§5 的业务字段并把 Status 改为 Ready。

**Status**: Draft
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 2 — config-cli-core
**Dependencies**: Phase 2 AUTO order; see adapter Task 总索引

## 1. Background

PRD 要求 promptfoo-rs 在 promptfoo 0.121.13 baseline 下建立 Rust-native core、兼容矩阵和 golden diff release gate。本 task 负责 cli 模块中的 workspace-cli-skeleton 工作。

## 2. Goal

建立 Rust workspace 与 promptfoo-compatible CLI command skeleton。

## 3. Scope

### In Scope

- cli 模块中与 workspace-cli-skeleton 直接相关的源码、测试、fixture 和文档。
- 与本 task AC 对应的 compatibility matrix 或 release gate 记录。
- <TBD-by-user>：Ready 前补充具体文件清单。

### Out Of Scope

- 不实现本 task AC 之外的长尾 provider/assertion/plugin。
- 不绕过 PRD 的 P0/P1/P2 兼容等级规则。
- 不修改 unrelated phase/task spec。

## 4. Users / Actors

- **AI 应用开发者**：通过 CLI、配置、输出和本地 viewer 感知兼容性。
- **AI infra / 平台工程团队**：在 CI 中依赖 exit code、JUnit/SARIF、golden diff 和 release gate。
- **安全红队团队**：依赖 redteam/MCP/scan/script bridge 的本地可审计执行边界。
- <TBD-by-user>：Ready 前确认本 task 是否还有额外 actor。

## 5. Behavior Contract

本 task 的外部可观察行为以 §6 AC、对应 BDD feature 和 compatibility fixture 为准。任何与 upstream promptfoo 0.121.13 的差异必须登记为 matching / intentional-difference / unsupported / later / upstream-ambiguous / bug。

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/phases/phase-2-config-cli-core.md
- test/features/cli.feature
- docs/decisions/adr-001-rust-core-with-optional-bridges.md
- docs/decisions/adr-004-cli-output-schema-protocol.md
- docs/decisions/adr-006-compatibility-first-test-toolchain.md

### 5.2 Imports

- <TBD-by-user>：Ready 前列出本 task 需要引入的 Rust crate、内部 module、Node/Python bridge 或 fixture helper。

### 5.3 函数签名

- <TBD-by-user>：Ready 前列出本 task 新增/修改的关键函数、trait、CLI handler 或 schema type。

## 6. Acceptance Criteria

- [ ] **AC1** (PRD §Implementation Phases / §Compatibility Matrix): cargo workspace 能承载 core/cli 代码并通过 cargo check
- [ ] **AC2** (PRD §Implementation Phases / §Compatibility Matrix): CLI 暴露 eval/view/cache/redteam/mcp/code-scans/scan-model/import/export skeleton
- [ ] **AC3** (PRD §Implementation Phases / §Compatibility Matrix): 未知命令和无效 flag 按稳定 stderr/exit code 返回

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-2.1.1 | TEST-2.1.1 | N/A until integration harness exists | install, typecheck, unit-test, manual | Not Started |
| AC2 | SCEN-2.1.2 | TEST-2.1.2 | N/A until integration harness exists | install, typecheck, unit-test, manual | Not Started |
| AC3 | SCEN-2.1.3 | TEST-2.1.3 | N/A until integration harness exists | install, typecheck, unit-test, manual | Not Started |

## 8. Risks

- upstream promptfoo 0.121.13 行为未文档化，fixture 可能覆盖不足。
- Windows/macOS/Linux path、env、shell 行为可能漂移。
- Draft 字段未清零就实施会破坏 S2V Ready Gate。

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Manual**: 审核本 task 的 AC、traceability、compatibility matrix 记录与 BDD scenario 是否一致。

## 10. Completion Notes

- **完成日期**：<TBD-after-impl>
- **改动文件**：
  - <TBD-after-impl>
- **commit 列表**：
  - <TBD-after-impl>
- **§9 Verification 结果**：
  - install: <TBD-after-impl>
  - typecheck: <TBD-after-impl>
  - unit-test: <TBD-after-impl>
  - manual: <TBD-after-impl>
- **剩余风险 / 未做项**：<TBD-after-impl>
- **下游 task 影响**：<TBD-after-impl>