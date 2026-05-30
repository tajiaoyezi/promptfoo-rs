# Task 12.3: golden-diff-ci-release-gate

**Status**: Done
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

- [x] **AC1** (PRD §Release): CI/release workflow runs full compatibility gate before stable artifact upload.
- [x] **AC2** (ADR-007): P0 bug/unclassified/missing fixture coverage blocks stable with nonzero exit.
- [x] **AC3** (ADR-008): gate summary artifact records stable/prerelease/nightly decision and artifact paths.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-12.3.1 | TEST-12.3.1 | tests/golden_diff_ci_release_gate.rs | install, typecheck, unit-test, build, manual | Done |
| AC2 | SCEN-12.3.1 | TEST-12.3.2 | tests/golden_diff_ci_release_gate.rs | install, typecheck, unit-test, build, manual | Done |
| AC3 | SCEN-12.3.1 | TEST-12.3.3 | tests/golden_diff_ci_release_gate.rs | install, typecheck, unit-test, build, manual | Done |

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

- **完成日期**：2026-05-30
- **改动文件**：
  - `src/compatibility/release_gate.rs`
  - `tests/golden_diff_ci_release_gate.rs`
  - `tests/release_docs_packaging.rs`
  - `.github/workflows/release.yml`
  - `docs/release.md`
  - `compatibility/artifacts/release-gate/README.md`
  - `docs/specs/tasks/task-12.3-golden-diff-ci-release-gate.md`
- **commit 列表**：
  - `b039ce9` `docs(spec): task-12.3 进入实施 (Status: Ready → In Progress)`
  - `431f78f` `test(release-gate): 加 SCEN-12.3.1 的 3 个 RED 测试`
  - `9bf73ef` `feat(release-gate): 接入 full golden diff release gate`
- **§9 Verification 结果**：
  - install: PASS — `s2v_verify_full "install typecheck unit-test build"` 中 `cargo fetch` 通过。
  - typecheck: PASS — `cargo check --workspace` 通过。
  - unit-test: PASS — `cargo test --workspace` 通过，包含 `tests/golden_diff_ci_release_gate.rs` 的 TEST-12.3.1~TEST-12.3.3。
  - build: PASS — `cargo build --workspace` 通过。
  - manual: PASS — 已检查 generated summary：`release_channel=stable`、`stable_allowed=true`、`status=Ready`、artifact_paths=2；TEST-12.3.2 覆盖 P0 bug、unclassified diff、49/50 fixture coverage 时 stable blocked 且 `GateError::exit_code()!=0`。非交互 helper 的 full run 仅因 `/dev/tty` manual 确认失败，机械 keys 已单独全绿。
- **剩余风险 / 未做项**：CI workflow 现在有 full gate 测试和 release-gate artifact 路径，但真实发布凭据与实际 stable artifact upload 仍按授权范围外处理；后续 Phase 15 会补齐多渠道发布 smoke。
- **下游 task 影响**：Phase 12 可收尾；Phase 15 release packaging 可复用 `ReleaseGateSummary.release_channel/stable_allowed/artifact_paths` 作为稳定发布决策输入。
