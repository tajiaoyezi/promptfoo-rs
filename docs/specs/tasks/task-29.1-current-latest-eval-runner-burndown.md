# Task 29.1: current-latest-eval-runner-burndown

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 29 - current-latest-eval-runner-burndown
**Dependencies**: task-3.1-scheduler-runtime, task-3.2-cache-resume-retry, task-13.2-eval-output-cache-parity, task-24.4-current-latest-exhaustive-quality-gate, task-28.1-current-latest-provider-fixture-burndown

## 1. Background

Phase 28 left `current-latest-golden-corpus.json` at 70 P0 golden blockers, including 18 `eval-runner` blockers. Earlier tasks already proved common eval command execution, output/exit-code behavior, resume/cache state, retry/backoff, max concurrency, delay/cancellation, and partial failure behavior. This task applies those existing fixtures to current-latest source rows while preserving unproven rate-limit/adaptive/provider-wrapper rows as blockers. 依据 PRD §Core Capabilities / §Compatibility Matrix、ADR-009、ADR-011、task 3.1、task 3.2、task 13.2、Phase 28 §9。

## 2. Goal

Reduce current-latest eval-runner P0 golden blockers from 18 generic blockers to 7 explicit blockers while preserving 8 fixture-covered rows as P0 native fixture evidence and 3 non-core/current-latest rows as P1 snapshot evidence.

## 3. Scope

### In Scope

- `src/compatibility/inventory.rs`
- `scripts/release/current-latest-source-inventory.sh`
- `tests/current_latest_eval_runner_burndown.rs`
- `target/release-gates/current-latest-source-inventory.json`
- `target/release-gates/current-latest-matrix.json`
- `target/release-gates/current-latest-golden-corpus.json`
- `target/release-gates/current-latest-quality.json`
- `docs/compatibility/matrix.md`
- `docs/s2v-adapter.md`
- `docs/prds/promptfoo-rs.prd.md`
- `test/features/perfect-refactor-parity.feature`

### Out Of Scope

- 不实现 upstream adaptive concurrency、provider rate-limit header parsing、provider wrapper semantic parity 或 prompt optimization behavior。
- 不调用真实 provider、外部服务、private SDK、账号或 API key。
- 不解决 prompt-processing、cache-store、script-bridge、provider external-authority、config external、current-target 或 publication blockers。
- 不承诺“无任何潜在 bug”；claim 仍受 ADR-011 的 evidence boundary 约束。

## 4. Users / Actors

- Eval maintainer: needs current-latest eval core rows to reuse existing deterministic eval/scheduler fixture evidence when that evidence is sufficient.
- Release reviewer: needs eval-runner blocker reduction to be item-level and not hide rate-limit/adaptive gaps.
- CI maintainer: needs eval stdout/stderr/exit code/output/cache/retry evidence to remain traceable to existing tests and artifacts.

## 5. Behavior Contract

Current-latest `category=eval-runner` rows must be classified by stable id and source path. Fixture-covered rows must use `level=P0`, `implementation_status=native`, `verification_owner=eval-runner`, `evidence_kind=fixture`, and `evidence_reference=fixture:<stable-id>`. P1 snapshot rows must use `level=P1`, `implementation_status=later`, `verification_owner=eval-runner`, `evidence_kind=snapshot`, and `evidence_reference=snapshot:<stable-id>`. Unproven adaptive/rate-limit/provider-wrapper rows must remain `level=P0`, `implementation_status=blocked`, `verification_owner=eval-runner`, `evidence_kind=blocker`, and `evidence_reference=blocker:<stable-id>`. Rust and shell artifact generation must use equivalent eval-runner rules.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/phases/phase-24-current-latest-perfect-refactor.md
- docs/specs/phases/phase-28-current-latest-provider-fixture-burndown.md
- docs/specs/tasks/task-3.1-scheduler-runtime.md
- docs/specs/tasks/task-3.2-cache-resume-retry.md
- docs/specs/tasks/task-13.2-eval-output-cache-parity.md
- docs/specs/tasks/task-24.4-current-latest-exhaustive-quality-gate.md
- docs/specs/tasks/task-28.1-current-latest-provider-fixture-burndown.md
- docs/compatibility/current-latest.lock.md
- docs/compatibility/matrix.md
- docs/decisions/adr-003-streaming-jsonl-sqlite-store.md
- docs/decisions/adr-004-cli-output-schema-protocol.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/eval-runner.feature
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `promptfoo_rs::compatibility::inventory::{extract_current_latest_inventory, CurrentLatestTargetLock}`, `serde_json::Value`, `std::path::Path`, `std::process::Command`.
- Tooling commands: `bash scripts/release/current-latest-source-inventory.sh`, `bash scripts/release/current-latest-golden-corpus.sh`, `bash scripts/release/current-latest-quality-gate.sh`, `s2v_verify_full "$(s2v_extract_verify_keys "$TASK_SPEC")"`, adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke.

### 5.3 函数签名

- `current_latest_eval_runner_fixture_ids(stable_id: &str) -> &'static [&'static str]`
- `is_current_latest_eval_runner_fixture(stable_id: &str, file: &str) -> bool`
- `is_current_latest_eval_runner_snapshot(file: &str) -> bool`
- `current_latest_eval_runner_blocker_reason(stable_id: &str, file: &str) -> String`
- Shell contract: `currentLatestEvalRunnerFixtureIds(id)`, `isCurrentLatestEvalRunnerFixture(id, file)`, `isCurrentLatestEvalRunnerSnapshot(file)`, `currentLatestEvalRunnerBlockerReason(id, file)`, `metadata(category, id, file)`.

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Eval runner / task 3.1 / task 13.2): 8 current-latest eval/evaluator/scheduler rows have P0 native fixture evidence and do not produce golden release blockers.
- [x] **AC2** (ADR-009): 3 current-latest optimizer/event/synthesis rows are P1 snapshot evidence and do not weaken P0 eval-runner semantics.
- [x] **AC3** (ADR-011): 7 current-latest adaptive/rate-limit/provider-wrapper rows remain explicit P0 eval-runner blockers.
- [x] **AC4** (ADR-009): Rust extractor and shell extractor emit equivalent eval-runner classification, evidence kind, evidence reference, and owner values.
- [x] **AC5** (ADR-011 / task 24.4): source/matrix/golden/quality artifacts show eval-runner blockers reduced to 7 and total blockers reduced to 59, while perfect-refactor completion remains false.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-29.1.1 | TEST-29.1.1 | tests/current_latest_eval_runner_burndown.rs | install, lint, typecheck, unit-test, integration, build | Done |
| AC2 | SCEN-29.1.1 | TEST-29.1.2 | tests/current_latest_eval_runner_burndown.rs | install, typecheck, unit-test, coverage, build | Done |
| AC3 | SCEN-29.1.1 | TEST-29.1.3 | tests/current_latest_eval_runner_burndown.rs | install, typecheck, unit-test, e2e, runtime-smoke, build | Done |
| AC4 | SCEN-29.1.1 | TEST-29.1.4 | tests/current_latest_eval_runner_burndown.rs | install, lint, typecheck, unit-test, coverage, build | Done |
| AC5 | SCEN-29.1.1 | TEST-29.1.5 | tests/current_latest_eval_runner_burndown.rs | install, lint, typecheck, unit-test, integration, e2e, runtime-smoke, build | Done |

## 8. Risks

- Marking all scheduler rows native would hide adaptive concurrency, rate-limit, header parsing, and provider wrapper gaps that are not covered by task 3.1/3.2/13.2.
- Downgrading optimizer/test synthesis to P1 must stay traceable as snapshot evidence; it is not a native implementation claim.
- Reducing eval-runner blockers does not prove prompt-processing, cache-store, script-bridge, provider external authority, publication, or current-target readiness.

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
  - `docs/specs/phases/phase-29-current-latest-eval-runner-burndown.md`
  - `docs/specs/tasks/task-29.1-current-latest-eval-runner-burndown.md`
  - `docs/s2v-adapter.md`
  - `docs/prds/promptfoo-rs.prd.md`
  - `test/features/perfect-refactor-parity.feature`
  - `tests/current_latest_eval_runner_burndown.rs`
  - `tests/current_latest_source_taxonomy_burndown.rs`
  - `src/compatibility/inventory.rs`
  - `scripts/release/current-latest-source-inventory.sh`
  - `docs/compatibility/matrix.md`
- **commit 列表**：
  - `0b4d52b` `docs(spec): add phase 29 current latest eval runner burndown`
  - `5b6b0b5` `docs(spec): task-29.1 enters implementation`
  - `9d8c110` `test(eval-runner): add current latest eval runner burndown RED tests`
  - `148d502` `feat(eval-runner): classify current latest eval runner evidence`
  - `83f2abf` `test(eval-runner): keep taxonomy blocker representative current`
  - 本次 docs 回填提交：`docs(spec): complete task 29.1 current latest eval runner burndown`
- **§9 Verification 结果**：
  - install: PASS - helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - lint: PASS - helper 执行 `bash scripts/release/lint.sh` 通过。
  - typecheck: PASS - helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS - helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-29.1.1 ~ TEST-29.1.5 通过，旧 TEST-25.1.2 已更新为仍未证明的 eval-runner blocker 代表行并通过。
  - integration: PASS - helper 执行 adapter Integration tests 通过。
  - e2e: PASS - helper 执行 adapter E2E tests 通过。
  - coverage: PASS - helper 执行 adapter Coverage，通过覆盖率阈值守卫。
  - build: PASS - helper 执行 adapter Build 通过。
  - runtime-smoke: PASS - helper 执行 adapter Runtime smoke 通过；`current-latest-golden-corpus.json` 仍为 `ready-with-blockers`，`perfect_refactor_claim_allowed=false`。
- **剩余风险 / 未做项**：仍保留 7 个 P0 eval-runner blockers（adaptive concurrency、provider rate-limit/header parsing、provider wrapper 等），以及 provider external-authority、prompt-processing、cache-store、script-bridge、current-target、publication 等非本 task 范围 blockers；不承诺“无任何潜在 bug”。
- **下游 task 影响**：后续 current-latest burndown task 可从 59 个总 blockers 继续推进；任何涉及 adaptive/rate-limit/provider-wrapper native parity 的 task 必须新增专用 fixture 和 RED 测试，不能复用本 task 的 P1 snapshot 或 blocker 记录作为 native 证明。
