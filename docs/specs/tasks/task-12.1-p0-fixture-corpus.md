# Task 12.1: p0-fixture-corpus

**Status**: Done
**Priority**: P0
**Owner**: leafiellune
**Related Phase**: Phase 12 — compatibility-fixtures-golden-diff
**Dependencies**: task-11.3-compatibility-matrix-expansion

## 1. Background

PRD primary metric requires at least 50 P0 fixtures, but audit found 0 tracked compatibility fixtures excluding `.gitkeep`. This task creates the corpus structure and minimum fixture set. Basis: PRD §Compatibility Harness Design / §Success Metrics, ADR-006, ADR-007.

## 2. Goal

Add at least 50 tracked P0 compatibility fixtures with metadata, expected domains, mock provider assets, and matrix linkage.

## 3. Scope

### In Scope

- compatibility/fixtures/
- compatibility/fixtures/schema.json
- compatibility/fixtures/**/fixture.yaml
- tests/p0_fixture_corpus.rs
- docs/compatibility/fixtures.md

### Out Of Scope

- Does not yet execute upstream or rs binaries; task 12.2 owns execution.
- Does not use real provider API keys.

## 4. Users / Actors

- Maintainer: curates P0 corpus.
- AI infra / platform team: uses corpus to decide migration readiness.
- Release manager: requires corpus count before stable release.

## 5. Behavior Contract

Every P0 fixture must include id, TEST-ID, matrix item ids, priority, provider mocking mode, required env, expected outputs, normalization rules, and whether it blocks stable release.

### 5.1 Required Reading

- docs/prds/promptfoo-rs.prd.md
- docs/specs/phases/phase-12-compatibility-fixtures-golden-diff.md
- docs/compatibility/matrix.md
- docs/decisions/adr-006-compatibility-first-test-toolchain.md
- test/features/perfect-refactor-parity.feature

### 5.2 Imports

- Rust crate / module：`serde`、`serde_yaml`、`serde_json`、内部模块 `compatibility::fixtures`。
- Fixture paths：`compatibility/fixtures/**/fixture.yaml`。

### 5.3 函数签名

- `load_fixture_manifest(path: &Path) -> Result<FixtureManifest, FixtureError>`
- `validate_p0_fixture_corpus(root: &Path, matrix: &CapabilityMatrix) -> FixtureCorpusReport`
- `fixture_count_by_priority(report: &FixtureCorpusReport, priority: Priority) -> usize`

## 6. Acceptance Criteria

- [x] **AC1** (PRD §Success Metrics): repository tracks at least 50 P0 fixtures excluding `.gitkeep`.
- [x] **AC2** (PRD §Compatibility Harness Design): every fixture validates metadata schema and links to matrix item ids.
- [x] **AC3** (ADR-006): fixtures use mock/recorded providers and do not require real secrets.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-12.1.1 | TEST-12.1.1 | tests/p0_fixture_corpus.rs | install, typecheck, unit-test, manual | Done |
| AC2 | SCEN-12.1.1 | TEST-12.1.2 | tests/p0_fixture_corpus.rs | install, typecheck, unit-test, manual | Done |
| AC3 | SCEN-12.1.1 | TEST-12.1.3 | tests/p0_fixture_corpus.rs | install, typecheck, unit-test, manual | Done |

## 8. Risks

- 50 fixtures can still miss high-value behavior; matrix linkage must show domain coverage.
- Fixture contents may drift from upstream examples; source references are required.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Manual**: review fixture coverage by CLI/config/provider/assertion/output/redteam domain.

## 10. Completion Notes

- **完成日期**：2026-05-30
- **改动文件**：
  - `src/compatibility/fixtures.rs`
  - `tests/p0_fixture_corpus.rs`
  - `compatibility/fixtures/schema.json`
  - `compatibility/fixtures/**/fixture.yaml`
  - `docs/compatibility/fixtures.md`
  - `docs/specs/tasks/task-12.1-p0-fixture-corpus.md`
- **commit 列表**：
  - `b9fedd7` `docs(spec): task-12.1 进入实施 (Status: Ready → In Progress)`
  - `235b9c3` `test(compatibility-fixtures): 加 SCEN-12.1.1 的 3 个 RED 测试`
  - `86dd4da` `feat(compatibility-fixtures): 建立 P0 fixture corpus 基础校验`
- **§9 Verification 结果**：
  - install: PASS — `s2v_verify_full "install typecheck unit-test"` 中 `cargo fetch` 通过。
  - typecheck: PASS — `cargo check --workspace` 通过。
  - unit-test: PASS — `cargo test --workspace` 通过，包含 `tests/p0_fixture_corpus.rs` 的 3 个 TEST-12.1.* 用例。
  - manual: PASS — fixture 实物审计通过：共 52 个 P0 fixture；域分布为 assertions=9, cli=11, config=4, eval=6, node-api=1, outputs=6, providers=8, redteam=4, release=2, viewer=1；所有 fixture 使用 Mock/Recorded provider，未要求真实 secret。非交互 helper 的 full run 仅因 `/dev/tty` manual 确认失败，机械 keys 已单独全绿。
- **剩余风险 / 未做项**：fixture corpus 目前只建立可追踪元数据和 mock/recorded 资产约束，尚未执行 upstream/rs 双跑；执行器由 task 12.2 负责，release gate 汇总由 task 12.3 负责。
- **下游 task 影响**：task 12.2 可直接消费 `compatibility/fixtures/**/fixture.yaml` 与 schema；task 12.3 可将 52 个 P0 fixture 作为稳定发布 gate 的输入基线。
