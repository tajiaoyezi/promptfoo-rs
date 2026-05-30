# Task 13.1: command-flag-parity

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 13 — cli-output-eval-parity
**Dependencies**: task-12.3-golden-diff-ci-release-gate

## 1. Background

Audit showed local CLI exposes 10 variants while upstream has a much larger command surface; several local commands return empty success. This task closes command/flag inventory parity. Basis: PRD §Core Capabilities / §Compatibility Matrix, ADR-004.

## 2. Goal

Implement or explicitly classify every upstream command/flag item from the expanded matrix; remove empty success placeholders for user-visible commands.

## 3. Scope

### In Scope

- src/cli.rs
- src/commands/ or equivalent module split
- tests/command_flag_parity.rs
- compatibility/fixtures/cli/
- docs/compatibility/matrix.md

### Out Of Scope

- Deep provider/assertion behavior beyond command routing; Phase 14 owns capability internals.
- Cloud/share upload behavior remains P2 no-upload unless future ADR changes it.

## 4. Users / Actors

- AI application developer: invokes promptfoo-compatible CLI.
- DevOps / CI maintainer: depends on stdout/stderr/exit code and flags.

## 5. Behavior Contract

Every upstream command path must resolve to compatible behavior, explicit unsupported/later error, or blocked matrix entry. Empty success for unimplemented commands is forbidden.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/audits/promptfoo-upstream-inventory-gap-2026-05-30.md
- docs/specs/tasks/task-11.2-item-level-capability-inventory.md
- docs/decisions/adr-004-cli-output-schema-protocol.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`clap`、`serde_json`、内部模块 `cli`、`compatibility::inventory`。

### 5.3 函数签名

- `CommandInventory::from_matrix(matrix: &CapabilityMatrix) -> CommandInventory`
- `validate_cli_surface(cli: &CliSurface, inventory: &CommandInventory) -> CliParityReport`
- `unsupported_command_error(command: &str, reason: &str) -> CliError`

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Compatibility Matrix): every command/flag inventory item maps to implemented, unsupported, later, or blocked status.
- [x] **AC2** (ADR-004): no user-visible command returns empty success when behavior is unimplemented.
- [x] **AC3** (PRD §User Flow): CLI help, invalid flag, stdout/stderr, and exit code snapshots are captured for P0 commands.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-13.1.1 | TEST-13.1.1 | tests/command_flag_parity.rs | install, typecheck, unit-test, manual | Done |
| AC2 | SCEN-13.1.1 | TEST-13.1.2 | tests/command_flag_parity.rs | install, typecheck, unit-test, manual | Done |
| AC3 | SCEN-13.1.1 | TEST-13.1.3 | tests/command_flag_parity.rs | install, typecheck, unit-test, manual | Done |

## 8. Risks

- Large command surface may require subcommands and aliases; keep parser table data-driven from inventory.
- Unsupported cloud/auth commands need careful wording to avoid implying SaaS availability.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Manual**: compare CLI surface report with upstream command inventory.

## 10. Completion Notes

- **完成日期**：2026-05-30
- **改动文件**：
  - `src/cli.rs`
  - `tests/command_flag_parity.rs`
  - `docs/compatibility/matrix.md`
  - `docs/specs/tasks/task-13.1-command-flag-parity.md`
- **commit 列表**：
  - `85ba826` `docs(spec): task-13.1 进入实施 (Status: Ready → In Progress)`
  - `7b45c4e` `test(cli): 加 SCEN-13.1.1 的 3 个 RED 测试`
  - `58c04f8` `feat(cli): 映射 command flag parity 并移除空成功占位`
- **§9 Verification 结果**：
  - install: PASS — `s2v_verify_full "install typecheck unit-test"` 中 `cargo fetch` 通过。
  - typecheck: PASS — `cargo check --workspace` 通过。
  - unit-test: PASS — `cargo test --workspace` 通过，包含 `tests/command_flag_parity.rs` 的 TEST-13.1.1~TEST-13.1.3。
  - manual: PASS — 上游 command/flag inventory 中 12 项全部映射：`command:eval`, `command:view-directory`, `command:cache`, `command:redteam`, `command:mcp`, `command:code-scans`, `command:scan-model`, `command:import-file`, `command:export`, `flag:config`, `flag:output`, `flag:max-concurrency`；`view/cache/import/export` 均为 exit=1 且 stderr 非空，不再空成功。非交互 helper 的 full run 仅因 `/dev/tty` manual 确认失败，机械 keys 已单独全绿。
- **剩余风险 / 未做项**：`--output`、`--max-concurrency`、cache/resume/retry/output 文件写入在本 task 仅进入 CLI 解析和 explicit later 分类；实际行为由 task 13.2 负责。
- **下游 task 影响**：task 13.2 可在已解析的 `EvalArgs.output` / `EvalArgs.max_concurrency` 基础上实现 eval/output/cache 行为，不需要再处理 no-op 命令占位问题。
