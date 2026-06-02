# Task 27.1: current-latest-core-config-burndown

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 27 — current-latest-core-config-burndown
**Dependencies**: task-26.1-current-latest-viewer-config-reclassification, task-19.2-core-config-source-fixture-burndown

## 1. Background

Phase 26 removed duplicate `src/app/**` config blockers from the current-latest target, but `current-latest-golden-corpus.json` still reports 18 non-app `config` P0 blockers. Task 19.2 already established a conservative decision model for frozen-baseline non-app config rows: runtime promptfooconfig/env/file rows are local fixture-covered, redteam promptfooconfig rows are fixture-covered, code scan/MCP helper config rows are auxiliary P1 evidence, and cloud/server/telemetry/global rows remain external authority blockers. This task applies that model to current-latest artifacts and fixes evidence-reference generation so fixture/snapshot config rows are not still emitted as `blocker:` rows. 依据 PRD §Core Capabilities / §Compatibility Matrix、ADR-009、ADR-011、Phase 26 §10 artifact evidence、task 19.2。

## 2. Goal

Reduce current-latest config P0 golden blockers from 18 generic blockers to 7 explicit external authority blockers while keeping local runtime config fixture evidence and auxiliary config snapshot evidence visible.

## 3. Scope

### In Scope

- `src/compatibility/inventory.rs`
- `scripts/release/current-latest-source-inventory.sh`
- `tests/current_latest_core_config_burndown.rs`
- `target/release-gates/current-latest-source-inventory.json`
- `target/release-gates/current-latest-matrix.json`
- `target/release-gates/current-latest-golden-corpus.json`
- `target/release-gates/current-latest-quality.json`
- `test/features/perfect-refactor-parity.feature`

### Out Of Scope

- 不实现 promptfoo cloud/server SaaS 行为或真实 telemetry backend。
- 不提供真实账号、credentials、私有服务、法律/品牌授权或 publication authority。
- 不解决 provider、eval-runner、prompt-processing、cache-store 或 script-bridge blockers。
- 不承诺“无任何潜在 bug”；claim 仍受 task 24.4 / ADR-011 的 evidence boundary 约束。

## 4. Users / Actors

- CLI user: needs promptfooconfig/env/file config compatibility to be counted as local fixture-covered evidence.
- Compatibility maintainer: needs non-app config blockers split into native fixture, auxiliary snapshot, and external blocker decisions.
- Release reviewer: needs config blocker reductions to preserve external authority blockers and not weaken P0 semantics.

## 5. Behavior Contract

Current-latest `category=config` rows must be classified by path. Runtime config rows (`src/commands/config.ts`, `src/configTypes.ts`, `src/util/config/**`) and redteam `promptfooconfig.yaml` must use fixture evidence and no longer appear in P0 golden blockers. Auxiliary rows (`src/codeScan/config/**`, `src/commands/mcp/tools/validatePromptfooConfig.ts`) must use P1 snapshot evidence. External authority rows (`src/globalConfig/**`, `src/server/config/**`, `src/server/routes/configs.ts`, `src/tracing/otelConfig.ts`, `src/types/api/configs.ts`) must remain P0 blockers with explicit external-authority reason. Rust and shell artifact generation must use equivalent path rules, and evidence references must follow `evidence_kind` rather than hard-coded `category=config`.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/phases/phase-26-current-latest-viewer-config-reclassification.md
- docs/specs/tasks/task-19.2-core-config-source-fixture-burndown.md
- docs/specs/tasks/task-24.4-current-latest-exhaustive-quality-gate.md
- docs/specs/tasks/task-26.1-current-latest-viewer-config-reclassification.md
- docs/compatibility/matrix.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `promptfoo_rs::compatibility::inventory::{extract_current_latest_inventory, reconcile_current_latest_matrix, write_current_latest_inventory_artifacts, CurrentLatestTargetLock}`, `promptfoo_rs::compatibility::harness::{build_current_latest_golden_corpus, evaluate_current_latest_release_blockers}`, `serde_json::Value`, `std::path::Path`, `std::process::Command`.
- Tooling commands: `bash scripts/release/current-latest-source-inventory.sh`, `bash scripts/release/current-latest-golden-corpus.sh`, `bash scripts/release/current-latest-quality-gate.sh`, `s2v_verify_full "$(s2v_extract_verify_keys "$TASK_SPEC")"`, adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke.

### 5.3 函数签名

- `current_latest_default_metadata(category: &str, stable_id: &str, file: &str) -> (String, String, String, String, Option<String>)`
- `default_evidence_reference(category: &str, stable_id: &str, evidence_kind: &str) -> String`
- `is_current_latest_runtime_config_file(file: &str) -> bool`
- `is_current_latest_auxiliary_config_file(file: &str) -> bool`
- `is_current_latest_external_config_file(file: &str) -> bool`
- Shell contract: `metadata(category, id, file)`, `evidenceReference(category, id, evidenceKind)`

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Core Capabilities): current-latest runtime config rows have fixture evidence, implementation status that does not trigger P0 golden blockers, and config-loader/redteam owner as appropriate.
- [x] **AC2** (ADR-009): auxiliary code scan/MCP config rows are P1 snapshot evidence and do not weaken core promptfooconfig P0 semantics.
- [x] **AC3** (PRD §Non Goals / ADR-011): cloud/server/telemetry/global config rows remain explicit external P0 blockers with external-authority owner and blocker evidence.
- [x] **AC4** (task 24.4 claim contract): source/matrix/golden/quality artifacts show config blockers reduced to 7 external rows, but perfect-refactor completion remains false while other current-latest blockers remain.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-27.1.1 | TEST-27.1.1 | tests/current_latest_core_config_burndown.rs | install, typecheck, unit-test, integration, build | Done |
| AC2 | SCEN-27.1.1 | TEST-27.1.2 | tests/current_latest_core_config_burndown.rs | install, lint, typecheck, unit-test, coverage, build | Done |
| AC3 | SCEN-27.1.1 | TEST-27.1.3 | tests/current_latest_core_config_burndown.rs | install, typecheck, unit-test, e2e, runtime-smoke, build | Done |
| AC4 | SCEN-27.1.1 | TEST-27.1.4 | tests/current_latest_core_config_burndown.rs | install, lint, typecheck, unit-test, runtime-smoke, build | Done |

## 8. Risks

- Treating all config rows as native would hide external authority gaps. Tests must include global/server/tracing/types API config rows.
- Leaving `evidence_reference=blocker:*` on fixture/snapshot rows would keep false blockers in golden artifacts despite metadata changes.
- Reducing config blockers does not prove provider/eval/cache/prompt/script bridge parity. AC4 requires quality gate to remain blocked.

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

- **完成日期**：2026-06-02
- **改动文件**：
  - `docs/prds/promptfoo-rs.prd.md`
  - `docs/s2v-adapter.md`
  - `docs/specs/phases/phase-27-current-latest-core-config-burndown.md`
  - `docs/specs/tasks/task-27.1-current-latest-core-config-burndown.md`
  - `test/features/perfect-refactor-parity.feature`
  - `tests/current_latest_core_config_burndown.rs`
  - `tests/current_latest_viewer_config_reclassification.rs`
  - `src/compatibility/inventory.rs`
  - `scripts/release/current-latest-source-inventory.sh`
- **commit 列表**：
  - `3b0200d` `docs(spec): add phase 27 current latest core config burndown`
  - `59f1060` `docs(spec): task-27.1 enters implementation`
  - `cc394f0` `test(config): add current latest core config burndown RED tests`
  - `a4f33b9` `feat(config): classify current latest core config evidence`
  - `a1cbfa7` `refactor(config): satisfy current latest config lint`
  - `bec1d4f` `refactor(config): align current latest viewer config expectations`
  - 本次 docs 回填提交：`docs(spec): complete task 27.1 current latest core config burndown`
- **§9 Verification 结果**：
  - install: PASS — helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - lint: PASS — helper 执行 `bash scripts/release/lint.sh` 通过；中途发现并修复 clippy `manual_ignore_case_cmp`。
  - typecheck: PASS — helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS — helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-27.1.1 ~ TEST-27.1.4 通过，Phase 26 viewer config tests 已更新为兼容 Phase 27 的 fixture/external evidence 语义。
  - integration: PASS — helper 执行 `bash scripts/release/integration.sh` 通过。
  - e2e: PASS — helper 执行 `bash scripts/release/e2e.sh` 通过。
  - build: PASS — helper 执行 `cargo build --workspace`、viewer/npm `pnpm build` 通过。
  - coverage: PASS — helper 执行 `bash scripts/release/coverage.sh` 通过；`s2v_coverage_threshold_guard` 通过。
  - runtime-smoke: PASS — helper 执行 `bash scripts/release/runtime-smoke.sh` 通过；真实 artifact 复核显示 `current-latest-source-inventory.json` 与 `current-latest-matrix.json` status=`ready`、rows=3858、config fixture=8、config auxiliary=3、config external=7，`current-latest-golden-corpus.json` status=`ready-with-blockers`、blocker_count=92、config_blockers=7，`current-latest-quality.json` status=`ready-with-blockers`、local_current_latest_ready=false、perfect_refactor_claim_allowed=false。
- **剩余风险 / 未做项**：本 task 只燃尽 current-latest generic config blockers；仍有 92 个 current-latest P0 golden blockers（provider=38、eval-runner=18、prompt-processing=13、cache-store=9、config=7、script-bridge=7）、current-target claim boundary、21 个 external authority blockers，以及 publication `credential-blocked`。这些继续阻止“完美重构完成”声明。
- **下游 task 影响**：后续可优先处理 provider=38 或 eval-runner=18 blockers；7 个 config blockers 现为 explicit external authority rows，不能由本地代码伪造完成。
