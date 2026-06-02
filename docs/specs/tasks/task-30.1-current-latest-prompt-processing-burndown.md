# Task 30.1: current-latest-prompt-processing-burndown

**Status**: Ready
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 30 - current-latest-prompt-processing-burndown
**Dependencies**: task-2.2-config-loader, task-2.3-eval-command-smoke, task-4.2-assertion-engine, task-9.1-script-bridge-sandbox, task-24.4-current-latest-exhaustive-quality-gate, task-29.1-current-latest-eval-runner-burndown

## 1. Background

Phase 29 left phase-smoke artifacts at 60 P0 golden blockers, including 13 `prompt-processing` blockers. Earlier tasks already proved basic prompt list loading, file prompt loading, env substitution, simple `{{var}}` rendering in eval output, model-graded prompt schema construction, and script bridge default-deny/authorization boundaries. This task applies only those existing fixtures to current-latest prompt rows while preserving unproven JSON, Markdown, Jinja, JavaScript, Python, and executable prompt processors as blockers. 依据 PRD §Core Capabilities / §Compatibility Matrix、ADR-009、ADR-011、task 2.2、task 2.3、task 4.2、task 9.1、Phase 29 §9。

## 2. Goal

Reduce current-latest prompt-processing P0 golden blockers from 13 generic blockers to 6 explicit blockers while preserving 4 fixture-covered rows as P0 native fixture evidence and 3 static/external helper rows as P1 snapshot evidence.

## 3. Scope

### In Scope

- `src/compatibility/inventory.rs`
- `scripts/release/current-latest-source-inventory.sh`
- `tests/current_latest_prompt_processing_burndown.rs`
- `target/release-gates/current-latest-source-inventory.json`
- `target/release-gates/current-latest-matrix.json`
- `target/release-gates/current-latest-golden-corpus.json`
- `target/release-gates/current-latest-quality.json`
- `docs/compatibility/matrix.md`
- `docs/s2v-adapter.md`
- `docs/prds/promptfoo-rs.prd.md`
- `test/features/perfect-refactor-parity.feature`

### Out Of Scope

- 不实现 upstream JSON、Markdown、Jinja、JavaScript、Python 或 executable prompt processor semantic parity。
- 不调用真实 provider、外部服务、private SDK、账号或 API key。
- 不解决 cache-store、script-bridge runtime discovery、provider external-authority、config external、current-target 或 publication blockers。
- 不承诺“无任何潜在 bug”；claim 仍受 ADR-011 的 evidence boundary 约束。

## 4. Users / Actors

- Prompt maintainer: needs current-latest prompt rows to reuse existing deterministic config/eval prompt fixture evidence when sufficient.
- Release reviewer: needs prompt-processing blocker reduction to be item-level and not hide rich processor gaps.
- Security reviewer: needs JS/Python/executable prompt processors to keep script-bridge boundaries visible until separately authorized and tested.

## 5. Behavior Contract

Current-latest `category=prompt-processing` rows must be classified by stable id and source path. Fixture-covered rows must use `level=P0`, `implementation_status=native`, `verification_owner=config-loader`, `evidence_kind=fixture`, and `evidence_reference=fixture:<stable-id>`. P1 snapshot rows must use `level=P1`, `implementation_status=later`, `verification_owner=config-loader`, `evidence_kind=snapshot`, and `evidence_reference=snapshot:<stable-id>`. Unproven prompt processor rows must remain `level=P0`, `implementation_status=blocked`, `verification_owner=config-loader` or `script-bridge`, `evidence_kind=blocker`, and `evidence_reference=blocker:<stable-id>`. Rust and shell artifact generation must use equivalent prompt-processing rules.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/phases/phase-24-current-latest-perfect-refactor.md
- docs/specs/phases/phase-29-current-latest-eval-runner-burndown.md
- docs/specs/tasks/task-2.2-config-loader.md
- docs/specs/tasks/task-2.3-eval-command-smoke.md
- docs/specs/tasks/task-4.2-assertion-engine.md
- docs/specs/tasks/task-9.1-script-bridge-sandbox.md
- docs/specs/tasks/task-24.4-current-latest-exhaustive-quality-gate.md
- docs/specs/tasks/task-29.1-current-latest-eval-runner-burndown.md
- docs/compatibility/current-latest.lock.md
- docs/compatibility/matrix.md
- docs/decisions/adr-005-explicit-script-authorization.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/config-loader.feature
- test/features/eval-runner.feature
- test/features/script-bridge.feature
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `promptfoo_rs::compatibility::inventory::{extract_current_latest_inventory, CurrentLatestTargetLock}`, `serde_json::Value`, `std::path::Path`, `std::process::Command`.
- Tooling commands: `bash scripts/release/current-latest-source-inventory.sh`, `bash scripts/release/current-latest-golden-corpus.sh`, `bash scripts/release/current-latest-quality-gate.sh`, `s2v_verify_full "$(s2v_extract_verify_keys "$TASK_SPEC")"`, adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke.

### 5.3 函数签名

- `current_latest_prompt_processing_fixture_ids(stable_id: &str) -> &'static [&'static str]`
- `is_current_latest_prompt_processing_fixture(stable_id: &str, file: &str) -> bool`
- `is_current_latest_prompt_processing_snapshot(file: &str) -> bool`
- `current_latest_prompt_processing_blocker_owner(stable_id: &str, file: &str) -> &'static str`
- `current_latest_prompt_processing_blocker_reason(stable_id: &str, file: &str) -> String`
- Shell contract: `currentLatestPromptProcessingFixtureIds(id)`, `isCurrentLatestPromptProcessingFixture(id, file)`, `isCurrentLatestPromptProcessingSnapshot(file)`, `currentLatestPromptProcessingBlockerOwner(id, file)`, `currentLatestPromptProcessingBlockerReason(id, file)`, `metadata(category, id, file)`.

## 6. Acceptance Criteria

- [ ] **AC1** (PRD §Config / task 2.2 / task 2.3): 4 current-latest prompt index/string/text/utils rows have P0 native fixture evidence and do not produce golden release blockers.
- [ ] **AC2** (ADR-009): 3 current-latest prompt constants, grading helper, and external Ragas prompt rows are P1 snapshot evidence and do not weaken P0 prompt processor semantics.
- [ ] **AC3** (ADR-005 / ADR-011): 6 current-latest JSON/Markdown/Jinja/JavaScript/Python/executable prompt processor rows remain explicit P0 prompt-processing blockers.
- [ ] **AC4** (ADR-009): Rust extractor and shell extractor emit equivalent prompt-processing classification, evidence kind, evidence reference, and owner values.
- [ ] **AC5** (ADR-011 / task 24.4): source/matrix/golden/quality artifacts show prompt-processing blockers reduced to 6 and total blockers reduced to 53 under the current Phase 29 smoke target, while perfect-refactor completion remains false.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-30.1.1 | TEST-30.1.1 | tests/current_latest_prompt_processing_burndown.rs | install, lint, typecheck, unit-test, integration, build | Ready |
| AC2 | SCEN-30.1.1 | TEST-30.1.2 | tests/current_latest_prompt_processing_burndown.rs | install, typecheck, unit-test, coverage, build | Ready |
| AC3 | SCEN-30.1.1 | TEST-30.1.3 | tests/current_latest_prompt_processing_burndown.rs | install, typecheck, unit-test, e2e, runtime-smoke, build | Ready |
| AC4 | SCEN-30.1.1 | TEST-30.1.4 | tests/current_latest_prompt_processing_burndown.rs | install, lint, typecheck, unit-test, coverage, build | Ready |
| AC5 | SCEN-30.1.1 | TEST-30.1.5 | tests/current_latest_prompt_processing_burndown.rs | install, lint, typecheck, unit-test, integration, e2e, runtime-smoke, build | Ready |

## 8. Risks

- Marking all prompt rows native would hide JSON, Markdown, Jinja, JS, Python, and executable processor gaps that are not covered by task 2.2/2.3/4.2/9.1.
- Downgrading static/external helper rows to P1 must stay traceable as snapshot evidence; it is not a native implementation claim.
- Reducing prompt-processing blockers does not prove cache-store, script-bridge runtime discovery, provider external authority, publication, or current-target readiness.

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
