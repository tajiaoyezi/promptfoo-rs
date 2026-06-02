# Task 32.1: current-latest-local-prompt-processor-burndown

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 32 - current-latest-local-prompt-processor-burndown
**Dependencies**: task-2.2-config-loader, task-2.3-eval-command-smoke, task-9.1-script-bridge-sandbox, task-24.4-current-latest-exhaustive-quality-gate, task-30.1-current-latest-prompt-processing-burndown, task-31.1-current-latest-cache-store-burndown

## 1. Background

Phase 31 leaves tracked-lock phase-smoke artifacts at 44 P0 golden blockers, including 6 `prompt-processing` blockers. Task 30.1 already split prompt index/string/text/utils rows into native fixture evidence and constants/grading/Ragas helper rows into P1 snapshot evidence. The remaining JSON, Markdown, and Jinja processor rows are local structured/template prompt parsing surfaces covered by config/prompt fixture semantics from task 2.2 and task 2.3. JavaScript, Python, and executable processor rows require script/subprocess execution authority and must remain P0 script-bridge blockers. 依据 PRD §Core Capabilities / §Compatibility Matrix、ADR-009、ADR-011、task 2.2、task 2.3、task 9.1、task 30.1、Phase 31 §9。

## 2. Goal

Reduce current-latest prompt-processing P0 golden blockers from 6 to 3 by promoting only local JSON/Markdown/Jinja processor rows to P0 native fixture evidence while preserving JS/Python/executable rows as explicit script-bridge blockers.

## 3. Scope

### In Scope

- `src/compatibility/inventory.rs`
- `scripts/release/current-latest-source-inventory.sh`
- `tests/current_latest_local_prompt_processor_burndown.rs`
- `target/release-gates/current-latest-source-inventory.json`
- `target/release-gates/current-latest-matrix.json`
- `target/release-gates/current-latest-golden-corpus.json`
- `target/release-gates/current-latest-quality.json`
- `docs/compatibility/matrix.md`
- `docs/s2v-adapter.md`
- `docs/prds/promptfoo-rs.prd.md`
- `test/features/perfect-refactor-parity.feature`

### Out Of Scope

- 不实现或声称 JS/Python/executable prompt processor native parity。
- 不放宽 script bridge default-deny、timeout、env allowlist、stdout/stderr、secret redaction 或 subprocess authorization 边界。
- 不解决 eval deletion、config external authority、provider external authority、eval-runner adaptive/rate-limit、script bridge runtime discovery、current-target drift、publication authority 或“无任何潜在 bug”承诺。

## 4. Users / Actors

- Prompt maintainer: needs local prompt parser rows to reuse deterministic config/eval prompt fixtures when sufficient.
- Release reviewer: needs script-backed processor rows to remain visible blockers until script bridge parity is proven.
- Security reviewer: needs JS/Python/executable processors to stay inside the script-bridge boundary and not become native by classification drift.

## 5. Behavior Contract

Current-latest `category=prompt-processing` rows must classify JSON, Markdown, and Jinja processor stable IDs as `level=P0`, `implementation_status=native`, `verification_owner=config-loader`, `evidence_kind=fixture`, and `evidence_reference=fixture:<stable-id>`. JavaScript, Python, and executable processor rows must remain `level=P0`, `implementation_status=blocked`, `verification_owner=script-bridge`, `evidence_kind=blocker`, and `evidence_reference=blocker:<stable-id>`. Rust and shell artifact generation must use equivalent local prompt processor rules.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/phases/phase-24-current-latest-perfect-refactor.md
- docs/specs/phases/phase-30-current-latest-prompt-processing-burndown.md
- docs/specs/phases/phase-31-current-latest-cache-store-burndown.md
- docs/specs/tasks/task-2.2-config-loader.md
- docs/specs/tasks/task-2.3-eval-command-smoke.md
- docs/specs/tasks/task-9.1-script-bridge-sandbox.md
- docs/specs/tasks/task-24.4-current-latest-exhaustive-quality-gate.md
- docs/specs/tasks/task-30.1-current-latest-prompt-processing-burndown.md
- docs/specs/tasks/task-31.1-current-latest-cache-store-burndown.md
- docs/compatibility/current-latest.lock.md
- docs/compatibility/matrix.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `promptfoo_rs::compatibility::inventory::{extract_current_latest_inventory, CurrentLatestTargetLock}`, `serde_json::Value`, `std::path::Path`, `std::process::Command`.
- Tooling commands: `bash scripts/release/current-latest-source-inventory.sh`, `bash scripts/release/current-latest-golden-corpus.sh`, `bash scripts/release/current-latest-quality-gate.sh`, `s2v_verify_full "$(s2v_extract_verify_keys "$TASK_SPEC")"`, adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke.

### 5.3 函数签名

- `current_latest_local_prompt_processor_fixture_ids(stable_id: &str) -> &'static [&'static str]`
- `is_current_latest_local_prompt_processor_fixture(stable_id: &str, file: &str) -> bool`
- Shell contract: `currentLatestLocalPromptProcessorFixtureIds(id)`, `isCurrentLatestLocalPromptProcessorFixture(id, file)`, `metadata(category, id, file)`.

## 6. Acceptance Criteria

- [x] **AC1** (task 2.2 / task 2.3 / ADR-009): JSON, Markdown, and Jinja current-latest prompt processor rows have P0 native fixture evidence and do not produce golden release blockers.
- [x] **AC2** (task 9.1 / ADR-009): JavaScript, Python, and executable current-latest prompt processor rows remain explicit P0 script-bridge blockers.
- [x] **AC3** (ADR-009): Rust extractor and shell extractor emit equivalent local prompt processor classification, evidence kind, evidence reference, and owner values.
- [x] **AC4** (ADR-011 / task 24.4): source/matrix/golden/quality artifacts show prompt-processing blockers reduced to 3 and total blockers reduced to 41 under the tracked-lock Phase 31 smoke target, while perfect-refactor completion remains false.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-32.1.1 | TEST-32.1.1 | tests/current_latest_local_prompt_processor_burndown.rs | install, lint, typecheck, unit-test, integration, build | Done |
| AC2 | SCEN-32.1.1 | TEST-32.1.2 | tests/current_latest_local_prompt_processor_burndown.rs | install, typecheck, unit-test, e2e, runtime-smoke, build | Done |
| AC3 | SCEN-32.1.1 | TEST-32.1.3 | tests/current_latest_local_prompt_processor_burndown.rs | install, lint, typecheck, unit-test, coverage, build | Done |
| AC4 | SCEN-32.1.1 | TEST-32.1.4 | tests/current_latest_local_prompt_processor_burndown.rs | install, lint, typecheck, unit-test, integration, e2e, runtime-smoke, build | Done |

## 8. Risks

- Treating JS/Python/executable rows as native would bypass script-bridge authorization and redaction requirements.
- Local JSON/Markdown/Jinja fixture evidence does not imply rich templating compatibility beyond the deterministic prompt/config surfaces covered by this task.
- Reducing prompt-processing blockers does not prove eval deletion, provider external authority, script bridge runtime discovery, config external authority, eval-runner rate-limit behavior, publication, or impossible zero-bug claims.

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
  - `docs/specs/phases/phase-32-current-latest-local-prompt-processor-burndown.md`
  - `docs/specs/tasks/task-32.1-current-latest-local-prompt-processor-burndown.md`
  - `docs/s2v-adapter.md`
  - `docs/prds/promptfoo-rs.prd.md`
  - `docs/compatibility/matrix.md`
  - `test/features/perfect-refactor-parity.feature`
  - `tests/current_latest_local_prompt_processor_burndown.rs`
  - `tests/current_latest_prompt_processing_burndown.rs`
  - `src/compatibility/inventory.rs`
  - `scripts/release/current-latest-source-inventory.sh`
- **commit 列表**：
  - `c243537` `docs(spec): add phase 32 current latest local prompt processor burndown`
  - `d4ba3a1` `docs(spec): task-32.1 enters implementation`
  - `ad58d24` `test(prompt-processing): add current latest local prompt processor RED tests`
  - `cfc05b9` `feat(prompt-processing): classify current latest local prompt processors`
  - 本次 docs 回填提交：`docs(spec): complete task 32.1 current latest local prompt processor burndown`
- **§9 Verification 结果**：
  - install: PASS - helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - lint: PASS - helper 执行 `bash scripts/release/lint.sh` 通过。
  - typecheck: PASS - helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS - helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-32.1.1 ~ TEST-32.1.4 与累计 TEST-30.1.1 ~ TEST-30.1.5 均通过。
  - integration: PASS - helper 执行 adapter Integration tests 通过。
  - e2e: PASS - helper 执行 adapter E2E tests 通过。
  - coverage: PASS - helper 执行 adapter Coverage，通过覆盖率阈值守卫。
  - build: PASS - helper 执行 adapter Build 通过。
  - runtime-smoke: PASS - helper 执行 adapter Runtime smoke 通过；`current-latest-golden-corpus.json` 为 `ready-with-blockers`、`p0_total=92`、`fixture_case_count=92`、`blocker_count=41`、分组 `cache-store=1, config=7, eval-runner=7, prompt-processing=3, provider=16, script-bridge=7`，`perfect_refactor_claim_allowed=false`。
- **剩余风险 / 未做项**：JS/Python/executable prompt processor rows 仍是 3 个 P0 script-bridge blockers；全局仍保留 cache-store=1、config=7、eval-runner=7、provider external-authority=16、script-bridge=7、current-target drift、external-authority、publication-authority 等 blockers；不承诺“无任何潜在 bug”。
- **下游 task 影响**：后续 current-latest burndown 可从 41 个总 blockers 继续推进；script-backed processor parity 必须通过 task 9.1 边界下的专用 subprocess/script fixture 证明，不能由本 task 的 local prompt parser fixture 间接视为 native。
