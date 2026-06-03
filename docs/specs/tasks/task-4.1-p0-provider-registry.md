# Task 4.1: p0-provider-registry

> ✅ **Status: Done** — task-4.1 已按 RED→GREEN→§9 verification 完成，详见 §10 Completion Notes。

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 4 — providers-assertions
**Dependencies**: Phase 4 AUTO order; see adapter Task 总索引

## 1. Background

PRD 要求 promptfoo-rs 在 promptfoo 0.121.13 baseline 下建立 Rust-native core、兼容矩阵和 golden diff release gate。本 task 负责 provider-registry 模块中的 p0-provider-registry 工作。

## 2. Goal

实现 OpenAI-compatible、HTTP、Ollama、Anthropic P0 provider registry。

## 3. Scope

### In Scope

- provider-registry 模块中与 p0-provider-registry 直接相关的源码、测试、fixture 和文档。
- 与本 task AC 对应的 compatibility matrix 或 release gate 记录。
- 具体文件清单：src/providers/mod.rs、src/providers/openai.rs、src/providers/http.rs、src/providers/ollama.rs、src/providers/anthropic.rs、tests/provider_registry.rs、compatibility/fixtures/providers/。依据 PRD §Compatibility Matrix 的 P0 providers。

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
- test/features/provider-registry.feature
- docs/decisions/adr-001-rust-core-with-optional-bridges.md
- docs/decisions/adr-006-compatibility-first-test-toolchain.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md

### 5.2 Imports

- Rust crate / module：`reqwest`、`serde`、`serde_json`、`tracing`、内部模块 `providers`；mock server fixture helper 放在 tests/support/mock_server.rs。依据 ADR-002 / ADR-006 / ADR-009。

### 5.3 函数签名

- `ProviderRegistry::register_p0_defaults() -> ProviderRegistry`
- `ProviderRegistry::resolve(id: &ProviderId) -> Result<Arc<dyn Provider>, ProviderError>`
- `normalize_provider_request(config: ProviderConfig, input: ProviderInput) -> ProviderRequest`
- P0 provider 覆盖 OpenAI-compatible、HTTP、Ollama、Anthropic，网络调用仅走 mock server；依据 PRD §Compatibility Matrix。

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Implementation Phases / §Compatibility Matrix): 四类 P0 provider 均有 request/response snapshot
- [x] **AC2** (PRD §Implementation Phases / §Compatibility Matrix): provider-scoped env/header/model/options 被归一化
- [x] **AC3** (PRD §Implementation Phases / §Compatibility Matrix): 网络调用通过 mock server 验证，不依赖真实模型

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-4.1.1 | TEST-4.1.1 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC2 | SCEN-4.1.2 | TEST-4.1.2 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC3 | SCEN-4.1.3 | TEST-4.1.3 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |

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
  - Cargo.toml
  - Cargo.lock
  - src/lib.rs
  - src/providers/mod.rs
  - src/providers/openai.rs
  - src/providers/http.rs
  - src/providers/ollama.rs
  - src/providers/anthropic.rs
  - tests/provider_registry.rs
  - docs/specs/tasks/task-4.1-p0-provider-registry.md
- **commit 列表**：
  - f0c2634 test(provider-registry): add task-4.1 P0 provider RED tests
  - a328839 feat(provider-registry): implement P0 provider normalization
  - 本 docs(spec) 回填提交见 git log：docs(spec): 回填 task-4.1 §10 Completion Notes + Status → Done
- **§9 Verification 结果**：
  - install: passed — `cargo fetch` 通过；viewer/npm install 按 adapter N/A 跳过。
  - typecheck: passed — `cargo check --workspace` 通过。
  - unit-test: passed — `cargo test --workspace` 通过，TEST-4.1.1~TEST-4.1.3 加入后 24 个 integration tests 全部通过。
  - manual: passed — 已核对 AC1/AC2/AC3、SCEN-4.1.1~4.1.3、TEST-4.1.1~TEST-4.1.3、compatibility matrix 的 OpenAI-compatible/HTTP/Ollama/Anthropic P0 provider 行与本实现/测试一致；Codex 非交互环境无 `/dev/tty`，manual key 以人工审查记录留证。
- **剩余风险 / 未做项**：当前 provider registry 固定 request/response snapshot 与本地 mock server 调用契约；upstream promptfoo 0.121.13 golden diff、长尾 provider 子矩阵和真实 provider latency/streaming 差异需在 compatibility harness task 中分类。
- **下游 task 影响**：task-4.2 assertion engine 可消费 provider response output；task-4.3 custom provider/assertion contract 可复用 provider registry 的 request normalization 与 mock transport 边界。
