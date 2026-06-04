# Phase 45: promptfoo-drop-in-cli-entrypoints

**Status**: Ready
**Owner**: leafiellune
**Related PRD**: ../../prds/promptfoo-rs.prd.md

## 1. Phase Goal

Provide drop-in local CLI entrypoints matching upstream promptfoo README usage: `promptfoo init`, `promptfoo eval`, `promptfoo view`, and package bin aliases `promptfoo` / `pf`, while preserving the existing `promptfoo-rs` binary and fail-closed cloud/publication boundaries. 依据 upstream `promptfoo/promptfoo` README Quick Start observed on 2026-06-04, `npm view promptfoo bin`, PRD §Compatibility Matrix / §Release constraints, ADR-004, ADR-008, ADR-011, and task 2.1 / task 9.2 / task 10.2.

## 2. Business Value

Existing promptfoo users copy commands from upstream docs and expect `promptfoo eval` to work. A Rust-compatible implementation that only exposes `promptfoo-rs eval` is usable but not migration-friendly. This phase turns command-name parity into tested behavior without claiming ownership of the upstream npm package name or public publication channels.

## 3. Scope / Modules

`Cargo.toml`, `src/main.rs`, `src/bin/`, `src/cli.rs`, `tests/promptfoo_cli_alias.rs`, `npm/package.json`, `npm/src/`, `npm/scripts/`, `tests/npm_promptfoo_bin_shims.rs` or npm smoke scripts, `README.md`, `README.en.md`, `docs/QUICKSTART.md`, `docs/QUICKSTART.en.md`, `docs/release.md`, `docs/PROJECT-OVERVIEW.md`, `test/features/cli.feature`, and release-gate/runtime-smoke wiring where needed.

## 4. Task List

| Task | Name | Spec | Status | Goal |
|---|---|---|---|---|
| 45.1 | rust-promptfoo-binary-alias | ../tasks/task-45.1-rust-promptfoo-binary-alias.md | Done | 让 Cargo release 同时生成 `promptfoo` 和 `promptfoo-rs` 两个等价 Rust CLI 入口 |
| 45.2 | npm-promptfoo-bin-shims | ../tasks/task-45.2-npm-promptfoo-bin-shims.md | Done | 让 npm wrapper 暴露 `promptfoo`、`promptfoo-rs`、`pf` bin shim 并保持本地 smoke 可验证 |
| 45.3 | readme-drop-in-cli-usage | ../tasks/task-45.3-readme-drop-in-cli-usage.md | Done | 把 README/Quickstart/release docs 从 `promptfoo-rs` only 更新为 drop-in CLI 用法，并保留品牌/发布边界 |

## 5. Dependencies

Depends on task 2.1 CLI skeleton, task 2.3 eval smoke, task 9.2 Node API wrapper, task 10.2 release docs/packaging, ADR-004 CLI/output protocol, ADR-008 binary-first release, and ADR-011 current-latest claim boundaries. No real registry credential, npm package ownership transfer, legal/brand approval, or public publication is required.

## 6. Phase Acceptance Criteria

- [ ] `cargo build --workspace --release` produces a runnable `promptfoo` binary in addition to `promptfoo-rs`, and both execute the same CLI command surface.
- [ ] Running `promptfoo --help`, `promptfoo eval -c promptfooconfig.yaml`, and `promptfoo view <dir>` is covered by RED/GREEN tests and produces command-name-appropriate UX.
- [ ] The npm wrapper declares and tests local bin shims for `promptfoo`, `promptfoo-rs`, and `pf` without publishing to npm or shadowing upstream ownership claims.
- [ ] README, Quickstart, release docs, and compatibility docs explain `promptfoo` as the drop-in local command and keep publication/perfect-refactor claims fail-closed.

## 7. Phase Risks

- The `promptfoo` command name can create brand confusion; docs must say this is an independent reimplementation and not upstream endorsement.
- Cargo and npm bin naming can drift if one alias is tested and the other is not; both surfaces need tests.
- Adding `promptfoo` must not remove `promptfoo-rs`; existing users and scripts should keep working.
- `pf` is an upstream npm alias, but local binary publication may require extra packaging decisions; this phase only plans and verifies local shim behavior.

## 8. Definition of Done

Tasks 45.1, 45.2, and 45.3 are Done; phase §6 smoke passes through task §9 verification; both Rust and npm wrapper command aliases are tested; docs prefer `promptfoo` while preserving `promptfoo-rs` as an explicit alias; and the repository is clean and pushed.

## 9. Phase Completion Notes

- **完成日期**：<TBD-after-impl>
- **Phase smoke**：<TBD-after-impl>
- **Artifact evidence**：<TBD-after-impl>
- **Remaining boundaries**：Public npm package ownership, actual npm publish authority, Homebrew formula publication, legal/brand approval, and perfect-refactor claim gates remain outside this phase.
