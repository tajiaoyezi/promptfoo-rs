# Task 2.3: eval-command-smoke

> ✅ **Status: Done** — task-2.3 已按 RED→GREEN→§9 verification 完成，详见 §10 Completion Notes。

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 2 — config-cli-core
**Dependencies**: Phase 2 AUTO order; see adapter Task 总索引

## 1. Background

PRD 要求 promptfoo-rs 在 promptfoo 0.121.13 baseline 下建立 Rust-native core、兼容矩阵和 golden diff release gate。本 task 负责 eval-runner 模块中的 eval-command-smoke 工作。

## 2. Goal

让 eval -c 进入 runner 并在 mock provider 下产生最小结果。

## 3. Scope

### In Scope

- eval-runner 模块中与 eval-command-smoke 直接相关的源码、测试、fixture 和文档。
- 与本 task AC 对应的 compatibility matrix 或 release gate 记录。
- 具体文件清单：src/eval/mod.rs、src/eval/result.rs、src/cli.rs、tests/eval_command_smoke.rs、test/fixtures/eval-runner/。依据 PRD §Technical Approach 的 `eval-runner` 与 `cli` 边界。

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
- test/features/eval-runner.feature
- docs/decisions/adr-001-rust-core-with-optional-bridges.md
- docs/decisions/adr-006-compatibility-first-test-toolchain.md

### 5.2 Imports

- Rust crate / module：`tokio`、`serde`、内部模块 `config`、`eval`、`cli`；fixture helper 使用 test/fixtures/eval-runner/。依据 PRD §User Flow / ADR-004 / ADR-006。

### 5.3 函数签名

- `run_eval(config: NormalizedConfig, options: EvalOptions) -> Result<EvalResultEnvelope, EvalError>`
- `EvalResultEnvelope { status, summary, results, errors, metadata }`
- `handle_eval_command(args: EvalArgs) -> Result<ExitCode, CliError>`
- 本 task 只做空/最小 eval smoke 与结构化错误，不实现完整 scheduler/provider；依据 Phase 2 scope。

## 6. Acceptance Criteria

- [ ] **AC1** (PRD §Implementation Phases / §Compatibility Matrix): eval -c promptfooconfig.yaml 能完成空/最小 eval smoke
- [ ] **AC2** (PRD §Implementation Phases / §Compatibility Matrix): runner 输出结构化 result envelope
- [ ] **AC3** (PRD §Implementation Phases / §Compatibility Matrix): 失败配置返回可定位错误和非零 exit code

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-2.3.1 | TEST-2.3.1 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC2 | SCEN-2.3.2 | TEST-2.3.2 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |
| AC3 | SCEN-2.3.3 | TEST-2.3.3 | N/A until integration harness exists | install, typecheck, unit-test, manual | Done |

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
  - src/config/mod.rs
  - src/eval/mod.rs
  - tests/eval_command_smoke.rs
  - docs/specs/tasks/task-2.3-eval-command-smoke.md
- **commit 列表**：
  - 721384d test(eval-runner): add task-2.3 eval smoke RED tests
  - 19c1564 feat(eval-runner): add minimal eval command smoke
  - 本 docs(spec) 回填提交见 git log：docs(spec): 回填 task-2.3 §10 Completion Notes + Status → Done
- **§9 Verification 结果**：
  - install: 通过；`s2v_verify_full` 自动项执行 `cargo fetch` 成功（viewer/npm package.json 不存在，按 adapter 条件跳过）。
  - typecheck: 通过；`cargo check --workspace` 成功。
  - unit-test: 通过；`cargo test --workspace` 成功，TEST-1.1.*、TEST-1.2.*、TEST-2.1.*、TEST-2.2.*、TEST-2.3.* 共 15 passed / 0 failed。
  - manual: 通过；已核对 AC、BDD SCEN-2.3.1~2.3.3、TEST-2.3.1~2.3.3 与 tests/eval_command_smoke.rs；直接运行缺失 config 的 `cargo run -- eval -c tests\fixtures\missing.yaml` 返回 exit code 1 且 stderr 包含 config path。注：当前非交互 shell 无 `/dev/tty`，`s2v_run manual` 无法读取确认输入，人工核验结果记录于本条。
- **剩余风险 / 未做项**：runner 仅实现 Phase 2 最小 smoke；并发、retry、cache/resume、real provider 调用留给 Phase 3/4。
- **下游 task 影响**：task-3.1 可扩展 `run_eval` 为 scheduler runtime；task-5.x 可替换当前 stdout JSON 为 output-writers contract。
