# Task 13.1: command-flag-parity

**Status**: Ready
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

- [ ] **AC1** (PRD §Compatibility Matrix): every command/flag inventory item maps to implemented, unsupported, later, or blocked status.
- [ ] **AC2** (ADR-004): no user-visible command returns empty success when behavior is unimplemented.
- [ ] **AC3** (PRD §User Flow): CLI help, invalid flag, stdout/stderr, and exit code snapshots are captured for P0 commands.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-13.1.1 | TEST-13.1.1 | tests/command_flag_parity.rs | install, typecheck, unit-test, manual | Not Started |
| AC2 | SCEN-13.1.1 | TEST-13.1.2 | tests/command_flag_parity.rs | install, typecheck, unit-test, manual | Not Started |
| AC3 | SCEN-13.1.1 | TEST-13.1.3 | tests/command_flag_parity.rs | install, typecheck, unit-test, manual | Not Started |

## 8. Risks

- Large command surface may require subcommands and aliases; keep parser table data-driven from inventory.
- Unsupported cloud/auth commands need careful wording to avoid implying SaaS availability.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Manual**: compare CLI surface report with upstream command inventory.

## 10. Completion Notes

- **完成日期**：<TBD-after-impl>
- **改动文件**：<TBD-after-impl>
- **commit 列表**：<TBD-after-impl>
- **§9 Verification 结果**：<TBD-after-impl>
- **剩余风险 / 未做项**：<TBD-after-impl>
- **下游 task 影响**：<TBD-after-impl>
