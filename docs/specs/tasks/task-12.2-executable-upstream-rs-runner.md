# Task 12.2: executable-upstream-rs-runner

**Status**: Ready
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 12 — compatibility-fixtures-golden-diff
**Dependencies**: task-12.1-p0-fixture-corpus

## 1. Background

Audit found `HarnessRunner` constructs in-memory artifacts and does not execute upstream promptfoo or promptfoo-rs. Perfect refactor proof requires real execution artifacts. Basis: PRD §Compatibility Harness Design, ADR-007, docs/audits/promptfoo-s2v-parity-claim-audit-2026-05-30.md.

## 2. Goal

Implement an executable runner that invokes pinned upstream promptfoo and current promptfoo-rs for the same fixture, then persists upstream, rs, normalized, and diff artifacts.

## 3. Scope

### In Scope

- src/compatibility/harness.rs
- src/compatibility/executor.rs
- compatibility/artifacts/
- tests/executable_upstream_rs_runner.rs
- docs/compatibility/harness.md

### Out Of Scope

- Does not decide fixture corpus contents; task 12.1 owns corpus.
- Does not publish release artifacts; task 12.3 and Phase 15 own release.

## 4. Users / Actors

- Release manager: needs reproducible artifacts for gate decisions.
- Maintainer: debugs fixture diffs and classification.

## 5. Behavior Contract

For each fixture, runner must create isolated work dirs, set deterministic env, disable upstream update checks, run upstream and rs commands, capture stdout/stderr/exit code/files, normalize artifacts, classify diffs, and persist outputs under compatibility/artifacts/<run-id>/.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/tasks/task-6.1-upstream-harness-runner.md
- docs/specs/tasks/task-12.1-p0-fixture-corpus.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`std::process::Command`、`tempfile`、`serde_json`、内部模块 `compatibility::harness`、`compatibility::normalize`、`compatibility::diff`。

### 5.3 函数签名

- `ExecutableHarnessRunner::run_fixture(fixture: &FixtureManifest) -> Result<PersistedRunArtifacts, HarnessError>`
- `PromptfooCommand::upstream_pinned(baseline: &BaselineReference) -> CommandSpec`
- `PromptfooCommand::current_rs(binary: &Path) -> CommandSpec`
- `persist_run_artifacts(run: &HarnessRun, output_dir: &Path) -> Result<PersistedRunArtifacts, HarnessError>`

## 6. Acceptance Criteria

- [ ] **AC1** (PRD §Compatibility Harness Design): runner executes upstream promptfoo and promptfoo-rs commands for the same fixture.
- [ ] **AC2** (ADR-007): runner persists upstream/rs/raw/normalized/diff artifacts with run metadata.
- [ ] **AC3** (docs/audits/S2V parity): command timeout, env isolation, update-disable, and no-secret behavior are enforced.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-12.2.1 | TEST-12.2.1 | tests/executable_upstream_rs_runner.rs | install, typecheck, unit-test, build, manual | Not Started |
| AC2 | SCEN-12.2.1 | TEST-12.2.2 | tests/executable_upstream_rs_runner.rs | install, typecheck, unit-test, build, manual | Not Started |
| AC3 | SCEN-12.2.1 | TEST-12.2.3 | tests/executable_upstream_rs_runner.rs | install, typecheck, unit-test, build, manual | Not Started |

## 8. Risks

- npx/upstream startup may hang; use pinned package, timeout, and `PROMPTFOO_DISABLE_UPDATE=true`.
- Windows shell quoting differs; command spec must avoid shell string concatenation.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Build**: adapter §Commands Build
- **Manual**: inspect one persisted fixture artifact tree.

## 10. Completion Notes

- **完成日期**：<TBD-after-impl>
- **改动文件**：<TBD-after-impl>
- **commit 列表**：<TBD-after-impl>
- **§9 Verification 结果**：<TBD-after-impl>
- **剩余风险 / 未做项**：<TBD-after-impl>
- **下游 task 影响**：<TBD-after-impl>
