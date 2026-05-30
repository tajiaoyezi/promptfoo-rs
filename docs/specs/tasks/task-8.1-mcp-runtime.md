# Task 8.1: mcp-runtime

> ✅ **Status: Done** — command skeleton、provider protocol snapshot 与 target materialization 错误路径已实现并通过 §9 验证。

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 8 — mcp-scan-audit
**Dependencies**: Phase 8 AUTO order; see adapter Task 总索引

## 1. Background

PRD 要求 promptfoo-rs 在 promptfoo 0.121.13 baseline 下建立 Rust-native core、兼容矩阵和 golden diff release gate。本 task 负责 mcp-runtime 模块中的 mcp-runtime 工作。

## 2. Goal

实现 promptfoo mcp 与 MCP provider 协议快照。

## 3. Scope

### In Scope

- mcp-runtime 模块中与 mcp-runtime 直接相关的源码、测试、fixture 和文档。
- 与本 task AC 对应的 compatibility matrix 或 release gate 记录。
- 具体文件清单：src/mcp/mod.rs、src/mcp/protocol.rs、src/mcp/provider.rs、src/cli.rs、tests/mcp_runtime.rs、test/fixtures/mcp-runtime/。依据 PRD §Technical Approach 的 `mcp-runtime` 边界。

### Out Of Scope

- 不实现本 task AC 之外的长尾 provider/assertion/plugin。
- 不绕过 PRD 的 P0/P1/P2 兼容等级规则。
- 不修改 unrelated phase/task spec。

## 4. Users / Actors

- **AI 应用开发者**：通过 CLI、配置、输出和本地 viewer 感知兼容性。
- **AI infra / 平台工程团队**：在 CI 中依赖 exit code、JUnit/SARIF、golden diff 和 release gate。
- **安全红队团队**：依赖 redteam/MCP/scan/script bridge 的本地可审计执行边界。
- 本 task 无额外 actor；沿用 adapter §Project 中的 AI 应用开发者、AI infra / 平台工程团队、安全红队团队与开源 maintainer。依据 docs/s2v-adapter.md §Project。

## 5. Behavior Contract

本 task 的外部可观察行为以 §6 AC、对应 BDD feature 和 compatibility fixture 为准。任何与 upstream promptfoo 0.121.13 的差异必须登记为 matching / intentional-difference / unsupported / later / upstream-ambiguous / bug。

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/phases/phase-8-mcp-scan-audit.md
- test/features/mcp-runtime.feature
- docs/decisions/adr-001-rust-core-with-optional-bridges.md
- docs/decisions/adr-006-compatibility-first-test-toolchain.md

### 5.2 Imports

- Rust crate / module：`serde`、`serde_json`、`tokio`、内部模块 `mcp`；MCP protocol 先以稳定 JSON schema 自建，新增外部 MCP crate 需补 ADR。依据 PRD §Technical Approach / ADR-002 / ADR-006。

### 5.3 函数签名

- `handle_mcp_command(args: McpArgs) -> Result<ExitCode, CliError>`
- `McpProvider::call(request: McpRequest) -> Result<McpResponse, McpError>`
- `materialize_mcp_target(config: McpTargetConfig) -> Result<McpTarget, McpError>`
- 本 task 固定 command skeleton、provider protocol snapshot 与 target materialization 错误路径；依据 PRD §Compatibility Matrix。

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Implementation Phases / §Compatibility Matrix): promptfoo mcp command skeleton 可运行
- [x] **AC2** (PRD §Implementation Phases / §Compatibility Matrix): MCP provider request/response 有 protocol snapshot
- [x] **AC3** (PRD §Implementation Phases / §Compatibility Matrix): MCP target materialization 错误路径稳定

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-8.1.1 | TEST-8.1.1 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC2 | SCEN-8.1.2 | TEST-8.1.2 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC3 | SCEN-8.1.3 | TEST-8.1.3 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |

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

- **完成日期**：2026-05-30
- **改动文件**：
  - src/lib.rs
  - src/cli.rs
  - src/mcp/mod.rs
  - src/mcp/protocol.rs
  - src/mcp/provider.rs
  - tests/mcp_runtime.rs
  - docs/specs/tasks/task-8.1-mcp-runtime.md
  - docs/specs/phases/phase-8-mcp-scan-audit.md
  - docs/s2v-adapter.md
  - docs/compatibility/matrix.md
- **commit 列表**：
  - f73bc4f test(mcp): add task-8.1 runtime RED tests
  - 9c4cf12 feat(mcp): add runtime protocol snapshot
  - 本 docs(spec) 回填提交见 git log：docs(spec): 回填 task-8.1 §10 Completion Notes + Status → Done
- **§9 Verification 结果**：
  - install: PASS — `s2v_verify_full "install typecheck unit-test"` / `cargo fetch`
  - typecheck: PASS — `cargo check --workspace`
  - unit-test: PASS — `cargo test --workspace`，含 `tests/mcp_runtime.rs` 的 TEST-8.1.1 ~ TEST-8.1.3（51 个 integration tests 全绿）
  - manual: PASS — 已核对 AC、SCEN/TEST、BDD feature、compatibility matrix 中 MCP 行与实现一致。
- **剩余风险 / 未做项**：真实 MCP server/process/HTTP transport 深度互操作仍属于后续扩展；本 task 固定 command skeleton、JSON-RPC protocol snapshot 与 target validation 错误契约。
- **下游 task 影响**：task 8.2 与 Phase 9 可复用已公开的 `mcp` 模块导出、CLI 子命令错误处理模式和兼容矩阵登记方式。
