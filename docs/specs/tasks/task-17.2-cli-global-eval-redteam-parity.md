# Task 17.2: cli-global-eval-redteam-parity

**Status**: Ready
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 17 — deep-upstream-parity-proof
**Dependencies**: task-17.1-frozen-source-inventory-extractor, task-16.1-cli-command-behavior-closure

## 1. Background

当前审计显示本地 top-level CLI、`eval --help`、`redteam --help` 仍明显小于 `promptfoo@0.121.13`。Phase 13/16 已消除空成功占位并实现核心本地命令，但尚未覆盖 upstream help surface 的完整 command/flag parity。依据 docs/audits/promptfoo-current-perfect-refactor-audit-2026-05-31.md §P0 Local CLI surface、PRD §Core Capabilities、ADR-004。

## 2. Goal

扩展 CLI inventory、help snapshot、parser 和 runtime behavior，使 upstream top-level commands、eval flags、redteam subcommands 全部实现兼容行为，或以非 0 explicit unsupported/later/blocked 错误和 matrix reason 呈现。

## 3. Scope

### In Scope

- `src/cli.rs`
- `src/config/`
- `src/eval/`
- `src/redteam/`
- `src/output/`
- `src/compatibility/matrix.rs`
- `compatibility/fixtures/cli/`
- `compatibility/fixtures/eval/`
- `compatibility/fixtures/redteam/`
- `tests/cli_global_eval_redteam_parity.rs`
- `scripts/release/e2e.sh`

### Out Of Scope

- 不实现 promptfoo cloud/share SaaS；`share` / auth-cloud paths 必须按 PRD Out of Scope 返回本地 unsupported/no-upload 行为。
- 不要求真实 LLM/provider network calls；provider 行为使用 mock/recorded fixtures。
- 不处理 provider/assertion/redteam 长尾 registry 分类；task 17.4 负责。

## 4. Users / Actors

- Existing promptfoo CLI user：希望现有脚本的 command/flag 不会静默失效。
- CI maintainer：需要 stdout/stderr/exit code 对 unsupported/later 与真实失败可区分。
- Release maintainer：需要 CLI surface coverage 进入 release gate summary。

## 5. Behavior Contract

`promptfoo-rs --help`、`promptfoo-rs eval --help`、`promptfoo-rs redteam --help` 必须与 frozen upstream help snapshot 建立 diffable mapping。可本地实现的命令/flags 需要真实影响 config/eval/redteam/output/cache 行为；不可实现或 out-of-scope 的命令/flags 必须返回非 0、明确 command/flag、classification、reason、matrix item id，并禁止上传数据。

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/compatibility/matrix.md
- docs/specs/tasks/task-13.1-command-flag-parity.md
- docs/specs/tasks/task-13.2-eval-output-cache-parity.md
- docs/specs/tasks/task-16.1-cli-command-behavior-closure.md
- docs/specs/tasks/task-17.1-frozen-source-inventory-extractor.md
- docs/decisions/adr-004-cli-output-schema-protocol.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`clap::{Args, Subcommand, ArgAction, ValueEnum}`、`std::path::PathBuf`、`serde_json`、内部模块 `cli`、`config`、`eval`、`redteam`、`compatibility::matrix`。
- Tooling commands：adapter §Commands Install / Typecheck / Unit Test / E2E tests / Coverage / Build。

### 5.3 函数签名

- `CliSurface::from_upstream_help_snapshot(snapshot: &UpstreamHelpSnapshot) -> CliSurface`
- `classify_cli_item(item: &CliInventoryItem, matrix: &CapabilityMatrix) -> CliCompatibilityStatus`
- `handle_explicit_gap_command(command: &str, item_id: &str, class: GapClass, reason: &str) -> CliError`
- `apply_eval_flag_overrides(args: &EvalArgs, config: PromptfooConfig) -> Result<PromptfooConfig, CliError>`
- `handle_redteam_subcommand(args: RedteamCommand) -> Result<ExitCode, CliError>`

## 6. Acceptance Criteria

- [ ] **AC1** (ADR-004): top-level commands from `promptfoo@0.121.13 --help` are represented in local CLI surface as implemented, unsupported, later, or blocked; missing command rows fail E2E.
- [ ] **AC2** (PRD §User Flow): upstream `eval --help` P0 flags for config/prompts/providers/tests/vars/output/concurrency/repeat/delay/cache/resume/retry/filter/env-file/no-write are parsed and either affect eval behavior or return explicit classified errors.
- [ ] **AC3** (PRD §Redteam / Core Capabilities): upstream redteam subcommands `init`, `eval`, `generate`, `run`, `report`, `plugins`, plus unsupported `discover`/`poison`/`setup` paths, have stable help, stdout/stderr, exit code, and matrix evidence.
- [ ] **AC4** (PRD §Security / Out of Scope): cloud/share/auth commands do not upload data by default and return user-visible no-upload / unsupported classification with matrix item id.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-17.2.1 | TEST-17.2.1 | tests/cli_global_eval_redteam_parity.rs | install, typecheck, unit-test, e2e, coverage, build | Spec Ready |
| AC2 | SCEN-17.2.1 | TEST-17.2.2 | tests/cli_global_eval_redteam_parity.rs | install, typecheck, unit-test, e2e, coverage, build | Spec Ready |
| AC3 | SCEN-17.2.1 | TEST-17.2.3 | tests/cli_global_eval_redteam_parity.rs | install, typecheck, unit-test, e2e, coverage, build | Spec Ready |
| AC4 | SCEN-17.2.1 | TEST-17.2.4 | tests/cli_global_eval_redteam_parity.rs | install, typecheck, unit-test, e2e, coverage, build | Spec Ready |

## 8. Risks

- Some upstream commands are cloud-account workflows; implementing fake success would be worse than explicit unsupported/no-upload errors.
- Upstream flags with overlapping aliases can make clap parsing ambiguous; tests must cover help output and representative runtime behavior.
- Expanding eval flags can alter existing successful fixtures; keep compatibility fixtures deterministic and classify intentional differences.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **E2E tests**: adapter §Commands E2E tests
- **Coverage**: adapter §Commands Coverage
- **Build**: adapter §Commands Build

## 10. Completion Notes

- **完成日期**：待实施后回填
- **改动文件**：待实施后回填
- **commit 列表**：待实施后回填
- **§9 Verification 结果**：待实施后回填
- **剩余风险 / 未做项**：待实施后回填
- **下游 task 影响**：待实施后回填
