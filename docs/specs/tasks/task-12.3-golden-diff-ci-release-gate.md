# Task 12.3: golden-diff-ci-release-gate

**Status**: Ready
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 12 — compatibility-fixtures-golden-diff
**Dependencies**: task-12.2-executable-upstream-rs-runner

## 1. Background

Existing release gate logic classifies supplied findings but does not run fixture corpus or publish artifacts. Stable release must depend on executable compatibility gate output. Basis: PRD §Compatibility Harness Design / §Release, ADR-007, ADR-008.

## 2. Goal

Wire executable golden diff runs into CI/release workflow, produce release gate summary artifacts, and block stable on P0 bug/unclassified or missing P0 fixture coverage.

## 3. Scope

### In Scope

- src/compatibility/release_gate.rs
- .github/workflows/release.yml
- compatibility/artifacts/release-gate/
- tests/golden_diff_ci_release_gate.rs
- docs/release.md

### Out Of Scope

- Does not implement missing CLI/provider behavior; later phases own behavior parity.
- Does not require real publish credentials; dry-run artifacts are acceptable.

## 4. Users / Actors

- Release manager: needs machine-readable stable/prerelease decision.
- CI maintainer: needs reproducible workflow steps.

## 5. Behavior Contract

Release gate command must fail stable candidate when P0 fixture coverage is incomplete, any P0 bug/unclassified diff exists, matrix has silent omissions, or artifacts are missing.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/tasks/task-12.2-executable-upstream-rs-runner.md
- docs/release.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- docs/decisions/adr-008-binary-first-multi-channel-release.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`serde_json`、内部模块 `compatibility::release_gate`、`compatibility::matrix`、`compatibility::harness`。
- CI：`.github/workflows/release.yml`。

### 5.3 函数签名

- `run_full_compatibility_gate(config: &GateConfig) -> Result<ReleaseGateSummary, GateError>`
- `write_release_gate_summary(summary: &ReleaseGateSummary, path: &Path) -> Result<(), GateError>`
- `assert_stable_allowed(summary: &ReleaseGateSummary) -> Result<(), GateError>`

## 6. Acceptance Criteria

- [ ] **AC1** (PRD §Release): CI/release workflow runs full compatibility gate before stable artifact upload.
- [ ] **AC2** (ADR-007): P0 bug/unclassified/missing fixture coverage blocks stable with nonzero exit.
- [ ] **AC3** (ADR-008): gate summary artifact records stable/prerelease/nightly decision and artifact paths.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-12.3.1 | TEST-12.3.1 | tests/golden_diff_ci_release_gate.rs | install, typecheck, unit-test, build, manual | Not Started |
| AC2 | SCEN-12.3.1 | TEST-12.3.2 | tests/golden_diff_ci_release_gate.rs | install, typecheck, unit-test, build, manual | Not Started |
| AC3 | SCEN-12.3.1 | TEST-12.3.3 | tests/golden_diff_ci_release_gate.rs | install, typecheck, unit-test, build, manual | Not Started |

## 8. Risks

- Full gate can be slow; allow quick P0 smoke for PR and full corpus for release candidate.
- Missing publish credentials should block publish, not compatibility evidence generation.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Build**: adapter §Commands Build
- **Manual**: inspect generated release gate summary and blocked stable failure path.

## 10. Completion Notes

- **完成日期**：<TBD-after-impl>
- **改动文件**：<TBD-after-impl>
- **commit 列表**：<TBD-after-impl>
- **§9 Verification 结果**：<TBD-after-impl>
- **剩余风险 / 未做项**：<TBD-after-impl>
- **下游 task 影响**：<TBD-after-impl>
