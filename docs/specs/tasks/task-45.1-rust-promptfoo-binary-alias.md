# Task 45.1: rust-promptfoo-binary-alias

**Status**: Ready
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 45 - promptfoo-drop-in-cli-entrypoints
**Dependencies**: task-2.1-workspace-cli-skeleton, task-2.3-eval-command-smoke

## 1. Background

Upstream promptfoo README Quick Start uses `promptfoo init --example getting-started`, `promptfoo eval`, and `promptfoo view`; `npm view promptfoo bin` reports `promptfoo` and `pf` bin entries. The current Rust package builds `promptfoo-rs.exe` only, so users cannot run the upstream command spelling locally. 依据 upstream README observed 2026-06-04, `npm view promptfoo bin`, PRD §Compatibility Matrix, task 2.1, and task 2.3.

## 2. Goal

Add a Rust binary target named `promptfoo` that routes to the same CLI implementation as `promptfoo-rs`, while preserving `promptfoo-rs` and making help/error UX reflect the invoked binary where applicable.

## 3. Scope

### In Scope

- `Cargo.toml` binary target declarations.
- `src/main.rs` and/or `src/bin/promptfoo.rs` entrypoint wiring.
- `src/cli.rs` command name handling if needed.
- `tests/promptfoo_cli_alias.rs`.
- Release build/runtime-smoke wiring if it hard-codes the binary name.
- `test/features/cli.feature` traceability.

### Out Of Scope

- Publishing a package named `promptfoo`.
- Removing or renaming `promptfoo-rs`.
- Implementing additional promptfoo subcommands beyond existing command surface.
- Resolving cloud/share/auth external authority blockers.

## 4. Users / Actors

- Existing promptfoo user: wants commands copied from upstream docs to work locally.
- CI maintainer: wants `promptfoo eval` to be the stable command in scripts.
- Existing promptfoo-rs user: needs `promptfoo-rs` to remain available.

## 5. Behavior Contract

`cargo build --workspace --release` must build both `promptfoo` and `promptfoo-rs` binaries. Both entrypoints call the same CLI dispatcher and support the same subcommands, exit codes, stdout/stderr contracts, and config parsing. The `promptfoo` binary should display `Usage: promptfoo [COMMAND]` or equivalent command-name-appropriate help. `promptfoo-rs` remains available for users who want an explicit reimplementation command name.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/s2v-adapter.md
- docs/specs/tasks/task-2.1-workspace-cli-skeleton.md
- docs/specs/tasks/task-2.3-eval-command-smoke.md
- docs/decisions/adr-004-cli-output-schema-protocol.md
- test/features/cli.feature
- tests/cli_skeleton.rs
- tests/eval_command_smoke.rs

### 5.2 Imports

- Rust crate / module: `promptfoo_rs::cli`, `std::process::Command`, `std::fs`, `std::path::PathBuf`.
- Cargo environment: `CARGO_BIN_EXE_promptfoo`, `CARGO_BIN_EXE_promptfoo-rs`.
- Shell/tooling commands: adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke.

### 5.3 函数签名

- `promptfoo_rs::cli::main() -> std::process::ExitCode`
- Test helper contract: `fn promptfoo_bin(name: &str) -> Command`
- Test helper contract: `fn write_minimal_promptfoo_config(dir: &Path) -> PathBuf`

## 6. Acceptance Criteria

- [ ] **AC1** (upstream README): release build emits a `promptfoo` binary that runs `--help` successfully and presents the `promptfoo` command spelling.
- [ ] **AC2** (task 2.3): `promptfoo eval -c promptfooconfig.yaml` passes the same minimal eval smoke as `promptfoo-rs eval -c promptfooconfig.yaml`.
- [ ] **AC3** (ADR-004): `promptfoo` and `promptfoo-rs` preserve equivalent stdout/stderr/exit-code behavior for valid eval, invalid config, and unknown command cases.
- [ ] **AC4** (task 2.1): existing `promptfoo-rs` binary remains built, tested, and documented as a non-conflicting explicit alias.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-45.1.1 | TEST-45.1.1 | tests/promptfoo_cli_alias.rs | install, lint, typecheck, unit-test, build | Not Started |
| AC2 | SCEN-45.1.1 | TEST-45.1.2 | tests/promptfoo_cli_alias.rs | install, typecheck, unit-test, integration, build | Not Started |
| AC3 | SCEN-45.1.1 | TEST-45.1.3 | tests/promptfoo_cli_alias.rs | install, lint, typecheck, unit-test, e2e, build | Not Started |
| AC4 | SCEN-45.1.1 | TEST-45.1.4 | tests/promptfoo_cli_alias.rs | install, lint, typecheck, unit-test, runtime-smoke, build | Not Started |

## 8. Risks

- Hard-coded clap command name can make `promptfoo --help` still display `promptfoo-rs`; tests must catch this.
- Duplicate bin targets can accidentally stop building the existing package default; AC4 prevents a breaking rename.
- Windows `.exe` naming needs explicit test expectations.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Lint**: adapter §Commands Lint
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Integration tests**: adapter §Commands Integration tests
- **E2E tests**: adapter §Commands E2E tests
- **Coverage**: adapter §Commands Coverage
- **Build**: adapter §Commands Build
- **Runtime smoke**: adapter §Commands Runtime smoke

## 10. Completion Notes

- **完成日期**：<TBD-after-impl>
- **改动文件**：<TBD-after-impl>
- **commit 列表**：<TBD-after-impl>
- **§9 Verification 结果**：
  - install: <TBD-after-impl>
  - lint: <TBD-after-impl>
  - typecheck: <TBD-after-impl>
  - unit-test: <TBD-after-impl>
  - integration: <TBD-after-impl>
  - e2e: <TBD-after-impl>
  - coverage: <TBD-after-impl>
  - build: <TBD-after-impl>
  - runtime-smoke: <TBD-after-impl>
- **剩余风险 / 未做项**：<TBD-after-impl>
- **下游 task 影响**：<TBD-after-impl>
