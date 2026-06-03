# Task 4.3: custom-assertion-contracts

> ✅ **Status: Done** — task-4.3 已按 RED→GREEN→§9 verification 完成，详见 §10 Completion Notes。

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 4 — providers-assertions
**Dependencies**: Phase 4 AUTO order; see adapter Task 总索引

## 1. Background

PRD 要求 promptfoo-rs 在 promptfoo 0.121.13 baseline 下建立 Rust-native core、兼容矩阵和 golden diff release gate。本 task 负责 assertion-engine 模块中的 custom-assertion-contracts 工作。

## 2. Goal

登记 custom provider/assertion contract 并连接后续 script bridge 验证。

## 3. Scope

### In Scope

- assertion-engine 模块中与 custom-assertion-contracts 直接相关的源码、测试、fixture 和文档。
- 与本 task AC 对应的 compatibility matrix 或 release gate 记录。
- 具体文件清单：src/assertions/custom.rs、src/script_bridge/mod.rs、tests/custom_assertion_contracts.rs、compatibility/fixtures/assertions/custom/。依据 PRD §provider/assertion/script bridge 与 ADR-005。

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
- docs/specs/phases/phase-4-providers-assertions.md
- test/features/assertion-engine.feature
- docs/decisions/adr-001-rust-core-with-optional-bridges.md
- docs/decisions/adr-006-compatibility-first-test-toolchain.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md

### 5.2 Imports

- Rust crate / module：`serde`、`serde_json`、`tokio::process`、内部模块 `assertions::custom`、`script_bridge`。依据 ADR-005 / ADR-006 / ADR-009。

### 5.3 函数签名

- `CustomAssertionRequest` / `CustomAssertionResponse` schema types
- `evaluate_custom_assertion(request: CustomAssertionRequest, auth: ScriptAuthorization) -> Result<AssertionResult, ScriptBridgeError>`
- `reject_unauthorized_script(kind: ScriptKind, path: &Path) -> ScriptBridgeError`
- 默认拒绝与输入输出 schema snapshot 是本 task 的外部契约；依据 PRD §Security / ADR-005。

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Implementation Phases / §Compatibility Matrix): JS/Python/Shell custom contract 在矩阵中标 P0/P1 与 bridge 状态
- [x] **AC2** (PRD §Implementation Phases / §Compatibility Matrix): 未启用 allow-scripts 时返回稳定拒绝错误
- [x] **AC3** (PRD §Implementation Phases / §Compatibility Matrix): custom assertion 输入输出 schema 有 snapshot

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-4.3.1 | TEST-4.3.1 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC2 | SCEN-4.3.2 | TEST-4.3.2 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC3 | SCEN-4.3.3 | TEST-4.3.3 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |

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
  - src/assertions/mod.rs
  - src/assertions/custom.rs
  - src/script_bridge/mod.rs
  - tests/custom_assertion_contracts.rs
  - docs/specs/tasks/task-4.3-custom-assertion-contracts.md
- **commit 列表**：
  - cc7a362 test(assertion-engine): add task-4.3 custom assertion RED tests
  - 4b947e7 feat(assertion-engine): add custom assertion script contracts
  - 本 docs(spec) 回填提交见 git log：docs(spec): 回填 task-4.3 §10 Completion Notes + Status → Done
- **§9 Verification 结果**：
  - install: passed — `cargo fetch` 通过；viewer/npm install 按 adapter N/A 跳过。
  - typecheck: passed — `cargo check --workspace` 通过。
  - unit-test: passed — `cargo test --workspace` 通过，TEST-4.3.1~TEST-4.3.3 加入后 30 个 integration tests 全部通过。
  - manual: passed — 已核对 AC1/AC2/AC3、SCEN-4.3.1~4.3.3、TEST-4.3.1~TEST-4.3.3、compatibility matrix 的 JS/TS、Python、Shell/Ruby custom provider/assertion bridge 行与本实现/测试一致；Codex 非交互环境无 `/dev/tty`，manual key 以人工审查记录留证。
- **剩余风险 / 未做项**：本 task 只固定 custom assertion bridge contract、默认拒绝错误与 schema snapshot；实际子进程执行、stdio/env allowlist、timeout 和 redaction 留给 Phase 9 script-bridge。
- **下游 task 影响**：Phase 9 可复用 `ScriptKind` / `ScriptAuthorization` / `ScriptBridgeError` 与 `CustomAssertionRequest` / `CustomAssertionResponse` schema；Phase 5 输出可直接序列化 script rejection error。
