# Task 7.1: redteam-command-flow

> ✅ **Status: Done** — task-7.1 已按 RED→GREEN→§9 verification 完成，详见 §10 Completion Notes。

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 7 — redteam-core
**Dependencies**: Phase 7 AUTO order; see adapter Task 总索引

## 1. Background

PRD 要求 promptfoo-rs 在 promptfoo 0.121.13 baseline 下建立 Rust-native core、兼容矩阵和 golden diff release gate。本 task 负责 redteam-engine 模块中的 redteam-command-flow 工作。

## 2. Goal

实现 redteam init/generate/eval/run/report 最小兼容闭环。

## 3. Scope

### In Scope

- redteam-engine 模块中与 redteam-command-flow 直接相关的源码、测试、fixture 和文档。
- 与本 task AC 对应的 compatibility matrix 或 release gate 记录。
- 具体文件清单：src/redteam/mod.rs、src/redteam/config.rs、src/redteam/flow.rs、src/cli.rs、tests/redteam_command_flow.rs、test/fixtures/redteam-engine/。依据 PRD §Technical Approach 的 `redteam-engine` 边界。

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
- docs/specs/phases/phase-7-redteam-core.md
- test/features/redteam-engine.feature
- docs/decisions/adr-001-rust-core-with-optional-bridges.md
- docs/decisions/adr-006-compatibility-first-test-toolchain.md

### 5.2 Imports

- Rust crate / module：`serde`、`serde_yaml`、`tokio`、内部模块 `redteam`、`config`、`output`。依据 PRD §Core Capabilities / ADR-006。

### 5.3 函数签名

- `load_redteam_config(path: &Path) -> Result<RedteamConfig, RedteamError>`
- `run_redteam_flow(config: RedteamConfig, target: MockTarget) -> Result<RedteamReport, RedteamError>`
- `handle_redteam_command(args: RedteamArgs) -> Result<ExitCode, CliError>`
- 本 task 固定 init/generate/eval/run/report skeleton 与失败报告路径；依据 PRD §User Flow。

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Implementation Phases / §Compatibility Matrix): redteam.yaml 能被加载并驱动 init/generate/eval/run/report skeleton
- [x] **AC2** (PRD §Implementation Phases / §Compatibility Matrix): 核心流程可在 mock target 下生成风险结果
- [x] **AC3** (PRD §Implementation Phases / §Compatibility Matrix): 失败路径输出可定位 report 错误

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-7.1.1 | TEST-7.1.1 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC2 | SCEN-7.1.2 | TEST-7.1.2 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC3 | SCEN-7.1.3 | TEST-7.1.3 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |

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
  - src/redteam/mod.rs
  - src/redteam/config.rs
  - src/redteam/flow.rs
  - tests/redteam_command_flow.rs
  - docs/specs/tasks/task-7.1-redteam-command-flow.md
  - docs/specs/phases/phase-7-redteam-core.md
  - docs/s2v-adapter.md
  - docs/compatibility/matrix.md
- **commit 列表**：
  - 878e473 test(redteam): add task-7.1 command flow RED tests
  - 0d168b9 feat(redteam): add command flow skeleton
  - 本 docs(spec) 回填提交见 git log：docs(spec): 回填 task-7.1 §10 Completion Notes + Status → Done
- **§9 Verification 结果**：
  - install: passed — `cargo fetch` 通过；viewer/npm install 按 adapter N/A 跳过。
  - typecheck: passed — `cargo check --workspace` 通过。
  - unit-test: passed — `cargo test --workspace` 通过，TEST-7.1.1~TEST-7.1.3 加入后 45 个 integration tests 全部通过。
  - manual: passed — 已核对 AC1/AC2/AC3、SCEN-7.1.1~7.1.3、TEST-7.1.1~TEST-7.1.3、compatibility matrix 的 redteam.yaml 行与本实现/测试一致；Codex 非交互环境无 `/dev/tty`，manual key 以人工审查记录留证。
- **剩余风险 / 未做项**：本 task 固定 redteam config load、CLI command skeleton、mock target flow 和 located report write failures；核心 plugin/strategy registry、risk summary 和 harness-ready report contract 留给 task 7.2。
- **下游 task 影响**：task 7.2 可复用 `RedteamConfig`、`RedteamReport`、`RedteamFinding` 和 `write_redteam_report` 作为 registry/risk/report 输出基础。
