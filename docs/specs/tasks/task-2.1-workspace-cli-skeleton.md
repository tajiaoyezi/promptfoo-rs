# Task 2.1: workspace-cli-skeleton

> ✅ **Status: Done** — task-2.1 已按 RED→GREEN→§9 verification 完成，详见 §10 Completion Notes。

**Status**: Done
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
- 具体文件清单：Cargo.toml、src/main.rs、src/lib.rs、src/cli.rs、tests/cli_skeleton.rs。依据 PRD §Technical Approach 的 `cli` 模块边界与 ADR-004。

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
- docs/specs/phases/phase-2-config-cli-core.md
- test/features/cli.feature
- docs/decisions/adr-001-rust-core-with-optional-bridges.md
- docs/decisions/adr-004-cli-output-schema-protocol.md
- docs/decisions/adr-006-compatibility-first-test-toolchain.md

### 5.2 Imports

- Rust crate / module：`clap`、`anyhow`、内部模块 `cli`；dev-dependencies 使用 `assert_cmd` 与 `predicates` 验证 CLI 协议。依据 ADR-002 / ADR-004 / ADR-006。

### 5.3 函数签名

- `Cli::parse_from(args: impl IntoIterator<Item = impl Into<OsString> + Clone>) -> Cli`
- `run_cli(cli: Cli) -> Result<ExitCode, CliError>`
- `enum Command { Eval, View, Cache, Redteam, Mcp, CodeScans, ScanModel, Import, Export }`
- CLI skeleton 只固定 command/flag/exit-code 外壳，不实现 eval 业务；依据 PRD §Core Capabilities / ADR-004。

## 6. Acceptance Criteria

- [ ] **AC1** (PRD §Implementation Phases / §Compatibility Matrix): cargo workspace 能承载 core/cli 代码并通过 cargo check
- [ ] **AC2** (PRD §Implementation Phases / §Compatibility Matrix): CLI 暴露 eval/view/cache/redteam/mcp/code-scans/scan-model/import/export skeleton
- [ ] **AC3** (PRD §Implementation Phases / §Compatibility Matrix): 未知命令和无效 flag 按稳定 stderr/exit code 返回

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-2.1.1 | TEST-2.1.1 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC2 | SCEN-2.1.2 | TEST-2.1.2 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC3 | SCEN-2.1.3 | TEST-2.1.3 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |

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
  - src/main.rs
  - src/cli.rs
  - tests/cli_skeleton.rs
  - docs/specs/tasks/task-2.1-workspace-cli-skeleton.md
- **commit 列表**：
  - 0fe1a0e test(cli): add task-2.1 CLI skeleton RED tests
  - 4504354 feat(cli): add promptfoo-compatible CLI skeleton
  - 本 docs(spec) 回填提交见 git log：docs(spec): 回填 task-2.1 §10 Completion Notes + Status → Done
- **§9 Verification 结果**：
  - install: 通过；`s2v_verify_full` 自动项执行 `cargo fetch` 成功（viewer/npm package.json 不存在，按 adapter 条件跳过）。
  - typecheck: 通过；`cargo check --workspace` 成功。
  - unit-test: 通过；`cargo test --workspace` 成功，TEST-1.1.*、TEST-1.2.*、TEST-2.1.* 共 9 passed / 0 failed。
  - manual: 通过；`cargo run -- --help` 显示 eval/view/cache/redteam/mcp/code-scans/scan-model/import/export skeleton；已核对 AC、BDD SCEN-2.1.1~2.1.3、TEST-2.1.1~2.1.3。注：当前非交互 shell 无 `/dev/tty`，`s2v_run manual` 无法读取确认输入，人工核验结果记录于本条。
- **剩余风险 / 未做项**：eval command 仅固定 CLI skeleton，配置加载与 runner 行为留给 task-2.2/task-2.3。
- **下游 task 影响**：task-2.2 可接入 `EvalArgs.config`；task-2.3 可复用 `handle_eval_command` 入口扩展真实 eval smoke。
