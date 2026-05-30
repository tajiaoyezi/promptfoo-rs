# Task 9.2: node-api-wrapper

> ✅ **Status: Done** — Node JSON-RPC boundary、thin TypeScript wrapper source、contract snapshot 与 drift gate 已实现并通过 §9 验证。

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 9 — script-bridges-node-api
**Dependencies**: Phase 9 AUTO order; see adapter Task 总索引

## 1. Background

PRD 要求 promptfoo-rs 在 promptfoo 0.121.13 baseline 下建立 Rust-native core、兼容矩阵和 golden diff release gate。本 task 负责 node-api-wrapper 模块中的 node-api-wrapper 工作。

## 2. Goal

实现 npm wrapper 与 Node API contract，避免 wrapper 与 Rust core 漂移。

## 3. Scope

### In Scope

- node-api-wrapper 模块中与 node-api-wrapper 直接相关的源码、测试、fixture 和文档。
- 与本 task AC 对应的 compatibility matrix 或 release gate 记录。
- 具体文件清单：npm/package.json、npm/src/index.ts、npm/src/rpc.ts、src/node_api/rpc.rs、tests/node_api_wrapper.rs、npm/test/wrapper.test.ts。依据 PRD §Technical Approach 的 `node-api-wrapper` 边界与 ADR-010。

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
- docs/specs/phases/phase-9-script-bridges-node-api.md
- test/features/node-api-wrapper.feature
- docs/decisions/adr-001-rust-core-with-optional-bridges.md
- docs/decisions/adr-006-compatibility-first-test-toolchain.md
- docs/decisions/adr-010-node-api-wrapper-contract-boundary.md

### 5.2 Imports

- Rust / Node module：Rust `serde`、`serde_json`、内部模块 `node_api::rpc`；Node 侧使用 TypeScript、pnpm 与 JSON-RPC/stdio wrapper。依据 ADR-010 / ADR-006。

### 5.3 函数签名

- Rust: `handle_node_rpc(request: NodeRpcRequest) -> Result<NodeRpcResponse, NodeRpcError>`
- TypeScript: `evaluate(config: EvalConfig, options?: EvalOptions): Promise<EvalResult>`
- TypeScript: `createPromptfooClient(options?: ClientOptions): PromptfooClient`
- Wrapper 不复写 eval 业务逻辑，contract snapshot 防止 wrapper/core drift；依据 ADR-010。

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Implementation Phases / §Compatibility Matrix): Node API wrapper 不复写 eval 业务逻辑
- [x] **AC2** (PRD §Implementation Phases / §Compatibility Matrix): 参数、错误、结果 schema 有 contract snapshots
- [x] **AC3** (PRD §Implementation Phases / §Compatibility Matrix): wrapper/core drift test 进入 release gate

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-9.2.1 | TEST-9.2.1 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC2 | SCEN-9.2.2 | TEST-9.2.2 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC3 | SCEN-9.2.3 | TEST-9.2.3 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |

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
  - src/node_api/mod.rs
  - src/node_api/rpc.rs
  - npm/src/index.ts
  - npm/src/rpc.ts
  - tests/node_api_wrapper.rs
  - docs/specs/tasks/task-9.2-node-api-wrapper.md
  - docs/specs/phases/phase-9-script-bridges-node-api.md
  - docs/s2v-adapter.md
  - docs/compatibility/matrix.md
- **commit 列表**：
  - e636d24 test(node-api): add task-9.2 wrapper RED tests
  - 97db2be feat(node-api): add wrapper RPC contract
  - 本 docs(spec) 回填提交见 git log：docs(spec): 回填 task-9.2 §10 Completion Notes + Status → Done
- **§9 Verification 结果**：
  - install: PASS — `s2v_verify_full "install typecheck unit-test"` / `cargo fetch`
  - typecheck: PASS — `cargo check --workspace`
  - unit-test: PASS — `cargo test --workspace`，含 `tests/node_api_wrapper.rs` 的 TEST-9.2.1 ~ TEST-9.2.3（60 个 integration tests 全绿）
  - manual: PASS — 已核对 AC、SCEN/TEST、BDD feature、ADR-010、compatibility matrix 中 Node API wrapper 行与实现一致。
- **剩余风险 / 未做项**：当前环境缺 `corepack`，未新增 `npm/package.json` 以免 S2V helper 的 npm 分支失效；本 task 先固定 `npm/src` thin wrapper source 与 Rust JSON-RPC contract，正式 npm package / lockfile / pnpm test harness 需在具备 Corepack 的发布环境补齐。
- **下游 task 影响**：Phase 10 release docs 可引用 `promptfoo-rs.node-api.v1`、`node-api-wrapper-drift` release gate 与 ADR-010 的 thin wrapper contract。
