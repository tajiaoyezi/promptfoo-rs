# Task 29.1: current-latest-eval-runner-burndown

**Status**: In Progress
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

- [ ] **AC1** (PRD §Eval runner / task 3.1 / task 13.2): 8 current-latest eval/evaluator/scheduler rows have P0 native fixture evidence and do not produce golden release blockers.
- [ ] **AC2** (ADR-009): 3 current-latest optimizer/event/synthesis rows are P1 snapshot evidence and do not weaken P0 eval-runner semantics.
- [ ] **AC3** (ADR-011): 7 current-latest adaptive/rate-limit/provider-wrapper rows remain explicit P0 eval-runner blockers.
- [ ] **AC4** (ADR-009): Rust extractor and shell extractor emit equivalent eval-runner classification, evidence kind, evidence reference, and owner values.
- [ ] **AC5** (ADR-011 / task 24.4): source/matrix/golden/quality artifacts show eval-runner blockers reduced to 7 and total blockers reduced to 59, while perfect-refactor completion remains false.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-29.1.1 | TEST-29.1.1 | tests/current_latest_eval_runner_burndown.rs | install, lint, typecheck, unit-test, integration, build | Ready |
| AC2 | SCEN-29.1.1 | TEST-29.1.2 | tests/current_latest_eval_runner_burndown.rs | install, typecheck, unit-test, coverage, build | Ready |
| AC3 | SCEN-29.1.1 | TEST-29.1.3 | tests/current_latest_eval_runner_burndown.rs | install, typecheck, unit-test, e2e, runtime-smoke, build | Ready |
| AC4 | SCEN-29.1.1 | TEST-29.1.4 | tests/current_latest_eval_runner_burndown.rs | install, lint, typecheck, unit-test, coverage, build | Ready |
| AC5 | SCEN-29.1.1 | TEST-29.1.5 | tests/current_latest_eval_runner_burndown.rs | install, lint, typecheck, unit-test, integration, e2e, runtime-smoke, build | Ready |

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

- **完成日期**：待实施
- **改动文件**：待实施
- **commit 列表**：待实施
- **§9 Verification 结果**：待实施
- **剩余风险 / 未做项**：待实施
- **下游 task 影响**：待实施
