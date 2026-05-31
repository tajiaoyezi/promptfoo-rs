# Task 16.1: cli-command-behavior-closure

**Status**: Ready
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 16 — parity-proof-hardening
**Dependencies**: task-13.1-command-flag-parity, task-13.2-eval-output-cache-parity, task-10.1-web-viewer

## 1. Background

复审发现 `src/cli.rs` 中 `view`、`cache`、`import`、`export` 仍返回 “not yet implemented”，`CliSurface::current()` 仍把这些 P0 command rows 标为 `later`。task-13.1 已将空成功改为 explicit later，但 PRD §Core Capabilities 要求 `view`、`cache`、`import/export` 进入 1.0 兼容闭环。依据 PRD §Core Capabilities / §Compatibility Matrix、ADR-004、task-13.1 §10、task-13.2 §10。

## 2. Goal

将 `view`、`cache`、`import`、`export` 从 explicit later 推进到本地可执行、可测试、可审计的兼容行为，并更新 CLI surface 状态。

## 3. Scope

### In Scope

- `src/cli.rs`
- `src/viewer_server.rs`
- `src/cache/resume.rs`
- `tests/cli_command_behavior_closure.rs`
- `tests/command_flag_parity.rs`
- `scripts/release/e2e.sh`
- `docs/compatibility/matrix.md`

### Out Of Scope

- 不实现 promptfoo cloud/share 上传或远程同步。
- 不承诺 Web viewer 像素级复刻；`view` 的 CLI 行为先输出 stable local viewer JSON contract。
- 不改变 eval runner/provider/assertion 行为；本 task 只处理 CLI command closure。

## 4. Users / Actors

- AI 应用开发者：希望 `view/cache/import/export` 不再是未实现占位。
- DevOps / CI 维护者：需要这些命令有稳定 stdout/stderr/exit code。
- Release maintainer：需要 P0 CLI rows 不再依赖 `later` 分类通过 release gate。

## 5. Behavior Contract

`promptfoo-rs view [path]` 读取 JSONL/SQLite result source 或目录中的默认结果文件，输出 `promptfoo-rs.viewer.cli.v1` JSON table summary；`promptfoo-rs cache` 读取/清理 local resume cache 并输出 `promptfoo-rs.cache.cli.v1` JSON；`promptfoo-rs import <file>` 汇总本地结果 artifact；`promptfoo-rs export --input <file> --output <file>` 将本地 viewer table 导出为 JSON/CSV。所有命令必须对缺参、缺文件、坏 JSONL 返回非 0 与明确 stderr，不再出现 “not yet implemented”。

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/s2v-adapter.md
- docs/specs/tasks/task-10.1-web-viewer.md
- docs/specs/tasks/task-13.1-command-flag-parity.md
- docs/specs/tasks/task-13.2-eval-output-cache-parity.md
- docs/decisions/adr-004-cli-output-schema-protocol.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`std::fs`、`std::path::PathBuf`、`serde_json`、内部模块 `cli`、`cache::resume::ResumeStore`、`viewer_server::{load_viewer_records, build_results_table, export_viewer_records}`。
- Tooling commands：adapter §Commands Install / Typecheck / Unit Test / E2E tests / Build。

### 5.3 函数签名

- `handle_view_command(args: ViewArgs) -> Result<ExitCode, CliError>`
- `handle_cache_command(args: CacheArgs) -> Result<ExitCode, CliError>`
- `handle_import_command(args: ImportArgs) -> Result<ExitCode, CliError>`
- `handle_export_command(args: ExportArgs) -> Result<ExitCode, CliError>`
- `resolve_viewer_source(path: Option<PathBuf>) -> Result<ResultSource, CliError>`

## 6. Acceptance Criteria

- [ ] **AC1** (PRD §Core Capabilities / task-10.1): `view [path]` reads JSONL/SQLite result artifacts or a result directory and prints stable viewer JSON with record counts and rows.
- [ ] **AC2** (PRD §Core Capabilities / task-13.2): `cache --path <file>` reports completed/corrupt/remaining cache state and `cache --clear` removes local cache state without uploading data.
- [ ] **AC3** (PRD §Core Capabilities / ADR-004): `import <file>` and `export --input <file> --output <file>` provide local artifact conversion/summarization with stable exit codes and no empty success.
- [ ] **AC4** (PRD §Compatibility Matrix / ADR-009): CLI surface status for `command:view-directory`, `command:cache`, `command:import-file`, `command:export`, `flag:output`, and `flag:max-concurrency` is implemented/native rather than `later`.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-16.1.1 | TEST-16.1.1 | tests/cli_command_behavior_closure.rs | install, typecheck, unit-test, e2e, build | Not Started |
| AC2 | SCEN-16.1.1 | TEST-16.1.2 | tests/cli_command_behavior_closure.rs | install, typecheck, unit-test, e2e, build | Not Started |
| AC3 | SCEN-16.1.1 | TEST-16.1.3 | tests/cli_command_behavior_closure.rs | install, typecheck, unit-test, e2e, build | Not Started |
| AC4 | SCEN-16.1.1 | TEST-16.1.4 | tests/command_flag_parity.rs | install, typecheck, unit-test, e2e, build | Not Started |

## 8. Risks

- Upstream import/export semantics include cloud-adjacent paths; this task intentionally keeps behavior local-first and documents the boundary.
- `view` without a browser launch may be less convenient than upstream; stable JSON contract keeps it testable while web packaging remains Phase 10/15 scope.
- Cache file schemas vary; malformed entries must be surfaced as corrupt records rather than causing silent success.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **E2E tests**: adapter §Commands E2E tests
- **Build**: adapter §Commands Build

## 10. Completion Notes

- **完成日期**：<TBD-after-impl>
- **改动文件**：<TBD-after-impl>
- **commit 列表**：<TBD-after-impl>
- **§9 Verification 结果**：<TBD-after-impl>
- **剩余风险 / 未做项**：<TBD-after-impl>
- **下游 task 影响**：<TBD-after-impl>
