# Task 9.1: script-bridge-sandbox

> ✅ **Status: Done** — script bridge 默认拒绝、授权 subprocess I/O/timeout、env allowlist 与 redaction 已实现并通过 §9 验证。

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 9 — script-bridges-node-api
**Dependencies**: Phase 9 AUTO order; see adapter Task 总索引

## 1. Background

PRD 要求 promptfoo-rs 在 promptfoo 0.121.13 baseline 下建立 Rust-native core、兼容矩阵和 golden diff release gate。本 task 负责 script-bridge 模块中的 script-bridge-sandbox 工作。

## 2. Goal

实现 JS/TS、Python、Shell/Ruby bridge 的显式授权、env、stdio、timeout 和 redaction。

## 3. Scope

### In Scope

- script-bridge 模块中与 script-bridge-sandbox 直接相关的源码、测试、fixture 和文档。
- 与本 task AC 对应的 compatibility matrix 或 release gate 记录。
- 具体文件清单：src/script_bridge/mod.rs、src/script_bridge/sandbox.rs、src/script_bridge/redaction.rs、tests/script_bridge_sandbox.rs、test/fixtures/script-bridge/。依据 PRD §Security 与 ADR-005。

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
- test/features/script-bridge.feature
- docs/decisions/adr-001-rust-core-with-optional-bridges.md
- docs/decisions/adr-005-explicit-script-authorization.md
- docs/decisions/adr-006-compatibility-first-test-toolchain.md

### 5.2 Imports

- Rust crate / module：`tokio::process`、`serde`、`serde_json`、`tempfile`、内部模块 `script_bridge`。依据 ADR-005 / ADR-006。

### 5.3 函数签名

- `ScriptBridge::execute(request: ScriptRequest, auth: ScriptAuthorization) -> Result<ScriptResponse, ScriptBridgeError>`
- `ScriptSandboxOptions { timeout, env_allowlist, cwd, stdin_limit }`
- `redact_secrets(value: &mut Value, policy: &RedactionPolicy)`
- 默认拒绝、授权后 subprocess I/O、timeout、env allowlist 与 redaction 均需测试；依据 PRD §Security。

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Implementation Phases / §Compatibility Matrix): 未启用 allow-scripts 时拒绝执行并返回稳定错误
- [x] **AC2** (PRD §Implementation Phases / §Compatibility Matrix): 启用后子进程输入输出和超时有 fixture
- [x] **AC3** (PRD §Implementation Phases / §Compatibility Matrix): env allowlist 与 secret redaction 有 tests

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-9.1.1 | TEST-9.1.1 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC2 | SCEN-9.1.2 | TEST-9.1.2 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC3 | SCEN-9.1.3 | TEST-9.1.3 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |

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
  - src/script_bridge/mod.rs
  - src/script_bridge/sandbox.rs
  - src/script_bridge/redaction.rs
  - tests/script_bridge_sandbox.rs
  - docs/specs/tasks/task-9.1-script-bridge-sandbox.md
  - docs/specs/phases/phase-9-script-bridges-node-api.md
  - docs/s2v-adapter.md
  - docs/compatibility/matrix.md
- **commit 列表**：
  - cba444e test(script-bridge): add task-9.1 sandbox RED tests
  - c5a0b7d feat(script-bridge): add sandbox execution contract
  - 本 docs(spec) 回填提交见 git log：docs(spec): 回填 task-9.1 §10 Completion Notes + Status → Done
- **§9 Verification 结果**：
  - install: PASS — `s2v_verify_full "install typecheck unit-test"` / `cargo fetch`
  - typecheck: PASS — `cargo check --workspace`
  - unit-test: PASS — `cargo test --workspace`，含 `tests/script_bridge_sandbox.rs` 的 TEST-9.1.1 ~ TEST-9.1.3（57 个 integration tests 全绿）
  - manual: PASS — 已核对 AC、SCEN/TEST、BDD feature、compatibility matrix 中 JS/TS、Python、Shell/Ruby script bridge 行与实现一致。
- **剩余风险 / 未做项**：本 task 固定 shared sandbox contract；具体 JS/TS、Python、Ruby runtime discovery / adapter wiring 仍需在后续 bridge 扩展中按 compatibility matrix 分语言补 fixture。
- **下游 task 影响**：task 9.2 Node API wrapper 可复用 `ScriptBridgeError` 稳定错误、redaction policy 与显式授权语义；Phase 10 文档需说明 `--allow-scripts` 默认拒绝和 env allowlist 行为。
