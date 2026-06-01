# Task 24.3: current-latest-full-function-golden-corpus

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 24 — current-latest-perfect-refactor
**Dependencies**: task-24.2-current-latest-source-inventory-reextract

## 1. Background

The user requires a complete functional refactor with a large test suite. The existing frozen corpus and 50+ fixture gate are insufficient for a current-latest perfect-refactor claim. 依据用户 2026-06-01 澄清、PRD §Compatibility Harness Design / §Compatibility Matrix、ADR-007、ADR-009、ADR-011。

## 2. Goal

Expand current-latest fixtures, snapshots, and golden diff artifacts so every P0 row is executable and every P1 row has snapshot/protocol evidence.

## 3. Scope

### In Scope

- `compatibility/fixtures/current-latest/`
- `compatibility/artifacts/current-latest/`
- `compatibility/matrix/current-latest-matrix.json`
- `scripts/release/current-latest-golden-corpus.sh`
- `scripts/release/runtime-smoke.sh`
- `src/compatibility/harness.rs`
- `tests/current_latest_golden_corpus.rs`
- `test/features/perfect-refactor-parity.feature`

### Out Of Scope

- 不使用真实 paid provider calls 作为默认 CI gate。
- 不把 mocked/recorded evidence 伪装成 live external provider authority。
- 不降低 P0 golden diff 要求。

## 4. Users / Actors

- Compatibility reviewer: verifies P0/P1 rows are covered by executable artifacts.
- CI maintainer: needs deterministic tests that run without private secrets.
- Release maintainer: needs blocker reports for rows without enough evidence.

## 5. Behavior Contract

The current-latest corpus generator must read the current-latest matrix from task 24.2 and ensure every P0 row has at least one executable fixture and golden diff result. P1 rows must have snapshot or protocol evidence. P2 rows must have known gap, later reason, unsupported reason, or formal waiver evidence. The release gate must fail if any P0 row lacks a fixture, any P0 diff is `bug` or unclassified, or any P1 row lacks a snapshot/protocol artifact.

### 5.1 Required Reading

- docs/specs/tasks/task-24.2-current-latest-source-inventory-reextract.md
- docs/specs/tasks/task-12.1-p0-fixture-corpus.md
- docs/specs/tasks/task-12.2-executable-upstream-rs-runner.md
- docs/specs/tasks/task-17.3-real-p0-golden-corpus-runner.md
- docs/decisions/adr-007-upstream-golden-diff-release-gate.md
- docs/decisions/adr-009-p0-p1-p2-compatibility-matrix.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module: `serde_json::Value`, `std::fs`, `std::path::PathBuf`, `promptfoo_rs::compatibility::harness::{GoldenCorpusReport, GoldenDiffFinding}`.
- Tooling commands: `bash scripts/release/current-latest-golden-corpus.sh`, adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke.

### 5.3 函数签名

- `build_current_latest_golden_corpus(matrix_path: &Path, fixtures_root: &Path, artifacts_root: &Path) -> Result<GoldenCorpusReport, HarnessError>`
- `evaluate_current_latest_release_blockers(report: &GoldenCorpusReport) -> Vec<GoldenDiffFinding>`
- Shell contract: `CURRENT_LATEST_FIXTURES_ROOT=compatibility/fixtures/current-latest bash scripts/release/current-latest-golden-corpus.sh`

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Compatibility Harness Design): 100% current-latest P0 rows have executable fixtures and upstream/rs normalized artifacts.
- [x] **AC2** (ADR-007): P0 `bug` and unclassified diffs block the current-latest perfect-refactor claim.
- [x] **AC3** (ADR-009): 100% current-latest P1 rows have snapshot or protocol evidence; P2 rows have explicit reason/waiver/later evidence.
- [x] **AC4** (user 2026-06-01): corpus scale is materially larger than the frozen baseline gate: at least 250 fixture cases or 100% of current-latest inventory rows when the inventory has fewer than 250 rows.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-24.3.1 | TEST-24.3.1 | tests/current_latest_golden_corpus.rs | install, typecheck, unit-test, integration, build | Done |
| AC2 | SCEN-24.3.1 | TEST-24.3.2 | tests/current_latest_golden_corpus.rs | install, lint, typecheck, unit-test, coverage, build | Done |
| AC3 | SCEN-24.3.1 | TEST-24.3.3 | tests/current_latest_golden_corpus.rs | install, typecheck, unit-test, e2e, runtime-smoke, build | Done |
| AC4 | SCEN-24.3.1 | TEST-24.3.4 | tests/current_latest_golden_corpus.rs | install, lint, typecheck, unit-test, runtime-smoke, build | Done |

## 8. Risks

- A 250+ fixture corpus can be expensive; fixtures should use mock/recorded providers for deterministic CI.
- Upstream behavior can be nondeterministic for model-graded assertions; normalization and recorded graders must be explicit.
- Live provider differences require external authority evidence, not silent pass.

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

- **完成日期**：2026-06-01
- **改动文件**：
  - `tests/current_latest_golden_corpus.rs`
  - `src/compatibility/harness.rs`
  - `scripts/release/current-latest-golden-corpus.sh`
  - `scripts/release/runtime-smoke.sh`
  - `scripts/release/integration.sh`
  - `scripts/release/coverage.sh`
  - `docs/compatibility/matrix.md`
  - `docs/compatibility/target-policy.md`
  - `docs/specs/tasks/task-24.3-current-latest-full-function-golden-corpus.md`
  - `docs/specs/phases/phase-24-current-latest-perfect-refactor.md`
  - `docs/s2v-adapter.md`
- **commit 列表**：
  - `c9e47c2` `test(compatibility): add current latest golden corpus RED tests`
  - `6728b32` `feat(compatibility): add current latest golden corpus gate`
- **§9 Verification 结果**：
  - install: PASS — helper 执行 adapter Install，`cargo fetch`、viewer/npm `pnpm install --frozen-lockfile` 通过。
  - lint: PASS — helper 执行 `bash scripts/release/lint.sh` 通过。
  - typecheck: PASS — helper 执行 `cargo check --workspace`、viewer/npm `pnpm typecheck` 通过。
  - unit-test: PASS — helper 执行 `cargo test --workspace`、viewer/npm `pnpm test` 通过；新增 TEST-24.3.1 ~ TEST-24.3.4 通过。
  - integration: PASS — helper 执行 `bash scripts/release/integration.sh` 通过，包含 `current_latest_golden_corpus`。
  - e2e: PASS — helper 执行 `bash scripts/release/e2e.sh` 通过。
  - build: PASS — helper 执行 `cargo build --workspace`、viewer/npm `pnpm build` 通过。
  - coverage: PASS — helper 执行 `bash scripts/release/coverage.sh` 通过；coverage traceability gate 包含 TEST-24.3.1 ~ TEST-24.3.4；`s2v_coverage_threshold_guard` 通过。
  - runtime-smoke: PASS — helper 执行 `bash scripts/release/runtime-smoke.sh` 通过；`current-latest-golden-corpus.json` status=`ready-with-blockers`，fixture_case_count=432，p0_total=432，p0_fixture_coverage_count=432，p0_artifact_coverage_count=432，p1_snapshot_coverage_count=2087/2087，p2_registration_coverage_count=1393/1393，blocker_count=432，`perfect_refactor_claim_allowed=false`；`release-candidate.json` 暴露 current-latest golden corpus gate 状态。
- **剩余风险 / 未做项**：本 task 生成 current-latest P0 fixture/artifact slots、P1 snapshot evidence、P2 registration evidence，并把 432 个 P0 blocker 继续保留为 release-blocking evidence。它不把 blocker row 伪装成 native parity，也不使用真实 paid provider calls；current-latest perfect-refactor claim 仍为 false，等待 task 24.4 聚合质量 gate 与后续 blocker/authority/waiver 决策。
- **下游 task 影响**：task 24.4 必须消费 `target/release-gates/current-latest-golden-corpus.json`，把 432 个 P0 corpus blockers、existing source/matrix blockers、stress/property/regression gates 和 forbidden zero-bug wording 一并纳入最终质量 claim。
